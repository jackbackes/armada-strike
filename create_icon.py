#!/usr/bin/env python3
import numpy as np
from PIL import Image, ImageDraw

def create_battleship_icon():
    """Create a pixel art battleship icon"""
    # Create a 16x16 pixel art base (will be scaled up)
    size = 16
    icon = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    pixels = icon.load()
    
    # Define colors
    ocean_dark = (28, 107, 160)
    ocean_light = (64, 145, 215)
    ship_gray = (128, 128, 128)
    ship_dark = (64, 64, 64)
    hit_red = (255, 0, 0)
    miss_white = (255, 255, 255)
    grid_line = (180, 180, 180)
    
    # Draw ocean background (alternating colors for water effect)
    for y in range(size):
        for x in range(size):
            if (x + y) % 2 == 0:
                pixels[x, y] = (*ocean_dark, 255)
            else:
                pixels[x, y] = (*ocean_light, 255)
    
    # Draw grid lines
    for i in range(0, size, 4):
        for x in range(size):
            if x < size and i < size:
                pixels[x, i] = (*grid_line, 128)
                pixels[i, x] = (*grid_line, 128)
    
    # Draw a battleship (horizontal)
    ship_y = 6
    for x in range(4, 12):
        pixels[x, ship_y] = (*ship_gray, 255)
        pixels[x, ship_y + 1] = (*ship_dark, 255)
    
    # Add ship details (smokestack/turret)
    pixels[7, ship_y - 1] = (*ship_dark, 255)
    pixels[8, ship_y - 1] = (*ship_dark, 255)
    
    # Add some hits and misses for game feel
    pixels[5, 2] = (*hit_red, 255)  # Hit
    pixels[5, 3] = (*hit_red, 200)  # Hit glow
    
    pixels[10, 10] = (*miss_white, 255)  # Miss
    pixels[11, 10] = (*miss_white, 200)  # Miss splash
    pixels[10, 11] = (*miss_white, 200)  # Miss splash
    
    pixels[8, ship_y] = (*hit_red, 255)  # Hit on ship
    
    # Scale up to different sizes with nearest neighbor for pixel art look
    sizes_needed = [16, 32, 64, 128, 256, 512, 1024]
    icons = []
    
    for target_size in sizes_needed:
        scaled = icon.resize((target_size, target_size), Image.NEAREST)
        icons.append(scaled)
    
    return icons

def create_icns_file():
    """Create .icns file for macOS"""
    icons = create_battleship_icon()
    
    # Save as PNG files first
    icon_files = []
    for i, icon in enumerate(icons):
        size = icon.size[0]
        filename = f"icon_{size}x{size}.png"
        icon.save(filename)
        icon_files.append(filename)
    
    # Also save the main icon for other uses
    icons[4].save("battleship_icon.png")  # 256x256 version
    
    print("Icon PNG files created!")
    
    # Create iconset directory structure for macOS
    import os
    import shutil
    
    iconset_dir = "Battleship.iconset"
    if os.path.exists(iconset_dir):
        shutil.rmtree(iconset_dir)
    os.makedirs(iconset_dir)
    
    # Map sizes to iconset naming convention
    size_mapping = {
        16: "16x16",
        32: "16x16@2x",
        32: "32x32", 
        64: "32x32@2x",
        128: "128x128",
        256: "128x128@2x",
        256: "256x256",
        512: "256x256@2x",
        512: "512x512",
        1024: "512x512@2x"
    }
    
    # Copy files with proper naming
    icons[0].save(f"{iconset_dir}/icon_16x16.png")
    icons[1].save(f"{iconset_dir}/icon_16x16@2x.png")
    icons[1].save(f"{iconset_dir}/icon_32x32.png")
    icons[2].save(f"{iconset_dir}/icon_32x32@2x.png")
    icons[3].save(f"{iconset_dir}/icon_128x128.png")
    icons[4].save(f"{iconset_dir}/icon_128x128@2x.png")
    icons[4].save(f"{iconset_dir}/icon_256x256.png")
    icons[5].save(f"{iconset_dir}/icon_256x256@2x.png")
    icons[5].save(f"{iconset_dir}/icon_512x512.png")
    icons[6].save(f"{iconset_dir}/icon_512x512@2x.png")
    
    print(f"Iconset directory '{iconset_dir}' created!")
    print("Run 'iconutil -c icns Battleship.iconset' to create the .icns file")
    
    # Clean up temporary files
    for f in icon_files:
        if os.path.exists(f):
            os.remove(f)

if __name__ == "__main__":
    create_icns_file()