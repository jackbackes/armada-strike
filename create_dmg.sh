#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Creating Battleship.app bundle and DMG installer...${NC}"

# Variables
APP_NAME="Battleship"
APP_BUNDLE="$APP_NAME.app"
DMG_NAME="Battleship-Installer.dmg"
VOLUME_NAME="Battleship Installer"
BUILD_DIR="dmg_build"

# Clean up any existing build
echo "Cleaning up old builds..."
rm -rf "$APP_BUNDLE" "$BUILD_DIR" "$DMG_NAME"

# Create app bundle structure
echo "Creating app bundle structure..."
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"

# Copy the binary
echo "Copying binary..."
# Check for platform-specific release binary first, then generic location
if [ -f "target/aarch64-apple-darwin/release/battleship" ]; then
    cp "target/aarch64-apple-darwin/release/battleship" "$APP_BUNDLE/Contents/MacOS/$APP_NAME"
    chmod +x "$APP_BUNDLE/Contents/MacOS/$APP_NAME"
elif [ -f "target/x86_64-apple-darwin/release/battleship" ]; then
    cp "target/x86_64-apple-darwin/release/battleship" "$APP_BUNDLE/Contents/MacOS/$APP_NAME"
    chmod +x "$APP_BUNDLE/Contents/MacOS/$APP_NAME"
elif [ -f "target/release/battleship" ]; then
    cp "target/release/battleship" "$APP_BUNDLE/Contents/MacOS/$APP_NAME"
    chmod +x "$APP_BUNDLE/Contents/MacOS/$APP_NAME"
else
    echo -e "${RED}Error: Release binary not found${NC}"
    echo "Please run 'cargo build --release' first"
    echo "Searched in:"
    echo "  - target/aarch64-apple-darwin/release/battleship"
    echo "  - target/x86_64-apple-darwin/release/battleship"
    echo "  - target/release/battleship"
    exit 1
fi

# Copy assets
echo "Copying game assets..."
if [ -d "assets" ]; then
    cp -r "assets" "$APP_BUNDLE/Contents/Resources/"
fi

# Create Info.plist
echo "Creating Info.plist..."
cat > "$APP_BUNDLE/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>$APP_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>com.battleship.game</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
</dict>
</plist>
EOF

# Create an icon if the Python script exists and hasn't been run
if [ -f "create_icon.py" ] && [ ! -f "Battleship.icns" ]; then
    echo "Generating icon..."
    if [ -d "venv" ]; then
        source venv/bin/activate
        python3 create_icon.py 2>/dev/null
        deactivate
        # Create icns from iconset if it was generated
        if [ -d "Battleship.iconset" ]; then
            iconutil -c icns Battleship.iconset 2>/dev/null
        fi
    else
        echo -e "${YELLOW}Warning: Python venv not found. Run ./setup_asset_env.sh first${NC}"
    fi
fi

# If we have an icns file, use it; otherwise try png
if [ -f "Battleship.icns" ]; then
    cp "Battleship.icns" "$APP_BUNDLE/Contents/Resources/Battleship.icns"
    # Update Info.plist to reference the icon after it's copied
    /usr/libexec/PlistBuddy -c "Add :CFBundleIconFile string Battleship" "$APP_BUNDLE/Contents/Info.plist" 2>/dev/null || \
    /usr/libexec/PlistBuddy -c "Set :CFBundleIconFile Battleship" "$APP_BUNDLE/Contents/Info.plist"
elif [ -f "battleship_icon.png" ]; then
    cp "battleship_icon.png" "$APP_BUNDLE/Contents/Resources/icon.png"
fi

# Create the DMG
echo "Creating DMG installer..."
mkdir -p "$BUILD_DIR"
cp -r "$APP_BUNDLE" "$BUILD_DIR/"

# Create a symbolic link to Applications
ln -s /Applications "$BUILD_DIR/Applications"

# Create the DMG with better settings
hdiutil create -volname "$VOLUME_NAME" \
    -srcfolder "$BUILD_DIR" \
    -ov \
    -format UDZO \
    -imagekey zlib-level=9 \
    "$DMG_NAME"

# Clean up
echo "Cleaning up temporary files..."
rm -rf "$BUILD_DIR"

# Verify the DMG was created
if [ -f "$DMG_NAME" ]; then
    DMG_SIZE=$(du -h "$DMG_NAME" | cut -f1)
    echo -e "${GREEN}✅ Successfully created $DMG_NAME (Size: $DMG_SIZE)${NC}"
    echo -e "${GREEN}The installer is ready for distribution!${NC}"
    echo ""
    echo "To install:"
    echo "1. Double-click $DMG_NAME"
    echo "2. Drag Battleship.app to the Applications folder"
    echo "3. Eject the installer"
    echo "4. Launch Battleship from Applications"
else
    echo -e "${RED}❌ Failed to create DMG${NC}"
    exit 1
fi