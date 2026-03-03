#!/bin/bash
# FastPack 完整流程测试：自动识别 → 编译 → 打包

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_step() { echo -e "${YELLOW}[STEP]${NC} $1"; }

SOURCE_DIR="${1:-./examples/cpp_project}"
OUTPUT_NAME="demo-package"

echo "=========================================="
echo "  FastPack 完整流程测试"
echo "=========================================="
echo ""

log_step "1. 检测项目类型..."
PROJECT_TYPE=""
if [ -f "$SOURCE_DIR/CMakeLists.txt" ]; then
    PROJECT_TYPE="CMake"
elif [ -f "$SOURCE_DIR/Makefile" ]; then
    PROJECT_TYPE="Make"
elif [ -f "$SOURCE_DIR/Cargo.toml" ]; then
    PROJECT_TYPE="Cargo"
elif [ -f "$SOURCE_DIR/package.json" ]; then
    PROJECT_TYPE="Node"
fi

if [ -z "$PROJECT_TYPE" ]; then
    echo "❌ 未识别的项目类型"
    exit 1
fi

log_success "检测到项目类型：$PROJECT_TYPE"
echo ""

log_step "2. 编译项目..."
BUILD_DIR="$SOURCE_DIR/build"
mkdir -p "$BUILD_DIR"

case "$PROJECT_TYPE" in
    "CMake")
        cd "$BUILD_DIR"
        cmake .. -DCMAKE_BUILD_TYPE=Release
        make -j$(nproc 2>/dev/null || echo 4)
        cd - > /dev/null
        ;;
    "Cargo")
        cd "$SOURCE_DIR"
        cargo build --release
        cd - > /dev/null
        ;;
    "Node")
        cd "$SOURCE_DIR"
        npm install
        npm run build 2>/dev/null || true
        cd - > /dev/null
        ;;
esac

log_success "编译完成!"
echo ""

log_step "3. 创建打包配置..."
cat > "$SOURCE_DIR/fastpack.json" << EOF
{
  "source_dir": "$BUILD_DIR",
  "output_path": "./${OUTPUT_NAME}.run",
  "package_name": "demo-app",
  "version": "1.0.0",
  "install_dir": "/opt/demo-app",
  "compression_level": 3,
  "threads": null,
  "exclude_patterns": [
    "*.o",
    "*.cmake",
    "CMakeFiles/",
    ".git"
  ]
}
EOF

log_success "配置已创建：$SOURCE_DIR/fastpack.json"
echo ""

log_step "4. 使用 FastPack 打包..."
./src-tauri/target/release/fastpack pack \
    --source "$BUILD_DIR" \
    --output "./${OUTPUT_NAME}.run" \
    --name "demo-app" \
    --version "1.0.0" \
    --install-dir "/opt/demo-app"

echo ""
log_success "打包完成！"
echo ""
echo "=========================================="
echo "  生成的文件:"
ls -lh "${OUTPUT_NAME}.run" 2>/dev/null || echo "  (打包输出位置请查看上方日志)"
echo "=========================================="
