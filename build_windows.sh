#!/bin/bash

# Build script for Windows cross-compilation from macOS/Linux
# Supports both x86_64 (64-bit) and i686 (32-bit) Windows targets

set -e

echo "========================================="
echo "   Armada Strike - Windows Build Script"
echo "========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

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

# Check for Rust
if ! command_exists rustc; then
    print_error "Rust is not installed. Please install Rust first."
    echo "Visit: https://rustup.rs/"
    exit 1
fi

# Function to install Windows targets
install_windows_targets() {
    echo ""
    echo "Installing Windows compilation targets..."
    
    # x86_64 Windows targets
    if rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
        print_status "x86_64-pc-windows-gnu already installed"
    else
        print_warning "Installing x86_64-pc-windows-gnu target..."
        rustup target add x86_64-pc-windows-gnu
    fi
    
    if rustup target list | grep -q "x86_64-pc-windows-msvc (installed)"; then
        print_status "x86_64-pc-windows-msvc already installed"
    else
        print_warning "Installing x86_64-pc-windows-msvc target..."
        rustup target add x86_64-pc-windows-msvc || print_warning "MSVC target may require Visual Studio on Windows"
    fi
    
    # i686 (32-bit) Windows targets
    if rustup target list | grep -q "i686-pc-windows-gnu (installed)"; then
        print_status "i686-pc-windows-gnu already installed"
    else
        print_warning "Installing i686-pc-windows-gnu target..."
        rustup target add i686-pc-windows-gnu
    fi
}

# Function to check for MinGW (needed for GNU targets)
check_mingw() {
    echo ""
    echo "Checking for MinGW-w64 (required for Windows GNU targets)..."
    
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        if command_exists x86_64-w64-mingw32-gcc; then
            print_status "MinGW-w64 found"
        else
            print_warning "MinGW-w64 not found. To install:"
            echo "  brew install mingw-w64"
            return 1
        fi
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        if command_exists x86_64-w64-mingw32-gcc; then
            print_status "MinGW-w64 found"
        else
            print_warning "MinGW-w64 not found. To install:"
            echo "  Ubuntu/Debian: sudo apt-get install mingw-w64"
            echo "  Fedora: sudo dnf install mingw64-gcc"
            echo "  Arch: sudo pacman -S mingw-w64"
            return 1
        fi
    fi
    return 0
}

# Function to build for a specific target
build_target() {
    local target=$1
    local build_type=$2
    
    echo ""
    echo "Building for target: $target ($build_type)"
    echo "----------------------------------------"
    
    if [[ "$build_type" == "release" ]]; then
        cargo build --release --target "$target" 2>&1 | while IFS= read -r line; do
            echo "  $line"
        done
        
        if [ ${PIPESTATUS[0]} -eq 0 ]; then
            print_status "Build successful for $target"
            
            # Create output directory
            mkdir -p "dist/windows/$target"
            
            # Copy the executable
            if [ -f "target/$target/release/armada-strike.exe" ]; then
                cp "target/$target/release/armada-strike.exe" "dist/windows/$target/"
                print_status "Executable copied to dist/windows/$target/"
            fi
            
            # Copy assets
            cp -r assets "dist/windows/$target/"
            print_status "Assets copied"
            
            return 0
        else
            print_error "Build failed for $target"
            return 1
        fi
    else
        cargo build --target "$target" 2>&1 | while IFS= read -r line; do
            echo "  $line"
        done
        
        if [ ${PIPESTATUS[0]} -eq 0 ]; then
            print_status "Debug build successful for $target"
            return 0
        else
            print_error "Debug build failed for $target"
            return 1
        fi
    fi
}

# Function to create Windows installer script
create_installer_script() {
    local target=$1
    
    cat > "dist/windows/$target/install.bat" << 'EOF'
@echo off
echo ==========================================
echo    Armada Strike - Windows Installer
echo ==========================================
echo.

set INSTALL_DIR=%LOCALAPPDATA%\ArmadaStrike

echo Installing to: %INSTALL_DIR%
echo.

:: Create installation directory
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

:: Copy files
echo Copying game files...
xcopy /E /Y /Q "assets" "%INSTALL_DIR%\assets\" > nul
copy /Y "armada-strike.exe" "%INSTALL_DIR%\" > nul

:: Create Start Menu shortcut
echo Creating Start Menu shortcut...
powershell -Command "$WshShell = New-Object -ComObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%APPDATA%\Microsoft\Windows\Start Menu\Programs\Armada Strike.lnk'); $Shortcut.TargetPath = '%INSTALL_DIR%\armada-strike.exe'; $Shortcut.WorkingDirectory = '%INSTALL_DIR%'; $Shortcut.Description = 'Armada Strike - Naval Combat Game'; $Shortcut.Save()"

:: Create Desktop shortcut (optional)
choice /C YN /M "Create Desktop shortcut"
if %ERRORLEVEL% EQU 1 (
    powershell -Command "$WshShell = New-Object -ComObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%USERPROFILE%\Desktop\Armada Strike.lnk'); $Shortcut.TargetPath = '%INSTALL_DIR%\armada-strike.exe'; $Shortcut.WorkingDirectory = '%INSTALL_DIR%'; $Shortcut.Description = 'Armada Strike - Naval Combat Game'; $Shortcut.Save()"
    echo Desktop shortcut created.
)

echo.
echo Installation complete!
echo.
echo You can find Armada Strike in your Start Menu.
echo.
pause
EOF
    
    print_status "Windows installer script created"
}

# Function to create README for Windows distribution
create_windows_readme() {
    local target=$1
    
    cat > "dist/windows/$target/README.txt" << 'EOF'
ARMADA STRIKE - Windows Edition
================================

QUICK START:
1. Run "install.bat" to install the game
2. Find "Armada Strike" in your Start Menu
3. Or run "armada-strike.exe" directly

SYSTEM REQUIREMENTS:
- Windows 7 or later
- DirectX 11 compatible graphics
- 100 MB free disk space

CONTROLS:
- 1-5: Select ship to place
- R: Rotate ship during placement
- Click: Place ship or fire at target
- Arrow Keys: Move cursor
- Tab: Switch between boards
- S: Save game
- L: Load game
- ESC: Settings menu

TROUBLESHOOTING:

If the game doesn't start:
1. Make sure you have Visual C++ Redistributables installed
   Download from: https://aka.ms/vs/17/release/vc_redist.x64.exe

2. Check that your antivirus isn't blocking the game

3. Try running as Administrator (right-click -> Run as administrator)

If you have no sound:
- Check the game settings (ESC key)
- Ensure Windows audio is not muted

SAVE GAME LOCATION:
%APPDATA%\armada-strike\saves\

UNINSTALL:
1. Delete the game folder from %LOCALAPPDATA%\ArmadaStrike
2. Delete shortcuts from Desktop and Start Menu
3. Delete save games from %APPDATA%\armada-strike (optional)

For support, visit: https://github.com/yourusername/armada-strike

Enjoy the game!
EOF
    
    print_status "Windows README created"
}

# Main build process
main() {
    echo ""
    echo "Checking prerequisites..."
    
    # Install Windows targets
    install_windows_targets
    
    # Check for MinGW (only warn, don't fail)
    MINGW_AVAILABLE=false
    if check_mingw; then
        MINGW_AVAILABLE=true
    fi
    
    # Parse command line arguments
    BUILD_TYPE="release"
    TARGET_ARCH="all"
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --debug)
                BUILD_TYPE="debug"
                shift
                ;;
            --x86_64|--64bit)
                TARGET_ARCH="x86_64"
                shift
                ;;
            --i686|--32bit)
                TARGET_ARCH="i686"
                shift
                ;;
            --help)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --debug        Build debug version instead of release"
                echo "  --x86_64       Build only 64-bit version"
                echo "  --i686         Build only 32-bit version"
                echo "  --help         Show this help message"
                echo ""
                echo "Examples:"
                echo "  $0                    # Build all release versions"
                echo "  $0 --debug           # Build all debug versions"
                echo "  $0 --x86_64          # Build only 64-bit release"
                echo "  $0 --debug --i686    # Build only 32-bit debug"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                echo "Use --help for usage information"
                exit 1
                ;;
        esac
    done
    
    echo ""
    echo "Build configuration:"
    echo "  Type: $BUILD_TYPE"
    echo "  Architecture: $TARGET_ARCH"
    
    # Create distribution directory
    mkdir -p dist/windows
    
    # Build based on selected architecture
    BUILD_SUCCESS=false
    
    if [[ "$TARGET_ARCH" == "all" ]] || [[ "$TARGET_ARCH" == "x86_64" ]]; then
        if [[ "$MINGW_AVAILABLE" == true ]]; then
            if build_target "x86_64-pc-windows-gnu" "$BUILD_TYPE"; then
                BUILD_SUCCESS=true
                if [[ "$BUILD_TYPE" == "release" ]]; then
                    create_installer_script "x86_64-pc-windows-gnu"
                    create_windows_readme "x86_64-pc-windows-gnu"
                fi
            fi
        else
            print_warning "Skipping x86_64-pc-windows-gnu (MinGW not available)"
        fi
    fi
    
    if [[ "$TARGET_ARCH" == "all" ]] || [[ "$TARGET_ARCH" == "i686" ]]; then
        if [[ "$MINGW_AVAILABLE" == true ]]; then
            if build_target "i686-pc-windows-gnu" "$BUILD_TYPE"; then
                BUILD_SUCCESS=true
                if [[ "$BUILD_TYPE" == "release" ]]; then
                    create_installer_script "i686-pc-windows-gnu"
                    create_windows_readme "i686-pc-windows-gnu"
                fi
            fi
        else
            print_warning "Skipping i686-pc-windows-gnu (MinGW not available)"
        fi
    fi
    
    # Summary
    echo ""
    echo "========================================="
    if [[ "$BUILD_SUCCESS" == true ]]; then
        print_status "Build completed successfully!"
        echo ""
        echo "Windows builds are located in:"
        echo "  dist/windows/"
        echo ""
        echo "To distribute:"
        echo "  1. Zip the contents of dist/windows/[target]/"
        echo "  2. Users can run install.bat for easy installation"
        echo "  3. Or run armada-strike.exe directly"
    else
        print_error "No successful builds completed"
        echo ""
        echo "To enable Windows cross-compilation from macOS:"
        echo "  1. Install MinGW-w64: brew install mingw-w64"
        echo "  2. Run this script again"
        echo ""
        echo "Alternatively, build on a Windows machine with:"
        echo "  cargo build --release"
    fi
    echo "========================================="
}

# Run main function
main "$@"