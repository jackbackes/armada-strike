# Building Armada Strike for Multiple Platforms

This guide explains how to build Armada Strike for Windows, Linux, and macOS from any development machine.

## Quick Start

```bash
# Build for your current platform
make release

# Build for all platforms (requires Docker)
make build-all

# Test what platforms you can build for
make test-cross
```

## Platform-Specific Builds

### macOS

Native builds on macOS are straightforward:

```bash
# Debug build
make build

# Release build with DMG installer
make dmg
```

The DMG installer will be created as `Armada-Strike-Installer.dmg`.

### Windows

There are three ways to build for Windows:

#### Option 1: Cross-compilation with MinGW (from macOS/Linux)

First, install MinGW-w64:
```bash
# macOS
brew install mingw-w64

# Ubuntu/Debian
sudo apt-get install mingw-w64

# Fedora
sudo dnf install mingw64-gcc mingw32-gcc
```

Then build:
```bash
# Build for both 32-bit and 64-bit Windows
make windows

# Build for 64-bit Windows only
make windows-x64

# Build for 32-bit Windows only
make windows-x86
```

Output will be in `dist/windows/`.

#### Option 2: Docker-based Cross-compilation (Recommended)

This is the most reliable method and works on any platform with Docker:

1. Install Docker Desktop from https://www.docker.com/products/docker-desktop

2. Start Docker Desktop

3. Build:
```bash
# Build for Windows and Linux using Docker
make cross-build

# Or use the script directly for more options
./build_cross_platform.sh --all
```

Output will be in `dist/` with platform-specific archives.

#### Option 3: Native Windows Build

On a Windows machine with Rust installed:
```bash
cargo build --release
```

### Linux

#### Cross-compilation from macOS/Windows

Using Docker (recommended):
```bash
# Ensure Docker is running
make cross-build
```

Using the cross tool:
```bash
# Install cross
cargo install cross

# Build for Linux x64
cross build --release --target x86_64-unknown-linux-gnu

# Build static binary (works on any Linux)
cross build --release --target x86_64-unknown-linux-musl
```

#### Native Linux Build

On a Linux machine:
```bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get install libasound2-dev libudev-dev

# Build
cargo build --release
```

## Build Matrix

| Host OS | Target OS | Method | Requirements |
|---------|-----------|--------|--------------|
| macOS | macOS | Native | Rust toolchain |
| macOS | Windows | Cross-compile | MinGW-w64 or Docker |
| macOS | Linux | Cross-compile | Docker |
| Windows | Windows | Native | Rust toolchain |
| Windows | Linux | Cross-compile | Docker or WSL2 |
| Windows | macOS | Not supported | - |
| Linux | Linux | Native | Rust toolchain |
| Linux | Windows | Cross-compile | MinGW-w64 or Docker |
| Linux | macOS | Not supported | - |

## Testing Cross-Compilation

To see what platforms you can build for on your current system:

```bash
make test-cross
```

This will:
1. Check for required tools (MinGW, Docker, etc.)
2. Test compilation for various targets
3. Generate a report in `cross_compile_report.txt`

## Distribution

After building, you'll find distributable packages:

- **macOS**: `Armada-Strike-Installer.dmg`
- **Windows**: `dist/armada-strike-x86_64-pc-windows-gnu.zip`
- **Linux**: `dist/armada-strike-x86_64-unknown-linux-gnu.tar.gz`

Each package includes:
- The game executable
- Game assets (sounds, sprites)
- Platform-specific README with instructions

## Troubleshooting

### "Docker not found"
Install Docker Desktop from https://www.docker.com/products/docker-desktop

### "MinGW not found"
- macOS: `brew install mingw-w64`
- Linux: `sudo apt-get install mingw-w64`

### "cargo: command not found"
Install Rust from https://rustup.rs/

### Build fails with "linking failed"
This usually means you're missing the cross-compilation toolchain. Use Docker-based builds instead:
```bash
make cross-build
```

### "target not found"
Install the target with rustup:
```bash
rustup target add x86_64-pc-windows-gnu
```

## Advanced Options

### Custom Build Targets

Edit `build_cross_platform.sh` to add new targets:
```bash
./build_cross_platform.sh --help
```

### Optimized Builds

For smallest binary size:
```toml
# In Cargo.toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

### WebAssembly Build (Experimental)

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

Note: WebAssembly support requires additional setup for Bevy's web renderer.

## CI/CD Integration

For GitHub Actions, use the provided workflow:
```yaml
name: Build All Platforms

on: [push, pull_request]

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
```

## Support

For build issues, please check:
1. The `cross_compile_report.txt` file after running `make test-cross`
2. Docker logs if using Docker-based builds
3. The project's issue tracker on GitHub