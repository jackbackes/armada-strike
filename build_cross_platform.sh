#!/bin/bash

# Cross-platform build script using Docker and the 'cross' tool
# This provides a reliable way to build for Windows and Linux from macOS

set -e

echo "============================================"
echo "   Armada Strike Cross-Platform Builder"
echo "============================================"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to print colored messages
print_status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_info() {
    echo -e "${BLUE}[i]${NC} $1"
}

# Check for Docker
check_docker() {
    if command_exists docker; then
        if docker info >/dev/null 2>&1; then
            print_status "Docker is installed and running"
            return 0
        else
            print_error "Docker is installed but not running"
            echo "Please start Docker Desktop and try again"
            return 1
        fi
    else
        print_error "Docker is not installed"
        echo ""
        echo "To install Docker:"
        echo "  1. Visit: https://www.docker.com/products/docker-desktop"
        echo "  2. Download and install Docker Desktop for Mac"
        echo "  3. Start Docker Desktop"
        echo "  4. Run this script again"
        return 1
    fi
}

# Check for cross tool
check_cross() {
    if command_exists cross; then
        print_status "cross is installed ($(cross --version))"
        return 0
    else
        print_warning "cross is not installed"
        echo "Installing cross..."
        if cargo install cross --git https://github.com/cross-rs/cross; then
            print_status "cross installed successfully"
            return 0
        else
            print_error "Failed to install cross"
            return 1
        fi
    fi
}

# Build for a specific target
build_target() {
    local target=$1
    local description=$2
    
    echo ""
    echo "Building: $description"
    echo "Target: $target"
    echo "----------------------------------------"
    
    if cross build --release --target "$target"; then
        print_status "Build successful!"
        
        # Create distribution directory
        local dist_dir="dist/$target"
        mkdir -p "$dist_dir"
        
        # Determine the output binary name
        local binary_name="armada-strike"
        local binary_ext=""
        
        if [[ "$target" == *"windows"* ]]; then
            binary_ext=".exe"
        fi
        
        # Copy the binary
        if [ -f "target/$target/release/${binary_name}${binary_ext}" ]; then
            cp "target/$target/release/${binary_name}${binary_ext}" "$dist_dir/"
            print_status "Binary copied to $dist_dir/"
        else
            print_warning "Binary not found at expected location"
        fi
        
        # Copy assets
        cp -r assets "$dist_dir/" 2>/dev/null || true
        
        # Create platform-specific README
        create_readme "$target" "$dist_dir"
        
        # Create archive
        create_archive "$target" "$dist_dir" "$description"
        
        return 0
    else
        print_error "Build failed for $target"
        return 1
    fi
}

# Create platform-specific README
create_readme() {
    local target=$1
    local dist_dir=$2
    
    if [[ "$target" == *"windows"* ]]; then
        cat > "$dist_dir/README.txt" << 'EOF'
ARMADA STRIKE - Windows Edition
================================

TO RUN THE GAME:
Double-click "armada-strike.exe"

CONTROLS:
- 1-5: Select ship to place
- R: Rotate ship during placement
- Click: Place ship or fire at target
- Arrow Keys: Move cursor
- Tab: Switch between boards
- Ctrl+S: Save game
- Ctrl+L: Load game
- ESC: Settings menu

TROUBLESHOOTING:
If the game doesn't start, you may need to install:
- Visual C++ Redistributables: https://aka.ms/vs/17/release/vc_redist.x64.exe

EOF
    elif [[ "$target" == *"linux"* ]]; then
        cat > "$dist_dir/README.txt" << 'EOF'
ARMADA STRIKE - Linux Edition
==============================

TO RUN THE GAME:
1. Open a terminal in this directory
2. Make the game executable: chmod +x armada-strike
3. Run: ./armada-strike

CONTROLS:
- 1-5: Select ship to place
- R: Rotate ship during placement
- Click: Place ship or fire at target
- Arrow Keys: Move cursor
- Tab: Switch between boards
- Ctrl+S: Save game
- Ctrl+L: Load game
- ESC: Settings menu

DEPENDENCIES:
The game requires ALSA sound libraries. Install with:
- Ubuntu/Debian: sudo apt-get install libasound2
- Fedora: sudo dnf install alsa-lib
- Arch: sudo pacman -S alsa-lib

EOF
    else
        cat > "$dist_dir/README.txt" << 'EOF'
ARMADA STRIKE
=============

TO RUN: Execute the armada-strike binary

CONTROLS:
- 1-5: Select ship to place
- R: Rotate ship during placement
- Click: Place ship or fire at target
- Arrow Keys: Move cursor
- Tab: Switch between boards
- Ctrl+S: Save game
- Ctrl+L: Load game
- ESC: Settings menu

EOF
    fi
}

# Create distribution archive
create_archive() {
    local target=$1
    local dist_dir=$2
    local description=$3
    
    local archive_name="armada-strike-${target}"
    
    echo -n "Creating archive... "
    
    if [[ "$target" == *"windows"* ]]; then
        # Create zip for Windows
        (cd dist && zip -qr "${archive_name}.zip" "$target")
        print_status "Created dist/${archive_name}.zip"
    else
        # Create tar.gz for Linux/Unix
        (cd dist && tar -czf "${archive_name}.tar.gz" "$target")
        print_status "Created dist/${archive_name}.tar.gz"
    fi
}

# Main build process
main() {
    echo ""
    print_info "Checking prerequisites..."
    
    # Check Docker
    if ! check_docker; then
        exit 1
    fi
    
    # Check cross
    if ! check_cross; then
        exit 1
    fi
    
    # Parse command line arguments
    TARGETS=()
    
    if [ $# -eq 0 ]; then
        # Default: build for common platforms
        TARGETS=(
            "x86_64-pc-windows-gnu:Windows 64-bit"
            "x86_64-unknown-linux-gnu:Linux 64-bit"
        )
    else
        while [[ $# -gt 0 ]]; do
            case $1 in
                --windows|--win)
                    TARGETS+=("x86_64-pc-windows-gnu:Windows 64-bit")
                    TARGETS+=("i686-pc-windows-gnu:Windows 32-bit")
                    ;;
                --linux)
                    TARGETS+=("x86_64-unknown-linux-gnu:Linux 64-bit")
                    TARGETS+=("x86_64-unknown-linux-musl:Linux 64-bit (static)")
                    ;;
                --all)
                    TARGETS=(
                        "x86_64-pc-windows-gnu:Windows 64-bit"
                        "i686-pc-windows-gnu:Windows 32-bit"
                        "x86_64-unknown-linux-gnu:Linux 64-bit"
                        "x86_64-unknown-linux-musl:Linux 64-bit (static)"
                        "aarch64-unknown-linux-gnu:Linux ARM64"
                    )
                    ;;
                --help)
                    echo "Usage: $0 [OPTIONS]"
                    echo ""
                    echo "Options:"
                    echo "  --windows    Build for Windows (x86_64 and i686)"
                    echo "  --linux      Build for Linux (x86_64 gnu and musl)"
                    echo "  --all        Build for all supported platforms"
                    echo "  --help       Show this help message"
                    echo ""
                    echo "If no options are provided, builds for Windows x64 and Linux x64"
                    echo ""
                    echo "Examples:"
                    echo "  $0                    # Build Windows x64 and Linux x64"
                    echo "  $0 --windows          # Build all Windows targets"
                    echo "  $0 --all              # Build everything"
                    exit 0
                    ;;
                *)
                    print_error "Unknown option: $1"
                    echo "Use --help for usage information"
                    exit 1
                    ;;
            esac
            shift
        done
    fi
    
    # Remove duplicates from TARGETS
    TARGETS=($(printf "%s\n" "${TARGETS[@]}" | sort -u))
    
    echo ""
    print_info "Build targets:"
    for target_info in "${TARGETS[@]}"; do
        IFS=':' read -r target description <<< "$target_info"
        echo "  - $description ($target)"
    done
    
    # Create distribution directory
    mkdir -p dist
    
    # Build each target
    echo ""
    print_info "Starting builds..."
    
    SUCCESS_COUNT=0
    FAIL_COUNT=0
    
    for target_info in "${TARGETS[@]}"; do
        IFS=':' read -r target description <<< "$target_info"
        if build_target "$target" "$description"; then
            ((SUCCESS_COUNT++))
        else
            ((FAIL_COUNT++))
        fi
    done
    
    # Summary
    echo ""
    echo "============================================"
    if [ $SUCCESS_COUNT -gt 0 ]; then
        print_status "Build Summary: $SUCCESS_COUNT successful, $FAIL_COUNT failed"
        echo ""
        echo "Distribution files created in: dist/"
        echo ""
        echo "To distribute:"
        echo "  - Windows: Share the .zip file"
        echo "  - Linux: Share the .tar.gz file"
        echo ""
        echo "Users should extract the archive and run the executable."
    else
        print_error "All builds failed"
        echo ""
        echo "Common issues:"
        echo "  1. Make sure Docker is running"
        echo "  2. Check your internet connection (Docker needs to download images)"
        echo "  3. Ensure you have enough disk space"
    fi
    echo "============================================"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found"
    echo "Please run this script from the project root directory"
    exit 1
fi

# Run main function
main "$@"