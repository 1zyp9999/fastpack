# FastPack Quick Start Guide

## 🚀 Get Started in 5 Minutes

### 1. Install Prerequisites

#### Windows
```powershell
# Install Node.js
# Download from: https://nodejs.org/

# Install Rust
# Download from: https://www.rust-lang.org/tools/install

# Install CMake (optional, for building C++ projects)
# Download from: https://cmake.org/download/
```

#### Linux
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y nodejs npm cargo cmake zstd

# Fedora/RHEL
sudo dnf install -y nodejs npm cargo cmake zstd

# Arch Linux
sudo pacman -S nodejs npm cargo cmake zstd
```

### 2. Build FastPack

#### Windows
```powershell
# Clone the repository
git clone https://github.com/yourusername/fastpack.git
cd fastpack

# Build the application
.\build.bat build
```

#### Linux
```bash
# Clone the repository
git clone https://github.com/yourusername/fastpack.git
cd fastpack

# Build the application
chmod +x build.sh
./build.sh build
```

### 3. Create Your First Package

#### Option A: Using the GUI

##### Windows
```powershell
# Launch FastPack
.\src-tauri\target\release\fastpack.exe

# Or install system-wide
copy .\src-tauri\target\release\fastpack.exe C:\Windows\System32\
fastpack
```

##### Linux
```bash
# Launch FastPack
./src-tauri/target/release/fastpack

# Or install system-wide
sudo cp ./src-tauri/target/release/fastpack /usr/local/bin/
fastpack
```

#### Option B: Using Command Line

##### Windows
```powershell
# Create a simple package
.\src-tauri\target\release\fastpack.exe pack `
  --source .\examples\cpp_project `
  --output myapp.exe `
  --name "My App" `
  --version "1.0.0"
```

##### Linux
```bash
# Create a simple package
./src-tauri/target/release/fastpack pack \
  --source ./examples/cpp_project \
  --output myapp.run \
  --name "My App" \
  --version "1.0.0"
```

### 4. Test Your Package

#### Windows
```powershell
# Run the installer
.\myapp.exe

# Your app is now installed!
# Check C:\Program Files\MyApp
```

#### Linux
```bash
# Make the installer executable
chmod +x myapp.run

# Run the installer
sudo ./myapp.run

# Your app is now installed!
# Check /opt/myapp
```

## 📦 Example: Package a C++ Project

### Windows
```powershell
cd examples\cpp_project

# Build the project first
.\build.bat

# Package it with FastPack
fastpack

# In the GUI:
# 1. Set Source Directory to current directory
# 2. Set Output Path to myapp.exe
# 3. Click "Build Project"
# 4. Click "Create Package"
```

### Linux
```bash
cd examples/cpp_project

# Build the project first
./build.sh

# Package it with FastPack
fastpack

# In the GUI:
# 1. Set Source Directory to current directory
# 2. Set Output Path to myapp.run
# 3. Click "Build Project"
# 4. Click "Create Package"
```

## ⚡ Performance Tips

1. **Use zstd compression**: It's 3-5x faster than gzip
2. **Enable multi-threading**: Set threads to match your CPU cores
3. **Use incremental packaging**: Only changed files are packed
4. **Choose appropriate compression level**:
   - Level 1: Fastest, for development builds
   - Level 3: Balanced, recommended for most cases
   - Level 19: Maximum, for final releases

## 🔧 Configuration File

Create a `fastpack.json` in your project:

### Windows Example
```json
{
  "source_dir": "./",
  "output_path": "./output.exe",
  "package_name": "myapp",
  "version": "1.0.0",
  "install_dir": "C:\\Program Files\\MyApp",
  "compression_level": 3,
  "exclude_patterns": [
    "*.git",
    "*.obj",
    "*.pdb",
    "build/"
  ]
}
```

### Linux Example
```json
{
  "source_dir": "./",
  "output_path": "./output.run",
  "package_name": "myapp",
  "version": "1.0.0",
  "install_dir": "/opt/myapp",
  "compression_level": 3,
  "exclude_patterns": [
    "*.git",
    "*.o",
    "*.so",
    "build/"
  ]
}
```

Then run:

#### Windows
```powershell
fastpack --config fastpack.json
```

#### Linux
```bash
fastpack --config fastpack.json
```

## 🎯 Common Use Cases

### Package a Node.js Application

#### Windows
```powershell
fastpack pack `
  --source .\my-node-app `
  --output myapp.exe `
  --name "My Node App" `
  --install-dir "C:\Program Files\MyApp" `
  --project-type node
```

#### Linux
```bash
fastpack pack \
  --source ./my-node-app \
  --output myapp.run \
  --name "My Node App" \
  --install-dir "/opt/myapp" \
  --project-type node
```

### Package a Rust Application

#### Windows
```powershell
fastpack pack `
  --source .\my-rust-app `
  --output myapp.exe `
  --name "My Rust App" `
  --install-dir "C:\Program Files\MyApp" `
  --project-type cargo
```

#### Linux
```bash
fastpack pack \
  --source ./my-rust-app \
  --output myapp.run \
  --name "My Rust App" \
  --install-dir "/opt/myapp" \
  --project-type cargo
```

### Package with Custom Scripts

#### Windows
```json
{
  "source_dir": "./",
  "output_path": "./output.exe",
  "package_name": "myapp",
  "version": "1.0.0",
  "pre_install_script": "scripts\\pre-install.ps1",
  "post_install_script": "scripts\\post-install.ps1"
}
```

#### Linux
```json
{
  "source_dir": "./",
  "output_path": "./output.run",
  "package_name": "myapp",
  "version": "1.0.0",
  "pre_install_script": "scripts/pre-install.sh",
  "post_install_script": "scripts/post-install.sh"
}
```

## 📊 Benchmark Your Project

### Windows
```powershell
.\benchmark.bat
```

### Linux
```bash
chmod +x benchmark.sh
./benchmark.sh
```

This will show you how FastPack compares to traditional tools for your specific project.

## 🆘 Troubleshooting

### Build Fails

#### Windows
```powershell
# Check Rust installation
cargo --version

# Check Node.js installation
node --version
npm --version

# Clean and rebuild
.\build.bat clean
.\build.bat build
```

#### Linux
```bash
# Check Rust installation
cargo --version

# Check Node.js installation
node --version
npm --version

# Clean and rebuild
./build.sh clean
./build.sh build
```

### Package Creation Fails

#### Windows
```powershell
# Check source directory exists
dir .\your-project

# Check write permissions
dir .\output-directory

# Try with verbose output
fastpack pack --verbose --source .\your-project
```

#### Linux
```bash
# Check source directory exists
ls -la ./your-project

# Check write permissions
ls -la ./output-directory

# Try with verbose output
fastpack pack --verbose --source ./your-project
```

### Installation Fails

#### Windows
```powershell
# Check if installer is executable
dir myapp.exe

# Run with PowerShell for debugging
powershell -ExecutionPolicy Bypass -File myapp.exe

# Check administrator privileges
# Right-click and select "Run as administrator"
```

#### Linux
```bash
# Check if installer is executable
ls -l myapp.run

# Make executable if needed
chmod +x myapp.run

# Run with bash for debugging
bash -x myapp.run

# Check if running with sudo
sudo ./myapp.run
```

## 📚 Next Steps

- Read the full [README.md](README.md) for detailed documentation
- Check out the [examples/](examples/) directory for more samples
- Join our Discord community for support
- Contribute to the project on GitHub

## 🌍 Cross-Platform Tips

FastPack works seamlessly on both Windows and Linux:

1. **Same Configuration**: Use `fastpack.json` for both platforms
2. **Automatic Detection**: Project type detection works on both platforms
3. **Native Installers**: 
   - Windows: `.exe` installer
   - Linux: `.run` installer
4. **Path Handling**: FastPack automatically handles path separators
5. **Script Support**: 
   - Windows: PowerShell scripts (`.ps1`)
   - Linux: Bash scripts (`.sh`)

---

**Happy Packaging! 🚀**