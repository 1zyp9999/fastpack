#!/bin/bash
# Build script for FastPack

set -e

echo "=========================================="
echo "  FastPack Build Script"
echo "=========================================="
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    local missing=()

    if ! command -v node &> /dev/null; then
        missing+=("node")
    fi

    if ! command -v npm &> /dev/null; then
        missing+=("npm")
    fi

    if ! command -v cargo &> /dev/null; then
        missing+=("cargo")
    fi

    if [ ${#missing[@]} -ne 0 ]; then
        log_error "Missing prerequisites: ${missing[*]}"
        echo ""
        echo "Please install the following:"
        echo "  - Node.js: https://nodejs.org/"
        echo "  - Rust: https://www.rust-lang.org/tools/install"
        exit 1
    fi

    log_success "All prerequisites found"
}

# Install dependencies
install_dependencies() {
    log_info "Installing dependencies..."
    npm install
    log_success "Dependencies installed"
}

# Build the application
build() {
    log_info "Building FastPack..."
    npm run build
    log_success "Build complete!"
    echo ""
    log_info "Binary location: src-tauri/target/release/fastpack"
}

# Run development mode
dev() {
    log_info "Starting development mode..."
    npm run dev
}

# Clean build artifacts
clean() {
    log_info "Cleaning build artifacts..."
    rm -rf src-tauri/target
    rm -rf node_modules
    rm -f package-lock.json
    log_success "Clean complete!"
}

# Build release with installer
release() {
    build
    echo ""
    log_info "Creating installer..."
    log_info "Installer will be in src-tauri/target/release/bundle/"
    echo ""
    log_success "Release build complete!"
}

# Main
main() {
    check_prerequisites

    case "${1:-build}" in
        "build")
            install_dependencies
            build
            ;;
        "dev")
            install_dependencies
            dev
            ;;
        "clean")
            clean
            ;;
        "release")
            install_dependencies
            release
            ;;
        *)
            echo "Usage: $0 [build|dev|clean|release]"
            echo ""
            echo "Commands:"
            echo "  build    - Build the application for release"
            echo "  dev      - Run in development mode"
            echo "  clean    - Remove build artifacts"
            echo "  release  - Build and create installer"
            exit 1
            ;;
    esac
}

main "$@"