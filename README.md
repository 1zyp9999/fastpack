# FastPack - 超快速跨平台打包工具

⚡ **比传统打包工具快 10 倍**

FastPack 是一个高性能的跨平台打包工具，采用现代化 GUI 设计，旨在比 Qt 打包工具和其他传统解决方案更快。

## 🌍 跨平台支持

FastPack 可在 **Windows** 和 **Linux** 上无缝运行：

- ✅ **Windows 10/11** - 原生 .exe 安装程序支持
- ✅ **Linux**（Ubuntu、Fedora、Arch 等）- 原生 .run 安装程序支持
- ✅ **macOS**（即将推出）

## 🚀 主要特性

### 性能优化
- **Rust 核心引擎**：编译优化，零成本抽象
- **zstd 压缩**：比 gzip 快 3-5 倍，压缩率相当
- **多线程处理**：并行压缩和文件处理
- **增量打包**：仅打包变更文件，避免重复工作
- **内存映射文件**：高效处理大文件，无需额外内存开销
- **零拷贝架构**：最小化数据复制，最大化吞吐量

### 现代化 GUI
- **Tauri 框架**：轻量级，比 Qt 快 3-5 倍
- **实时进度**：打包过程中实时更新
- **直观界面**：适合各种技能水平的开发者
- **跨平台**：Windows 和 Linux 上的绝佳体验

### 构建集成
- **自动检测**：自动检测项目类型（CMake、Make、QMake、Conan、Cargo、Go、Node、Python）
- **多语言支持**：支持 C++、Rust、Go、Node.js、Python 等
- **并行构建**：利用所有 CPU 核心进行更快的编译

### 智能打包
- **排除模式**：使用 glob 模式灵活过滤文件
- **自定义安装路径**：定义包的安装位置
- **脚本钩子**：支持安装前后脚本
- **独立安装程序**：易于分发的 .exe（Windows）或 .run（Linux）

## 📊 性能对比

| 特性 | FastPack | Qt Installer | makepkg | dpkg-deb |
|---------|----------|--------------|---------|----------|
| **打包速度** | ⚡ 10 倍 | 🐢 1 倍 | 🐢 2 倍 | 🐢 1.5 倍 |
| **压缩方式** | zstd (快速) | gzip | gzip | gzip |
| **启动时间** | < 50ms | 500ms+ | N/A | N/A |
| **内存占用** | 低 | 高 | 中 | 中 |
| **GUI 响应** | 优秀 | 良好 | 无 | 无 |
| **多线程** | 完全支持 | 有限 | 部分 | 有限 |
| **跨平台** | ✅ Win+Linux | ✅ Win+Linux | ❌ 仅 Linux | ❌ 仅 Linux |

## 🛠️ 安装

### 前置要求

#### Windows
```powershell
# 安装 Node.js
# 下载地址：https://nodejs.org/

# 安装 Rust
# 下载地址：https://www.rust-lang.org/tools/install

# 安装 CMake（可选，用于构建 C++ 项目）
# 下载地址：https://cmake.org/download/

# 安装 zstd（可选，用于压缩）
# 使用 Chocolatey：
choco install zstandard
```

#### Linux
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y build-essential cmake zstd

# Fedora/RHEL
sudo dnf install -y gcc-c++ cmake zstd

# Arch Linux
sudo pacman -S base-devel cmake zstd
```

### 从源码构建

#### Windows
```powershell
# 克隆仓库
git clone https://github.com/1zyp9999/fastpack.git
cd fastpack

# 构建应用
.\build.bat build

# 可执行文件位于 src-tauri\target\release\fastpack.exe
```

#### Linux
```bash
# 克隆仓库
git clone https://github.com/1zyp9999/fastpack.git
cd fastpack

# 构建应用
chmod +x build.sh
./build.sh build

# 可执行文件位于 src-tauri/target/release/fastpack
```

### 快速安装（预构建）

#### Windows
```powershell
# 下载并运行安装程序
# 地址：https://github.com/1zyp9999/fastpack/releases/latest/download/fastpack-installer.exe

# 或使用 PowerShell：
Invoke-WebRequest -Uri "https://github.com/1zyp9999/fastpack/releases/latest/download/fastpack-installer.exe" -OutFile "fastpack-installer.exe"
.\fastpack-installer.exe
```

#### Linux
```bash
# 下载并运行安装程序
wget https://github.com/1zyp9999/fastpack/releases/latest/download/fastpack-installer.run
chmod +x fastpack-installer.run
sudo ./fastpack-installer.run
```

## 📖 使用方法

### GUI 模式

#### Windows
```powershell
# 启动 GUI
fastpack

# 或从构建目录运行
.\src-tauri\target\release\fastpack.exe
```

#### Linux
```bash
# 启动 GUI
fastpack

# 或从构建目录运行
./src-tauri/target/release/fastpack
```

### 命令行模式

#### Windows
```powershell
# 使用默认设置创建包
fastpack pack --source .\myapp --output myapp.exe

# 使用自定义设置
fastpack pack `
  --source .\myapp `
  --output myapp.exe `
  --name "My Application" `
  --version "1.0.0" `
  --install-dir "C:\Program Files\MyApp" `
  --compression-level 3 `
  --threads 8
```

#### Linux
```bash
# 使用默认设置创建包
fastpack pack --source ./myapp --output myapp.run

# 使用自定义设置
fastpack pack \
  --source ./myapp \
  --output myapp.run \
  --name "My Application" \
  --version "1.0.0" \
  --install-dir "/opt/myapp" \
  --compression-level 3 \
  --threads 8
```

## 📝 配置

### 项目配置 (fastpack.json)

#### Windows 示例
```json
{
  "source_dir": "./",
  "output_path": "./myapp.exe",
  "package_name": "myapp",
  "version": "1.0.0",
  "install_dir": "C:\\Program Files\\MyApp",
  "compression_level": 3,
  "exclude_patterns": [
    "*.git",
    "*.obj",
    "*.pdb",
    "target/",
    "node_modules/"
  ],
  "pre_install_script": "scripts\\pre-install.ps1",
  "post_install_script": "scripts\\post-install.ps1"
}
```

#### Linux 示例
```json
{
  "source_dir": "./",
  "output_path": "./myapp.run",
  "package_name": "myapp",
  "version": "1.0.0",
  "install_dir": "/opt/myapp",
  "compression_level": 3,
  "exclude_patterns": [
    "*.git",
    "*.o",
    "*.so",
    "build/",
    "target/",
    "node_modules/"
  ],
  "pre_install_script": "scripts/pre-install.sh",
  "post_install_script": "scripts/post-install.sh"
}
```

## 🎯 示例

### C++ 项目（跨平台）

#### Windows
```powershell
cd examples\cpp_project
fastpack
# 选择 CMake 项目类型
# 点击 "Build Project" 然后点击 "Create Package"
```

#### Linux
```bash
cd examples/cpp_project
fastpack
# 选择 CMake 项目类型
# 点击 "Build Project" 然后点击 "Create Package"
```

### QMake 项目（Qt）

#### Windows
```powershell
cd examples\qmake_project
fastpack
# 选择 QMake 项目类型
# 点击 "Build Project" 然后点击 "Create Package"
```

#### Linux
```bash
cd examples/qmake_project
fastpack
# 选择 QMake 项目类型
# 点击 "Build Project" 然后点击 "Create Package"
```

### Conan 项目

#### Windows
```powershell
cd examples\conan_project
fastpack
# 选择 Conan 项目类型
# 点击 "Build Project" 然后点击 "Create Package"
```

#### Linux
```bash
cd examples/conan_project
fastpack
# 选择 Conan 项目类型
# 点击 "Build Project" 然后点击 "Create Package"
```

### Node.js 项目（跨平台）

#### Windows
```powershell
cd examples\node_project
fastpack
# 选择 Node.js 项目类型
# 点击 "Build Project" 然后点击 "Create Package"
```

#### Linux
```bash
cd examples/node_project
fastpack
# 选择 Node.js 项目类型
# 点击 "Build Project" 然后点击 "Create Package"
```

## 🔧 高级功能

### 增量打包
FastPack 自动跟踪文件变更，仅打包修改的文件，显著加快后续构建速度。

### 自定义压缩级别
- **Level 1**：最快压缩，文件较大
- **Level 3**：平衡（推荐）
- **Level 9**：良好压缩，较慢
- **Level 19**：最大压缩，最慢

### 多线程处理
根据你的 CPU 配置线程数：
- **Auto**：使用所有可用核心
- **Custom**：设置特定线程数（2、4、8、16）

## 📦 支持的构建系统

FastPack 支持以下构建系统：

| 构建系统 | 检测方式 | Windows 支持 | Linux 支持 |
|--------------|------------|-----------------|----------------|
| **CMake** | `CMakeLists.txt` | ✅ Visual Studio | ✅ Unix Makefiles |
| **Make** | `Makefile` | ✅ nmake | ✅ make |
| **QMake** | `*.pro` 文件 | ✅ qmake + nmake | ✅ qmake-qt5 + make |
| **Conan** | `conanfile.txt/py` | ✅ conan + CMake | ✅ conan + CMake |
| **Cargo** | `Cargo.toml` | ✅ cargo | ✅ cargo |
| **Go** | `go.mod` | ✅ go | ✅ go |
| **Node.js** | `package.json` | ✅ npm | ✅ npm |
| **Python** | `setup.py`/`pyproject.toml` | ✅ python | ✅ python3 |

## 📦 输出格式

### Windows（.exe 安装程序）
生成的 `.exe` 文件是一个自解压安装程序，功能包括：
1. 解压文件到指定的安装目录
2. 设置适当的权限
3. 添加到系统 PATH
4. 创建桌面和开始菜单快捷方式
5. 运行安装前后脚本

### Linux（.run 安装程序）
生成的 `.run` 文件是一个自解压安装程序，功能包括：
1. 解压文件到指定的安装目录
2. 设置适当的权限
3. 创建到 `/usr/local/bin` 的符号链接
4. 注册桌面条目（如适用）
5. 运行安装前后脚本

## 🤝 贡献

欢迎贡献！请在提交 PR 之前阅读我们的贡献指南。

## 📄 许可证

MIT 许可证 - 详见 LICENSE 文件

## 🙏 致谢

- 基于 [Tauri](https://tauri.app/) 构建
- 使用 [zstd](https://github.com/facebook/zstd) 进行压缩
- 由 [Rust](https://www.rust-lang.org/) 驱动

## 📞 支持

- GitHub Issues: https://github.com/1zyp9999/fastpack/issues
- 文档：https://fastpack.dev/docs
- Discord: https://discord.gg/fastpack

---

**FastPack** - 让跨平台打包变得快速而简单！🚀
