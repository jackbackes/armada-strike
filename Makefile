.PHONY: all setup assets build release dmg clean run help windows test-cross

# Default target
all: release

# Setup Python environment for asset generation
setup:
	@echo "Setting up Python environment..."
	@./setup_asset_env.sh

# Generate all assets (icons, sounds)
assets: setup
	@echo "Generating assets..."
	@source venv/bin/activate && \
		python3 create_icon.py && \
		([ -d "ArmadaStrike.iconset" ] && iconutil -c icns ArmadaStrike.iconset || true) && \
		([ -f "generate_sounds.py" ] && [ ! -d "assets/sounds" ] && python3 generate_sounds.py || true) && \
		deactivate
	@echo "✅ Assets generated"

# Build debug version
build:
	@echo "Building debug version..."
	@cargo build
	@echo "✅ Debug build complete"

# Build release version with assets
release: assets
	@echo "Building release version..."
	@cargo build --release
	@echo "✅ Release build complete"

# Create DMG installer
dmg: release
	@echo "Creating DMG installer..."
	@./create_dmg.sh
	@echo "✅ DMG created"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf ArmadaStrike.app ArmadaStrike-Installer.dmg
	@rm -rf ArmadaStrike.iconset armada_strike_icon.png ArmadaStrike.icns
	@rm -rf venv __pycache__
	@echo "✅ Clean complete"

# Run the game (debug mode)
run:
	@cargo run

# Run the game (release mode)
run-release: release
	@if [ -f "target/aarch64-apple-darwin/release/armada-strike" ]; then \
		target/aarch64-apple-darwin/release/armada-strike; \
	elif [ -f "target/x86_64-apple-darwin/release/armada-strike" ]; then \
		target/x86_64-apple-darwin/release/armada-strike; \
	elif [ -f "target/release/armada-strike" ]; then \
		target/release/armada-strike; \
	else \
		echo "Release binary not found. Run 'make release' first."; \
	fi

# Build for Windows (cross-compilation)
windows:
	@echo "Building for Windows..."
	@chmod +x build_windows.sh
	@./build_windows.sh
	@echo "✅ Windows build complete (if successful)"

# Build for Windows 64-bit only
windows-x64:
	@echo "Building for Windows x64..."
	@chmod +x build_windows.sh
	@./build_windows.sh --x86_64
	@echo "✅ Windows x64 build complete (if successful)"

# Build for Windows 32-bit only
windows-x86:
	@echo "Building for Windows x86..."
	@chmod +x build_windows.sh
	@./build_windows.sh --i686
	@echo "✅ Windows x86 build complete (if successful)"

# Test cross-compilation capabilities
test-cross:
	@echo "Testing cross-compilation capabilities..."
	@chmod +x test_cross_compile.sh
	@./test_cross_compile.sh
	@echo "✅ Cross-compilation test complete"

# Cross-platform build using Docker
cross-build:
	@echo "Building cross-platform releases with Docker..."
	@chmod +x build_cross_platform.sh
	@./build_cross_platform.sh
	@echo "✅ Cross-platform builds complete (check dist/ folder)"

# Build all platforms
build-all: release dmg cross-build
	@echo "✅ All platform builds complete"

# Show help
help:
	@echo "Armada Strike Build System"
	@echo "======================="
	@echo ""
	@echo "Available targets:"
	@echo "  make setup      - Setup Python environment for asset generation"
	@echo "  make assets     - Generate icons and sound effects"
	@echo "  make build      - Build debug version"
	@echo "  make release    - Build release version with all assets"
	@echo "  make dmg        - Create macOS DMG installer (includes release build)"
	@echo "  make clean      - Clean all build artifacts"
	@echo "  make run        - Run debug version"
	@echo "  make run-release - Run release version"
	@echo ""
	@echo "Cross-platform targets:"
	@echo "  make windows    - Build for Windows (requires MinGW-w64)"
	@echo "  make windows-x64 - Build for Windows 64-bit only"
	@echo "  make windows-x86 - Build for Windows 32-bit only"
	@echo "  make cross-build - Build using Docker (Windows + Linux)"
	@echo "  make build-all  - Build for all platforms"
	@echo "  make test-cross - Test cross-compilation capabilities"
	@echo ""
	@echo "  make help       - Show this help message"
	@echo ""
	@echo "Quick start:"
	@echo "  make dmg        - Build everything and create macOS installer"
	@echo "  make windows    - Build for Windows (requires MinGW-w64)"