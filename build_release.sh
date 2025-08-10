#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}    Battleship Release Build Script     ${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Step 1: Setup Python environment
echo -e "${YELLOW}Step 1: Setting up Python environment...${NC}"
if [ ! -d "venv" ]; then
    python3 -m venv venv
    source venv/bin/activate
    pip install --upgrade pip --quiet
    pip install Pillow numpy --quiet
    echo -e "${GREEN}✅ Python environment created${NC}"
else
    source venv/bin/activate
    echo -e "${GREEN}✅ Python environment activated${NC}"
fi

# Step 2: Generate assets
echo -e "${YELLOW}Step 2: Generating assets...${NC}"

# Generate icon
if [ -f "create_icon.py" ]; then
    echo "  Generating app icon..."
    python3 create_icon.py
    if [ -f "battleship_icon.png" ]; then
        echo -e "${GREEN}  ✅ Icon generated${NC}"
    else
        echo -e "${RED}  ⚠️  Icon generation failed${NC}"
    fi
fi

# Generate sound effects if needed
if [ -f "generate_sounds.py" ] && [ ! -d "assets/sounds" ]; then
    echo "  Generating sound effects..."
    python3 generate_sounds.py
    echo -e "${GREEN}  ✅ Sound effects generated${NC}"
fi

# Deactivate Python environment
deactivate

# Step 3: Build release binary
echo -e "${YELLOW}Step 3: Building release binary...${NC}"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Release build successful${NC}"
else
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

# Step 4: Create DMG installer (optional)
echo ""
echo -e "${YELLOW}Step 4: Create DMG installer?${NC}"
read -p "Do you want to create a DMG installer? (y/n) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ -f "create_dmg.sh" ]; then
        ./create_dmg.sh
    else
        echo -e "${RED}create_dmg.sh not found${NC}"
    fi
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}         Build Complete!                ${NC}"
echo -e "${GREEN}========================================${NC}"

# Find the binary and report its location
if [ -f "target/aarch64-apple-darwin/release/battleship" ]; then
    BINARY_PATH="target/aarch64-apple-darwin/release/battleship"
elif [ -f "target/x86_64-apple-darwin/release/battleship" ]; then
    BINARY_PATH="target/x86_64-apple-darwin/release/battleship"
elif [ -f "target/release/battleship" ]; then
    BINARY_PATH="target/release/battleship"
fi

if [ -n "$BINARY_PATH" ]; then
    BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
    echo -e "${GREEN}Binary: $BINARY_PATH (${BINARY_SIZE})${NC}"
fi

if [ -f "Battleship-Installer.dmg" ]; then
    DMG_SIZE=$(du -h "Battleship-Installer.dmg" | cut -f1)
    echo -e "${GREEN}DMG: Battleship-Installer.dmg (${DMG_SIZE})${NC}"
fi

echo ""
echo "To run the game:"
echo "  $BINARY_PATH"
echo ""
echo "To distribute:"
echo "  Share the Battleship-Installer.dmg file"