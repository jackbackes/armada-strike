# Getting Started with Armada Strike

Welcome! This guide will help you get Armada Strike running on your computer, whether you're a player wanting to enjoy the game or a developer wanting to build it yourself.

## 🎮 Just Want to Play?

### Windows Users

1. **Download the Game**
   - Go to [Releases](https://github.com/jackbackes/armada-strike/releases)
   - Download `armada-strike-windows-x64.zip` (64-bit) or `armada-strike-windows-x86.zip` (32-bit)
   - Most modern PCs use 64-bit

2. **Extract and Run**
   - Right-click the ZIP file → "Extract All"
   - Open the extracted folder
   - Double-click `armada-strike.exe`
   - If Windows Defender warns, click "More info" → "Run anyway"

3. **Troubleshooting**
   - Missing DLL errors? Install [Visual C++ Redistributables](https://aka.ms/vs/17/release/vc_redist.x64.exe)
   - No sound? Check Windows volume and in-game settings (ESC key)

### Mac Users

1. **Download the Game**
   - Go to [Releases](https://github.com/jackbackes/armada-strike/releases)
   - Download `Armada-Strike-Installer.dmg`

2. **Install**
   - Double-click the DMG file
   - Drag Armada Strike to your Applications folder
   - Eject the DMG (drag to trash or right-click → Eject)

3. **First Run**
   - Open Applications folder
   - Right-click Armada Strike → Open (first time only)
   - Click "Open" when macOS asks

4. **Troubleshooting**
   - "Can't be opened" error? Right-click → Open instead of double-clicking
   - No sound? Check System Preferences → Sound

### Linux Users

1. **Download the Game**
   - Go to [Releases](https://github.com/jackbackes/armada-strike/releases)
   - Download `armada-strike-linux-x64.tar.gz`

2. **Extract and Run**
   ```bash
   # Extract the archive
   tar -xzf armada-strike-linux-x64.tar.gz
   cd armada-strike-linux-x64
   
   # Make executable
   chmod +x armada-strike
   
   # Run the game
   ./armada-strike
   ```

3. **Install Dependencies** (if needed)
   ```bash
   # Ubuntu/Debian
   sudo apt-get install libasound2 libudev1
   
   # Fedora
   sudo dnf install alsa-lib libudev
   
   # Arch
   sudo pacman -S alsa-lib systemd-libs
   ```

## 🛠️ Want to Build from Source?

### Prerequisites (All Platforms)

1. **Install Rust**
   - Visit https://rustup.rs/
   - Follow instructions for your platform
   - Restart your terminal/command prompt

2. **Clone the Repository**
   ```bash
   git clone https://github.com/jackbackes/armada-strike.git
   cd armada-strike
   ```

### Windows Development Setup

1. **Install Build Tools**
   - Option A: Install [Visual Studio](https://visualstudio.microsoft.com/) with C++ tools
   - Option B: Install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)

2. **Build and Run**
   ```powershell
   # Build the game
   cargo build --release
   
   # Run it
   cargo run --release
   
   # Or use the target executable directly
   .\target\release\armada-strike.exe
   ```

3. **Create Distributable Package**
   ```powershell
   # Create a folder with all files
   mkdir dist
   copy target\release\armada-strike.exe dist\
   xcopy /E /I assets dist\assets
   
   # Zip it up
   Compress-Archive -Path dist\* -DestinationPath armada-strike-windows.zip
   ```

### Mac Development Setup

1. **Install Xcode Command Line Tools** (if not already installed)
   ```bash
   xcode-select --install
   ```

2. **Build and Run**
   ```bash
   # Quick build and run
   cargo run --release
   
   # Or use our Makefile
   make release    # Build optimized version
   make run        # Run debug version
   make dmg        # Create installer
   ```

3. **Create DMG Installer**
   ```bash
   make dmg
   # Creates Armada-Strike-Installer.dmg
   ```

### Linux Development Setup

1. **Install Dependencies**
   ```bash
   # Ubuntu/Debian
   sudo apt-get update
   sudo apt-get install build-essential libasound2-dev libudev-dev pkg-config
   
   # Fedora
   sudo dnf groupinstall "Development Tools"
   sudo dnf install alsa-lib-devel libudev-devel
   
   # Arch
   sudo pacman -S base-devel alsa-lib systemd-libs
   ```

2. **Build and Run**
   ```bash
   # Build and run
   cargo run --release
   
   # Or build only
   cargo build --release
   ./target/release/armada-strike
   ```

3. **Create Distribution Package**
   ```bash
   # Create distribution folder
   mkdir -p dist/armada-strike
   cp target/release/armada-strike dist/armada-strike/
   cp -r assets dist/armada-strike/
   
   # Create tarball
   cd dist
   tar -czf armada-strike-linux-x64.tar.gz armada-strike/
   ```

## 🌍 Cross-Platform Building

Want to build for other platforms from your current OS?

### Easy Mode: Use Docker

1. **Install Docker Desktop**
   - Download from https://docker.com
   - Install and start Docker Desktop

2. **Build for All Platforms**
   ```bash
   # Build Windows and Linux versions automatically
   make cross-build
   
   # Or build everything (Mac, Windows, Linux)
   make build-all
   ```

3. **Find Your Builds**
   - Check the `dist/` folder for platform-specific packages
   - Each includes the game and a README

### Advanced: Manual Cross-Compilation

See [BUILDING.md](BUILDING.md) for detailed cross-compilation instructions.

## 🎯 Quick Command Reference

### Using Make (Recommended)
```bash
make help        # Show all available commands
make build       # Build debug version
make release     # Build optimized version
make run         # Run the game
make clean       # Clean build artifacts
make dmg         # Create Mac installer
make windows     # Build for Windows
make cross-build # Build for multiple platforms
make test-cross  # Test what you can build
```

### Using Cargo Directly
```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo run               # Build and run debug
cargo run --release     # Build and run release
cargo clean             # Clean build artifacts
cargo test              # Run tests
```

## 🎮 Game Controls

Once you have the game running:

- **1-5**: Select ship to place
- **R**: Rotate ship during placement
- **Click**: Place ship or fire at target
- **Arrow Keys**: Move cursor
- **Tab**: Switch between boards
- **Ctrl+S**: Save game
- **Ctrl+L**: Load game
- **Ctrl+N**: New game
- **Ctrl+P**: Random ship placement
- **ESC**: Settings menu

## 📁 Save Game Locations

Your saved games are stored in:

- **Windows**: `%APPDATA%\armada-strike\saves\`
- **Mac**: `~/Library/Application Support/armada-strike/saves/`
- **Linux**: `~/.config/armada-strike/saves/`

## 🐛 Troubleshooting

### Common Issues

**"Rust not found"**
- Install from https://rustup.rs/
- Restart your terminal after installation

**"cargo: command not found"**
- Make sure Rust is in your PATH
- Try: `source ~/.cargo/env` (Mac/Linux)
- Or restart your terminal

**Build fails with "linking error"**
- Windows: Install Visual Studio Build Tools
- Mac: Install Xcode Command Line Tools
- Linux: Install build-essential

**Game won't start**
- Check you have the right version (32-bit vs 64-bit)
- Try running from terminal to see error messages
- Check antivirus isn't blocking it

**No sound**
- Check system volume
- Press ESC in game and check sound settings
- Linux: Ensure ALSA is installed

### Getting Help

1. Check the [Issues](https://github.com/jackbackes/armada-strike/issues) page
2. Read [BUILDING.md](BUILDING.md) for detailed build instructions
3. Look at the [walkthrough](walkthrough/) for code explanations

## 🚀 Next Steps

- **Players**: Enjoy the game! Check Settings (ESC) for options
- **Developers**: 
  - Read the [walkthrough](walkthrough/) to understand the code
  - Check [BUILDING.md](BUILDING.md) for advanced build options
  - Contribute improvements via Pull Requests!

## 📝 System Requirements

### Minimum
- **OS**: Windows 7+, macOS 10.12+, Ubuntu 18.04+
- **RAM**: 512 MB
- **Storage**: 100 MB
- **Graphics**: OpenGL 3.3 support

### Recommended
- **OS**: Windows 10/11, macOS 12+, Ubuntu 22.04+
- **RAM**: 1 GB
- **Storage**: 200 MB
- **Graphics**: Any modern GPU

## 📜 License

MIT License - See [LICENSE](LICENSE) file for details.

---

Happy gaming! 🎮 May your shots always hit their targets!