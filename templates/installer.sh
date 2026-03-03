#!/bin/bash
# FastPack Ultra-Fast Installer
# Optimized for maximum speed and minimal overhead

set -e

INSTALLER_VERSION="1.0.0"
INSTALLER_NAME="FastPack Installer"

COLOR_RESET='\033[0m'
COLOR_GREEN='\033[0;32m'
COLOR_BLUE='\033[0;34m'
COLOR_YELLOW='\033[1;33m'
COLOR_RED='\033[0;31m'

log_info() {
    echo -e "${COLOR_BLUE}[INFO]${COLOR_RESET} $1"
}

log_success() {
    echo -e "${COLOR_GREEN}[SUCCESS]${COLOR_RESET} $1"
}

log_warning() {
    echo -e "${COLOR_YELLOW}[WARNING]${COLOR_RESET} $1"
}

log_error() {
    echo -e "${COLOR_RED}[ERROR]${COLOR_RESET} $1"
}

print_header() {
    echo -e "${COLOR_GREEN}"
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                                                            ║"
    echo "║         $INSTALLER_NAME v$INSTALLER_VERSION         ║"
    echo "║                                                            ║"
    echo "║              Ultra-Fast Package Installer                 ║"
    echo "║                                                            ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo -e "${COLOR_RESET}"
}

check_root() {
    if [ "$EUID" -ne 0 ]; then
        if command -v sudo &> /dev/null; then
            log_info "Requesting root privileges..."
            exec sudo bash "$0" "$@"
        else
            log_error "This script must be run as root"
            exit 1
        fi
    fi
}

detect_compression() {
    local file="$1"
    local magic
    magic=$(head -c 4 "$file" 2>/dev/null | od -A n -t x1 | tr -d ' \n')
    
    case "$magic" in
        28b52ffd) echo "zstd" ;;
        1f8b0808) echo "gzip" ;;
        fd377a58) echo "xz" ;;
        52617221) echo "rar" ;;
        504b0304) echo "zip" ;;
        *) echo "none" ;;
    esac
}

extract_archive() {
    local archive="$1"
    local dest="$2"
    local compression="$3"
    
    mkdir -p "$dest"
    
    case "$compression" in
        zstd)
            zstd -d -c "$archive" | tar -x -C "$dest"
            ;;
        gzip)
            tar -xzf "$archive" -C "$dest"
            ;;
        xz)
            tar -xJf "$archive" -C "$dest"
            ;;
        zip)
            unzip -q "$archive" -d "$dest"
            ;;
        none)
            tar -xf "$archive" -C "$dest"
            ;;
        *)
            log_error "Unsupported compression format: $compression"
            return 1
            ;;
    esac
}

install_files() {
    local source="$1"
    local dest="$2"
    
    log_info "Installing files to $dest..."
    
    if [ -d "$source" ]; then
        cp -r "$source"/* "$dest/"
        log_success "Files installed successfully"
    else
        log_error "Source directory not found: $source"
        return 1
    fi
}

create_symlinks() {
    local bin_dir="/usr/local/bin"
    local app_dir="$1"
    local app_name="$2"
    
    if [ -d "$app_dir/bin" ]; then
        for binary in "$app_dir/bin"/*; do
            if [ -f "$binary" ]; then
                local name=$(basename "$binary")
                ln -sf "$binary" "$bin_dir/$name"
                log_info "Created symlink: $bin_dir/$name"
            fi
        done
    fi
}

setup_permissions() {
    local install_dir="$1"
    
    log_info "Setting up permissions..."
    
    find "$install_dir" -type f -exec chmod 644 {} \;
    find "$install_dir" -type d -exec chmod 755 {} \;
    
    if [ -d "$install_dir/bin" ]; then
        find "$install_dir/bin" -type f -exec chmod 755 {} \;
    fi
    
    log_success "Permissions configured"
}

register_desktop_entry() {
    local install_dir="$1"
    local desktop_file="$install_dir/share/applications"/*.desktop
    
    if [ -f "$desktop_file" ]; then
        cp "$desktop_file" "/usr/share/applications/"
        log_info "Desktop entry registered"
    fi
}

cleanup() {
    local temp_dir="$1"
    
    if [ -n "$temp_dir" ] && [ -d "$temp_dir" ]; then
        rm -rf "$temp_dir"
        log_info "Cleaned up temporary files"
    fi
}

main() {
    print_header
    
    local script_path="$0"
    local install_dir="/opt/fastpack"
    local temp_dir=""
    local archive_offset=0
    
    check_root "$@"
    
    log_info "Starting installation..."
    
    temp_dir=$(mktemp -d)
    log_info "Created temporary directory: $temp_dir"
    
    local compression="none"
    local archive_marker="__ARCHIVE_START__"
    
    archive_offset=$(grep -abo "$archive_marker" "$script_path" | head -n 1 | cut -d ':' -f 1)
    
    if [ -n "$archive_offset" ]; then
        archive_offset=$((archive_offset + ${#archive_marker} + 1))
        local archive="$temp_dir/archive.tar"
        
        log_info "Extracting package archive..."
        tail -c +$archive_offset "$script_path" > "$archive"
        
        compression=$(detect_compression "$archive")
        log_info "Detected compression: $compression"
        
        extract_archive "$archive" "$temp_dir/extracted" "$compression"
        
        if [ -d "$temp_dir/extracted" ]; then
            install_files "$temp_dir/extracted" "$install_dir"
            setup_permissions "$install_dir"
            create_symlinks "$install_dir" "fastpack"
            register_desktop_entry "$install_dir"
            
            log_success "Installation completed successfully!"
            log_info "Package installed to: $install_dir"
        else
            log_error "Failed to extract archive"
            cleanup "$temp_dir"
            exit 1
        fi
    else
        log_warning "No embedded archive found, installing from source..."
        install_dir="$1"
        
        if [ -z "$install_dir" ]; then
            install_dir="/opt/fastpack"
        fi
        
        mkdir -p "$install_dir"
        log_info "Installing to: $install_dir"
        
        log_success "Installation completed!"
    fi
    
    cleanup "$temp_dir"
    
    echo ""
    log_success "FastPack has been installed successfully!"
    log_info "Run 'fastpack' to start the application"
    echo ""
}

trap 'cleanup "$temp_dir"' EXIT

main "$@"

exit 0

__ARCHIVE_START__