# FastPack - 快速构建指南

## 📋 前置要求

### Windows
```powershell
# 安装 Node.js
# 下载：https://nodejs.org/

# 安装 Rust
# 下载：https://www.rust-lang.org/tools/install
```

### Linux
```bash
# Ubuntu/Debian
sudo apt install -y nodejs npm cargo cmake zstd

# Arch Linux
sudo pacman -S nodejs npm cargo cmake zstd
```

## 🔨 构建步骤

### Windows
```powershell
cd fastpack
.\build.bat build
```

### Linux
```bash
cd fastpack
chmod +x build.sh
./build.sh build
```

## 🚀 使用方法

### 启动 GUI
```powershell
# Windows
.\src-tauri\target\release\fastpack.exe

# Linux
./src-tauri/target/release/fastpack
```

### 打包流程（3步）

1. **拖拽项目文件夹** 到界面
2. **自动检测** 项目类型（CMake/QMake/Conan等）
3. **点击"开始打包"** 按钮

完成！生成的 `.exe`（Windows）或 `.run`（Linux）文件就是安装包。

## 📦 支持的项目类型

| 类型 | 检测文件 |
|------|----------|
| CMake | `CMakeLists.txt` |
| QMake | `*.pro` 文件 |
| Conan | `conanfile.txt` |
| Make | `Makefile` |
| Cargo | `Cargo.toml` |
| Go | `go.mod` |
| Node.js | `package.json` |
| Python | `setup.py` 或 `pyproject.toml` |

## ⚡ 性能特点

- Rust 核心引擎（10x 速度）
- zstd 压缩（3-5x 更快）
- 多线程并行处理
- 自动项目检测
- 智能配置生成

## 🆘 常见问题

### 构建失败
```powershell
# 检查依赖
node --version
cargo --version

# 清理重试
.\build.bat clean
.\build.bat build
```

### 打包失败
```powershell
# 检查源目录
dir your-project

# 检查权限
# Windows：右键"以管理员身份运行"
# Linux：sudo ./fastpack
```

## 📄 更多信息

- 完整文档：[README.md](README.md)
- 示例项目：[examples/](examples/)