#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Setting up Python virtual environment for asset generation...${NC}"

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv venv
else
    echo "Virtual environment already exists"
fi

# Activate virtual environment
source venv/bin/activate

# Upgrade pip
echo "Upgrading pip..."
pip install --upgrade pip --quiet

# Install required packages
echo "Installing required packages..."
pip install Pillow numpy --quiet

echo -e "${GREEN}✅ Python environment ready!${NC}"
echo ""
echo "To use the environment manually:"
echo "  source venv/bin/activate"
echo "  python3 create_icon.py"
echo "  deactivate"