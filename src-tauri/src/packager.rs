use crate::types::{PackConfig, PackProgress, PackResult};
use anyhow::{Context, Result};
use crossbeam_channel::Sender;
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use walkdir::WalkDir;

pub struct Packager {
    config: PackConfig,
}

impl Packager {
    pub fn new(config: PackConfig) -> Self {
        Self { config }
    }

    pub fn pack(&self, progress_sender: Option<Sender<PackProgress>>) -> Result<PackResult> {
        let start = Instant::now();

        // 1. 收集文件列表
        let files = self.collect_files()?;
        let total_files = files.len();
        let total_bytes: u64 = files.iter().map(|f| f.size).sum();

        let progress = Arc::new(PackProgressState::new(total_files, total_bytes));

        // 2. 并行压缩所有文件
        let compressed_files = self.compress_files_parallel(&files, progress.clone(), progress_sender)?;

        // 3. 生成自解压脚本
        let output_path = self.create_self_extracting_archive(&compressed_files)?;

        let duration = start.elapsed();

        Ok(PackResult {
            success: true,
            output_path,
            original_size: total_bytes,
            compressed_size: compressed_files.iter().map(|f| f.compressed_size).sum(),
            compression_ratio: (compressed_files.iter().map(|f| f.compressed_size).sum::<u64>() as f64 / total_bytes as f64) * 100.0,
            files_count: total_files,
            duration_ms: duration.as_millis(),
        })
    }

    fn collect_files(&self) -> Result<Vec<FileInfo>> {
        let mut files = Vec::new();
        let exclude_patterns: Vec<_> = self
            .config
            .exclude_patterns
            .iter()
            .map(|p| glob::Pattern::new(p))
            .collect::<Result<_, _>>()?;

        for entry in WalkDir::new(&self.config.source_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                let relative_path = path
                    .strip_prefix(&self.config.source_dir)
                    .context("Failed to get relative path")?;

                let path_str = relative_path.to_string_lossy();
                if !exclude_patterns.iter().any(|p| p.matches(&path_str)) {
                    let metadata = fs::metadata(path)?;
                    files.push(FileInfo {
                        path: relative_path.to_path_buf(),
                        size: metadata.len(),
                    });
                }
            }
        }

        files.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(files)
    }

    fn compress_files_parallel(
        &self,
        files: &[FileInfo],
        progress: Arc<PackProgressState>,
        progress_sender: Option<Sender<PackProgress>>,
    ) -> Result<Vec<CompressedFile>> {
        let threads = self.config.threads.unwrap_or_else(|| num_cpus::get());
        let compression_level = self.config.compression_level as i32;

        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .context("Failed to build thread pool")?;

        let compressed_files: Result<Vec<CompressedFile>> = files
            .par_iter()
            .map(|file_info| {
                let file_path = self.config.source_dir.join(&file_info.path);
                
                // 读取并压缩文件
                let mut file = File::open(&file_path)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                
                // 使用 zstd 压缩
                let compressed = zstd::encode_all(&buffer[..], compression_level)?;
                
                progress.update(1, file_info.size);

                if let Some(ref sender) = progress_sender {
                    let _ = sender.send(PackProgress {
                        current_file: file_info.path.to_string_lossy().to_string(),
                        files_processed: progress.files_processed.load(Ordering::Relaxed),
                        total_files: progress.total_files,
                        bytes_processed: progress.bytes_processed.load(Ordering::Relaxed),
                        total_bytes: progress.total_bytes,
                        compression_ratio: 0.0,
                    });
                }

                Ok(CompressedFile {
                    path: file_info.path.clone(),
                    original_size: file_info.size,
                    compressed_size: compressed.len() as u64,
                    data: compressed,
                })
            })
            .collect();

        compressed_files
    }

    fn create_self_extracting_archive(&self, files: &[CompressedFile]) -> Result<PathBuf> {
        let output_path = &self.config.output_path;
        let mut file = BufWriter::new(File::create(output_path)?);

        let install_dir = &self.config.install_dir;
        let package_name = &self.config.package_name;
        let version = &self.config.version;

        // 生成高效的 bash 安装脚本
        let script = self.generate_install_script(package_name, version, install_dir);
        file.write_all(script.as_bytes())?;

        // 写入二进制数据：每个文件 = 4 字节路径长度 + 路径 + 8 字节数据长度 + 压缩数据
        for cf in files {
            let path_str = cf.path.to_string_lossy();
            let path_bytes = path_str.as_bytes();
            let path_len = path_bytes.len() as u32;
            let data_len = cf.compressed_size as u64;

            file.write_all(&path_len.to_le_bytes())?;
            file.write_all(path_bytes)?;
            file.write_all(&data_len.to_le_bytes())?;
            file.write_all(&cf.data)?;
        }

        file.flush()?;
        drop(file);

        // 设置执行权限
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(output_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(output_path, perms)?;

        Ok(output_path.clone())
    }

    fn generate_install_script(&self, package_name: &str, version: &str, install_dir: &str) -> String {
        let auto_install_dir = format!("/opt/{}", package_name);
        
        // 使用 base64 编码的 Python 解压脚本，避免引号冲突
        let python_script = base64::encode(format!(r#"
import sys, struct, zstd, os
script_path = sys.argv[1]
install_dir = sys.argv[2]
with open(script_path, 'rb') as f:
    content = f.read()
    marker = b'# __BINARY_DATA__\n'
    marker_pos = content.rfind(marker)
    if marker_pos == -1: sys.exit(1)
    pos = marker_pos + len(marker)
    while pos < len(content):
        if pos + 4 > len(content): break
        path_len = struct.unpack('<I', content[pos:pos+4])[0]
        pos += 4
        if path_len == 0 or pos + path_len > len(content): break
        file_path = content[pos:pos+path_len].decode('utf-8')
        pos += path_len
        if pos + 8 > len(content): break
        data_len = struct.unpack('<Q', content[pos:pos+8])[0]
        pos += 8
        if data_len == 0 or pos + data_len > len(content): break
        compressed_data = content[pos:pos+data_len]
        pos += data_len
        try:
            decompressed = zstd.decompress(compressed_data)
            target_path = os.path.join(install_dir, file_path)
            os.makedirs(os.path.dirname(target_path), exist_ok=True)
            with open(target_path, 'wb') as out:
                out.write(decompressed)
            os.chmod(target_path, 0o755)
        except: pass
"#));
        
        format!(
            r##"#!/bin/bash
# FastPack Installer - {} v{}
# Graphical self-extracting archive with system menu integration

DEFAULT_INSTALL_DIR="{}"
PACKAGE_NAME="{}"
VERSION="{}"
SCRIPT_PATH="$(readlink -f "$0")"
PYTHON_SCRIPT="{}"

# 检测是否有图形界面
has_gui() {{
    [ -n "$DISPLAY" ] && command -v zenity &> /dev/null
}}

# 创建桌面菜单项
create_desktop_file() {{
    local inst_dir="$1"
    local desktop_file="/usr/share/applications/${{PACKAGE_NAME}}.desktop"
    
    # 查找可执行文件
    local exec_file=$(find "$inst_dir" -type f -executable | head -1)
    
    if [ -n "$exec_file" ]; then
        cat > "$desktop_file" << EOF
[Desktop Entry]
Version=$VERSION
Name=$PACKAGE_NAME
Comment=$PACKAGE_NAME Application
Exec=$exec_file
Icon=application-x-executable
Terminal=false
Type=Application
Categories=Utility;Application;
EOF
    fi
}}

# 图形化安装
gui_install() {{
    local inst_dir=$(zenity --file-selection --directory --title="选择安装目录" --filename="$DEFAULT_INSTALL_DIR" 2>/dev/null)
    if [ -z "$inst_dir" ]; then
        exit 1
    fi
    
    (
    echo 10
    sudo mkdir -p "$inst_dir"
    echo 30
    echo "$PYTHON_SCRIPT" | base64 -d | python3 - "$SCRIPT_PATH" "$inst_dir"
    echo 70
    create_desktop_file "$inst_dir"
    echo 100
    ) | zenity --progress --title="安装中" --auto-close --auto-kill 2>/dev/null
    
    zenity --info --title="安装完成" --text="$PACKAGE_NAME v$VERSION 已成功安装到:\n$inst_dir\n\n可以在系统菜单中找到并启动" 2>/dev/null
}}

# 命令行安装
cli_install() {{
    local inst_dir="${{INSTALL_DIR:-$DEFAULT_INSTALL_DIR}}"
    echo "Installing $PACKAGE_NAME v$VERSION to $inst_dir"
    
    sudo mkdir -p "$inst_dir"
    echo "$PYTHON_SCRIPT" | base64 -d | sudo python3 - "$SCRIPT_PATH" "$inst_dir"
    
    create_desktop_file "$inst_dir"
    echo "Installation complete!"
    echo "Desktop menu item created: /usr/share/applications/$PACKAGE_NAME.desktop"
}}

# 主程序
if [ "$1" = "--extract-only" ]; then
    cli_install
    exit 0
fi

if has_gui; then
    gui_install
else
    cli_install
fi

exit 0

# __BINARY_DATA__
"##,
            package_name, version, auto_install_dir, package_name, version, python_script
        )
    }
}

#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    size: u64,
}

struct CompressedFile {
    path: PathBuf,
    original_size: u64,
    compressed_size: u64,
    data: Vec<u8>,
}

struct PackProgressState {
    total_files: usize,
    total_bytes: u64,
    files_processed: AtomicUsize,
    bytes_processed: AtomicU64,
}

impl PackProgressState {
    fn new(total_files: usize, total_bytes: u64) -> Self {
        Self {
            total_files,
            total_bytes,
            files_processed: AtomicUsize::new(0),
            bytes_processed: AtomicU64::new(0),
        }
    }

    fn update(&self, files: usize, bytes: u64) {
        self.files_processed.fetch_add(files, Ordering::Relaxed);
        self.bytes_processed.fetch_add(bytes, Ordering::Relaxed);
    }
}
