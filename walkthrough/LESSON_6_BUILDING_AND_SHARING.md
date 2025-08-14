# Lesson 6: Building and Sharing Your Game - A Code Walkthrough for Logan

Hey Logan! You've learned how the game works - now let's learn how to build it into something you can share with friends and family, and understand why it works differently on different computers!

## Debug vs Release: Two Ways to Build

When you run `cargo run`, you're using **Debug mode** - like riding a bike with training wheels:
- Slower but safer
- Shows helpful error messages
- Easier to find problems
- Files are bigger

When you run `cargo build --release`, you're using **Release mode** - like riding without training wheels:
- MUCH faster (10-100x!)
- Smaller file size
- Ready to share
- No debug information

Try both:
```bash
cargo run                # Debug: might take 2 seconds to start
cargo run --release      # Release: starts instantly!
```

## Mac vs PC: Why Are Computers Different?

Imagine Mac and PC as two people who speak different languages:
- **Mac** speaks "Mac language" (called Darwin/Unix)
- **PC** speaks "Windows language" 
- **Linux** speaks "Linux language" (similar to Mac's)

Your Rust code is like a recipe written in English. The compiler (`rustc`) is like a translator that converts your recipe into the language each computer understands!

### File Differences

On **Mac**:
- Programs are usually `.app` bundles (folders that look like files)
- Can also be plain executable files (no extension)
- Example: `ArmadaStrike.app` or just `armada-strike`

On **Windows**:
- Programs end in `.exe`
- Example: `armada-strike.exe`
- Need all `.dll` files (Dynamic Link Libraries - helper files)

On **Linux**:
- Like Mac, no extension needed
- Example: `armada-strike`
- Needs permission to execute (`chmod +x`)

## Building for Your Computer

The simple way - build for whatever computer you're using:

```bash
cargo build --release
```

Your game will be in:
- Mac/Linux: `target/release/armada-strike`
- Windows: `target/release/armada-strike.exe`

## Building for Other Computers (Cross-Compilation)

Want to build a Windows version on your Mac? Or a Linux version on Windows? We've made it easy!

### Method 1: Using Our Build Scripts (Easiest!)

We have special scripts that handle all the complexity:

```bash
# Test what platforms you can build for
make test-cross

# Build for Windows (from Mac/Linux)
make windows         # Both 32-bit and 64-bit
make windows-x64     # 64-bit only
make windows-x86     # 32-bit only

# Build for all platforms using Docker
make cross-build     # Builds Windows + Linux versions
make build-all       # Builds EVERYTHING
```

### Method 2: Using Docker (Most Reliable)

Docker is like a computer inside your computer! It can pretend to be any operating system:

1. **Install Docker Desktop** from https://docker.com
2. **Start Docker Desktop**
3. **Run our cross-build script:**
```bash
./build_cross_platform.sh --all
```

This creates:
- `dist/armada-strike-x86_64-pc-windows-gnu.zip` (Windows 64-bit)
- `dist/armada-strike-i686-pc-windows-gnu.zip` (Windows 32-bit)
- `dist/armada-strike-x86_64-unknown-linux-gnu.tar.gz` (Linux)

### Method 3: Manual Cross-Compilation (Advanced)

#### For Mac Users Building for Windows:
```bash
# Install the Windows target
rustup target add x86_64-pc-windows-gnu

# Install cross-compilation tools
brew install mingw-w64

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu

# Or use our script
./build_windows.sh
```

#### For Windows Users Building for Linux:
```bash
# Install WSL2 (Windows Subsystem for Linux) first
# Then in WSL2:
cargo build --release

# Or use Docker method above
```

### Understanding Cross-Compilation

Think of it like translating a book:
- Your code is written in Rust (like English)
- Each computer type needs a different translation
- The compiler does the translation
- Cross-compilation = translating for a computer you don't have!

**Target Triple:** `x86_64-pc-windows-gnu`
- `x86_64` = 64-bit processor (or `i686` for 32-bit)
- `pc` = Personal Computer
- `windows` = Operating System
- `gnu` = Toolchain type (or `msvc` for Microsoft's)

## Creating an Installer for Mac (.dmg file)

On Mac, we create a `.dmg` file - like a virtual disk that contains your app:

```bash
# Run the build script
make dmg
# or
./create_dmg.sh
```

This script (look at `create_dmg.sh`):
1. Creates `ArmadaStrike.app` folder structure
2. Copies your game inside
3. Adds an icon
4. Creates a `.dmg` disk image
5. Result: `ArmadaStrike-Installer.dmg` that you can share!

When someone opens the DMG:
1. A window appears
2. They drag ArmadaStrike to Applications
3. They can run it from Applications folder

## Creating an Installer for Windows

Windows has different options:

### Option 1: Zip File (Simplest)
```powershell
# Build the game
cargo build --release

# Create a folder with everything needed
mkdir ArmadaStrike
copy target\release\armada-strike.exe ArmadaStrike\
copy -r assets ArmadaStrike\

# Zip it
Compress-Archive -Path ArmadaStrike -DestinationPath ArmadaStrike.zip
```

### Option 2: Real Installer (Advanced)
Use tools like:
- **Inno Setup** (free, creates .exe installers)
- **WiX** (Microsoft's tool, more complex)
- **NSIS** (open source, very flexible)

## The Build Scripts Explained

Look at `Makefile` - it's like a recipe book for building:

```makefile
# Build debug version (for testing)
build:
    cargo build

# Build release version (for sharing)  
release:
    cargo build --release

# Create Mac installer
dmg: release
    ./create_dmg.sh

# Clean everything and start fresh
clean:
    cargo clean
    rm -rf ArmadaStrike.app
```

You can run these with:
```bash
make build      # Build debug
make release    # Build release
make dmg        # Create Mac installer
make clean      # Delete all built files
```

## Understanding `Cargo.toml`

This file tells Rust what your game needs:

```toml
[package]
name = "armada-strike"        # Internal name
version = "0.1.0"             # Version number
edition = "2024"              # Rust edition (like a rulebook version)

[dependencies]
bevy = { version = "0.16" }   # Game engine
serde = { version = "1.0" }   # For saving games
rand = "0.8"                  # For random numbers
```

When you run `cargo build`, it:
1. Reads this file
2. Downloads all dependencies
3. Compiles everything together

## File Sizes: Why So Different?

You might notice:
- Debug build: 500+ MB (huge!)
- Release build: 50 MB (much smaller!)
- After stripping: 15 MB (even smaller!)

Why? Debug includes:
- Symbol tables (for debugging)
- No optimizations
- Extra safety checks

Release mode:
- Optimizes code
- Removes debug info
- Can be stripped further:
```bash
# Mac/Linux: Make it even smaller
strip target/release/armada-strike
```

## Sharing Your Game

### Method 1: GitHub Releases (Recommended)
1. Push your code to GitHub
2. Go to "Releases" → "Create new release"
3. Upload your `.dmg` (Mac) or `.zip` (Windows)
4. Share the download link!

### Method 2: Direct File Sharing
- Google Drive
- Dropbox
- WeTransfer
- USB drives

### Method 3: itch.io (Game Platform)
1. Create account on itch.io
2. Create new project
3. Upload your builds
4. Get a custom page for your game!

## Handling Different Screens

Your game needs to work on different screen sizes:

```rust
// In main.rs
WindowPlugin {
    primary_window: Some(Window {
        resolution: (1000.0, 700.0).into(),  // Default size
        resizable: true,                     // Can be resized!
        ..default()
    }),
}
```

## Version Numbers

In `Cargo.toml`:
```toml
version = "0.1.0"
```

This means:
- 0 = Major version (big changes)
- 1 = Minor version (new features)
- 0 = Patch version (bug fixes)

When you:
- Fix a bug: 0.1.0 → 0.1.1
- Add feature: 0.1.0 → 0.2.0
- Big rewrite: 0.1.0 → 1.0.0

## Troubleshooting Build Issues

### "Command not found"
- Make sure Rust is installed
- Restart terminal after installing

### "Permission denied" (Mac/Linux)
```bash
chmod +x create_dmg.sh
chmod +x target/release/armada-strike
```

### Game won't run on friend's computer
- Mac: They need to right-click → Open (first time)
- Windows: They might need to click "Run anyway" if Windows Defender warns
- Missing libraries: Include all files from target folder

### Build takes forever
- First build downloads dependencies (normal)
- Use `cargo build --release -j 4` to use 4 CPU cores
- Close other programs

## Platform-Specific Code

Sometimes you need different code for different systems:

```rust
#[cfg(target_os = "windows")]
fn get_save_dir() -> PathBuf {
    // Windows-specific path
}

#[cfg(target_os = "macos")]
fn get_save_dir() -> PathBuf {
    // Mac-specific path
}

#[cfg(target_os = "linux")]
fn get_save_dir() -> PathBuf {
    // Linux-specific path
}
```

## What You've Learned

✅ Debug vs Release builds  
✅ Why Mac/PC/Linux are different  
✅ How to build for your platform  
✅ Creating installers (.dmg for Mac)  
✅ Understanding build scripts  
✅ How to share your game  
✅ Handling version numbers  
✅ Troubleshooting common issues  

## Homework Challenge

1. Build a release version of the game
2. Check the file size difference between debug and release
3. If on Mac, try creating a DMG
4. Change the version in Cargo.toml to "0.2.0"
5. Build again - you've created version 0.2.0!

## Advanced Challenge: GitHub Actions

Create `.github/workflows/build.yml`:
```yaml
name: Build Game

on: [push]

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
    - uses: actions/checkout@v2
    - uses: actions-rs/toolchain@v1
    - run: cargo build --release
```

This automatically builds your game for all platforms when you push to GitHub!

## Professional Tips

1. **Always test release builds** - They might behave differently than debug
2. **Include a README** - Tell people how to run your game
3. **Version everything** - Use git tags for releases
4. **Test on different computers** - Borrow a friend's if needed
5. **Keep old versions** - In case new ones have problems

## Next Steps After Building

1. **Get feedback** - Share with friends and family
2. **Fix bugs** - They'll find things you missed
3. **Add features** - Based on feedback
4. **Polish** - Add menu, better graphics, music
5. **Publish** - itch.io, Steam (when you're ready!)

## Congratulations!

You now know how to:
- Build games for different platforms
- Create installers
- Share your creations with the world

This is huge! Most people who play games have no idea how they're built and distributed. You're now part of the small group who understands the full journey from code to playable game!

---

**Remember:** The first version doesn't need to be perfect. Ship it, get feedback, improve it. That's how all great games are made!

Your journey from player to developer is complete. Now go build something amazing! 🎮🚀