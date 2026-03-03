mod types;
mod packager;
mod compiler;

use packager::Packager;
use compiler::{Compiler, BuildConfig};
use types::{PackConfig, PackResult};
use std::path::PathBuf;
use std::env;

#[tauri::command]
fn shell_open(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| format!("无法打开文件夹：{}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("无法打开文件夹：{}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        // 在 Linux 上，使用 xdg-open 打开文件夹
        // 如果失败，尝试常见的文件管理器
        let result = std::process::Command::new("xdg-open")
            .arg(&path)
            .output();
        
        match result {
            Ok(_) => {},
            Err(e) => {
                // xdg-open 失败，尝试 nautilus
                let _ = std::process::Command::new("nautilus")
                    .arg(&path)
                    .spawn();
                
                // 如果都不行，返回错误
                return Err(format!("无法打开文件夹：{}\n请确保已安装 xdg-open 或 nautilus", e));
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn pack(config: PackConfig) -> Result<PackResult, String> {
    let source_path = PathBuf::from(&config.source_dir);
    
    // 1. 检测项目类型
    let project_type = Compiler::detect_from_dir(&source_path)
        .map_err(|e| format!("检测项目类型失败：{}", e))?;
    
    println!("[INFO] 检测到项目类型：{:?}", project_type);
    
    // 2. 编译项目
    println!("[INFO] 正在编译项目...");
    let build_dir = source_path.join("fastpack_build");
    let compiler = Compiler::new(project_type);
    
    // 构建配置
    let build_config = compiler::BuildConfig {
        install_root: config.install_root.clone(),
        qmake_args: config.qmake_args.clone(),
        make_args: config.make_args.clone(),
        use_qt_ifw_style: config.use_qt_ifw_style,
        bindist_target: config.bindist_script.clone(),
    };
    
    let build_result = compiler
        .build_with_config(&source_path, &build_dir, &build_config)
        .map_err(|e| format!("编译失败：{}", e))?;
    
    println!("[SUCCESS] 编译完成！耗时：{}ms", build_result.duration_ms);
    
    // 3. 确定打包目录
    let package_dir = if config.use_qt_ifw_style {
        // Qt IFW 风格：从 install 目录打包
        build_dir.join("install")
    } else {
        // 普通风格：从 build 目录打包
        build_dir.join("build")
    };
    
    if !package_dir.exists() {
        return Err(format!("打包目录不存在：{}", package_dir.display()));
    }
    
    println!("[INFO] 从 {} 打包...", package_dir.display());
    
    // 4. 打包
    let mut pack_config = config.clone();
    pack_config.source_dir = package_dir;
    
    let packager = Packager::new(pack_config);
    packager.pack(None).map_err(|e| e.to_string())
}

#[tauri::command]
fn detect_project_type(path: String) -> Result<String, String> {
    let project_type = Compiler::detect_from_dir(PathBuf::from(&path).as_path())
        .map_err(|e| e.to_string())?;
    Ok(format!("{:?}", project_type))
}

#[tauri::command]
fn get_project_version(path: String) -> Result<String, String> {
    let version = Compiler::read_version_from_dir(PathBuf::from(&path).as_path())
        .unwrap_or_else(|| "1.0.0".to_string());
    Ok(version)
}

#[tauri::command]
fn build_project(source_dir: String, output_dir: String) -> Result<String, String> {
    let project_type = Compiler::detect_from_dir(PathBuf::from(&source_dir).as_path())
        .map_err(|e| e.to_string())?;

    let compiler = Compiler::new(project_type);
    let result = compiler
        .build(PathBuf::from(&source_dir).as_path(), PathBuf::from(&output_dir).as_path())
        .map_err(|e| e.to_string())?;

    Ok(format!("Build completed in {}ms", result.duration_ms))
}

// CLI 命令行入口 - 超快打包
fn run_cli() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "pack" => {
            let mut source_dir = ".";
            let mut output_path = "output.run";
            let mut package_name = "myapp";
            let mut version = "1.0.0";
            let mut install_dir = "/opt/myapp";
            let mut compression_level = 3;
            
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--source" | "-s" => { i += 1; source_dir = &args[i]; }
                    "--output" | "-o" => { i += 1; output_path = &args[i]; }
                    "--name" | "-n" => { i += 1; package_name = &args[i]; }
                    "--version" | "-v" => { i += 1; version = &args[i]; }
                    "--install-dir" | "-i" => { i += 1; install_dir = &args[i]; }
                    "--compression-level" | "-c" => { i += 1; compression_level = args[i].parse().unwrap_or(3); }
                    _ => {}
                }
                i += 1;
            }

            // 1. 检测项目类型
            let source_path = PathBuf::from(source_dir);
            let project_type = Compiler::detect_from_dir(&source_path)
                .map_err(|e| format!("检测项目类型失败：{}", e))?;
            
            println!("[INFO] 检测到项目类型：{:?}", project_type);
            
            // 2. 编译项目
            println!("[INFO] 正在编译项目...");
            let build_dir = source_path.join("build");
            let compiler = Compiler::new(project_type);
            let build_result = compiler
                .build(&source_path, &build_dir)
                .map_err(|e| format!("编译失败：{}", e))?;
            
            println!("[SUCCESS] 编译完成！耗时：{}ms", build_result.duration_ms);
            
            // 3. 打包
            println!("[INFO] 正在打包...");
            let config = PackConfig {
                source_dir: build_dir.clone(),
                output_path: PathBuf::from(output_path),
                package_name: package_name.to_string(),
                version: version.to_string(),
                install_dir: install_dir.to_string(),
                compression_level,
                threads: None,
                exclude_patterns: vec![],
                use_qt_ifw_style: false,
                install_root: None,
                qmake_args: vec![],
                make_args: vec![],
                bindist_script: None,
            };
            
            let packager = Packager::new(config);
            let result = packager.pack(None)
                .map_err(|e| format!("打包失败：{}", e))?;
            
            println!("[SUCCESS] 打包完成!");
            println!("  输出文件：{}", result.output_path.display());
            println!("  原始大小：{} bytes", result.original_size);
            println!("  压缩后：{} bytes", result.compressed_size);
            println!("  压缩率：{:.1}%", result.compression_ratio);
            println!("  总耗时：{}ms", result.duration_ms + build_result.duration_ms);
            
            Ok(())
        }
        "detect" => {
            if args.len() < 3 {
                return Err("用法：fastpack detect <路径>".to_string());
            }
            let path = PathBuf::from(&args[2]);
            let project_type = Compiler::detect_from_dir(&path)
                .map_err(|e| format!("检测失败：{}", e))?;
            println!("{:?}", project_type);
            Ok(())
        }
        "--help" | "-h" => {
            print_help();
            Ok(())
        }
        _ => {
            eprintln!("未知命令：{}", args[1]);
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!("⚡ FastPack - 超快跨平台打包工具");
    println!();
    println!("用法:");
    println!("  fastpack [命令] [选项]");
    println!();
    println!("命令:");
    println!("  pack      打包项目 (自动检测 → 编译 → 打包)");
    println!("  detect    检测项目类型");
    println!("  --help    显示帮助");
    println!();
    println!("pack 选项:");
    println!("  -s, --source <路径>       源目录 (默认：.)");
    println!("  -o, --output <文件>       输出文件 (默认：output.run)");
    println!("  -n, --name <名称>         包名称");
    println!("  -v, --version <版本>      版本号");
    println!("  -i, --install-dir <路径>  安装路径");
    println!("  -c, --compression-level   压缩级别 1-19 (默认：3)");
    println!();
    println!("示例:");
    println!("  fastpack pack -s ./myapp -o myapp.run -n myapp -v 1.0.0");
    println!("  fastpack detect ./myapp");
}

fn main() {
    // 检查是否是 CLI 模式运行
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1].starts_with('-') || args[1] == "pack" || args[1] == "detect") {
        match run_cli() {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                eprintln!("错误：{}", e);
                std::process::exit(1);
            }
        }
    }

    // GUI 模式
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![pack, detect_project_type, get_project_version, build_project, shell_open])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
