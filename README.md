# Armada Strike

A modern naval combat strategy game inspired by the classic children's game. Built with Rust and Bevy for high-performance cross-platform gameplay.

## Features

- Strategic ship placement and targeting
- Multiple save game slots with unique three-word names
- Retro 8-bit sound effects
- Clean, minimalist UI
- Cross-platform support (macOS, Linux, Windows)

## How to Play

1. Place your ships on the grid using number keys (1-5)
2. Take turns targeting enemy positions
3. First to sink all enemy ships wins!

## Quick Start

See [GETTING_STARTED.md](GETTING_STARTED.md) for detailed platform-specific instructions.

### Building

```bash
# Development build
cargo run

# Release build for your platform
make release

# Platform-specific builds
make dmg          # macOS installer
make windows      # Windows build (cross-compile)
make cross-build  # All platforms via Docker

# See all options
make help
```

For detailed build instructions, see [BUILDING.md](BUILDING.md).

## Controls

- **1-5**: Select ship to place
- **R**: Rotate ship during placement
- **Click**: Place ship or fire at target
- **S**: Save game
- **L**: Load game

## Requirements

- Rust 1.75 or later
- Cargo
- For macOS builds: Xcode Command Line Tools

## License

MIT