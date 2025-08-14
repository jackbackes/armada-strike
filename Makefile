.PHONY: all setup assets build release dmg clean run help

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
	@echo "  make help       - Show this help message"
	@echo ""
	@echo "Quick start:"
	@echo "  make dmg        - Build everything and create installer"