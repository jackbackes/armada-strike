# How Armada Strike Works: A Code Walkthrough for Young Programmers

Welcome! This guide will help you understand how Armada Strike is built. Think of this game like a LEGO set - we'll look at each piece and see how they connect together.

## What You'll Need to Know First

- **Variables**: Like boxes that store information (like your score or ship positions)
- **Functions**: Like recipes that tell the computer what to do step-by-step
- **Arrays**: Like a grid of boxes, perfect for our game board!

## The Big Picture 🎮

Armada Strike is built with:
- **Rust**: A programming language (like how we speak English, computers speak Rust)
- **Bevy**: A game engine (handles graphics, sounds, and player input)

## How the Code is Organized

```
src/main.rs         <- The main game code (the brain!)
assets/             <- Images and sounds
Cargo.toml          <- Tells Rust what tools we need
```

## Breaking Down the Game Code

### 1. The Game Board (Lines 10-20)

```rust
const GRID_SIZE: usize = 10;    // Our board is 10x10 squares
const CELL_SIZE: f32 = 30.0;    // Each square is 30 pixels wide
```

Think of this like setting up a chess board, but with 10 rows and 10 columns instead of 8x8.

### 2. What Can Be in Each Square? (Lines 14-20)

```rust
enum CellState {
    Empty,    // Water with nothing there
    Ship,     // Part of a ship
    Hit,      // You hit a ship here!
    Miss,     // You shot but hit water
}
```

An `enum` is like a multiple choice question - each square can only be ONE of these things.

### 3. Types of Ships (Lines 22-29)

```rust
enum ShipType {
    Carrier,    // 5 squares long (biggest ship)
    Battleship, // 4 squares long
    Cruiser,    // 3 squares long
    Submarine,  // 3 squares long
    Destroyer,  // 2 squares long (smallest ship)
}
```

Just like how different LEGO pieces have different sizes, our ships have different lengths!

### 4. The Game's Memory (Lines 155-180)

```rust
struct GameState {
    player_board: [[CellState; GRID_SIZE]; GRID_SIZE],    // Your board
    opponent_board: [[CellState; GRID_SIZE]; GRID_SIZE],  // Enemy board
    selected_x: usize,                                    // Where cursor is (left-right)
    selected_y: usize,                                    // Where cursor is (up-down)
    // ... more stuff the game remembers
}
```

A `struct` is like a backpack that holds all the information the game needs to remember. The `[[CellState; GRID_SIZE]; GRID_SIZE]` creates a 10x10 grid where each spot can be Empty, Ship, Hit, or Miss.

### 5. How Ships Are Placed (Lines 312-326)

```rust
fn place_random_ships(&mut self) {
    let ships = [Carrier, Battleship, Cruiser, Submarine, Destroyer];
    
    for ship_type in ships {
        // Keep trying random positions until we find a good spot
        loop {
            let x = random_number(0..10);  // Pick random column
            let y = random_number(0..10);  // Pick random row
            let horizontal = random_true_or_false();  // Ship goes → or ↓?
            
            if ship_fits_here(x, y, ship_size, horizontal) {
                place_ship_here(x, y, ship_size, horizontal);
                break;  // Stop trying, we found a spot!
            }
        }
    }
}
```

This is like trying to fit puzzle pieces - we keep trying random spots until we find one where the ship fits without overlapping others.

### 6. The Main Game Loop (Lines 379-410)

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)     // Add Bevy's tools
        .init_resource::<GameState>()    // Create our game memory
        .add_systems(Startup, setup)     // Run once when game starts
        .add_systems(Update, (           // Run every frame (60 times/second!)
            handle_input,                // Check if player pressed keys
            handle_mouse_click,          // Check if player clicked
            update_cell_colors,          // Color the squares
            update_status_text,          // Show messages
        ))
        .run();
}
```

This is like the conductor of an orchestra - it makes sure everything happens in the right order, 60 times every second!

### 7. Handling Player Input (Lines 774-840)

```rust
fn handle_input(keyboard: Res<ButtonInput<KeyCode>>, ...) {
    // Move selection with arrow keys
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        selected_y = selected_y - 1;  // Move up
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        selected_y = selected_y + 1;  // Move down
    }
    
    // Press 'S' to save game
    if keyboard.just_pressed(KeyCode::KeyS) {
        save_game();
    }
}
```

This watches what keys you press and tells the game what to do - like a remote control!

### 8. Drawing the Game (Lines 911-1000)

```rust
fn update_cell_colors(game_state: Res<GameState>, ...) {
    for each_square in game_board {
        match square_contents {
            Empty => color_it_blue(),      // Water
            Ship => color_it_gray(),       // Your ship
            Hit => color_it_red(),         // Explosion!
            Miss => color_it_white(),      // Splash
        }
    }
}
```

This is like a painting robot that colors each square based on what's there.

### 9. Saving Your Game (Lines 259-276)

```rust
fn save_to_file(&self) -> Result<String, Error> {
    let save_name = generate_random_name();  // "swift-eagle-strikes"
    let save_data = SaveGame {
        name: save_name,
        player_board: self.player_board,     // Copy your board
        opponent_board: self.opponent_board, // Copy enemy board
        ship_positions: self.ships,          // Remember where ships are
    };
    
    convert_to_json(save_data);  // Turn into text
    write_to_file(json);          // Save to computer
}
```

This is like taking a photo of your game board so you can set it up the same way later.

### 10. Cool Random Names (Lines 352-377)

```rust
fn generate_random_name() -> String {
    let adjectives = ["swift", "mighty", "brave", ...];
    let nouns = ["eagle", "shark", "wolf", ...];
    let verbs = ["strikes", "hunts", "soars", ...];
    
    pick_random(adjective) + "-" + pick_random(noun) + "-" + pick_random(verb)
    // Creates names like: "mighty-shark-hunts" or "swift-eagle-strikes"
}
```

Instead of boring names like "Save1", we create cool three-word names!

## How It All Works Together

1. **Game Starts**: Bevy creates windows and loads sounds
2. **Setup**: We create two 10x10 grids (your board and enemy board)
3. **Every Frame** (60 times per second):
   - Check what keys you pressed
   - Check if you clicked
   - Update what's shown on screen
   - Play sounds if something happened
4. **When You Click**: 
   - If placing ships: Put ship where you clicked
   - If playing: Fire at that square!
5. **Save/Load**: Turn the whole game into text and save it

## Fun Challenges to Try

1. **Change Grid Size**: Find `GRID_SIZE` and change 10 to 15 for a bigger board!
2. **New Ship Type**: Add a "SpeedBoat" that's only 1 square
3. **Change Colors**: Find the color codes (like `0.0, 0.5, 1.0` for blue) and make your own theme
4. **Sound Effects**: The game uses 8-bit retro sounds - try changing the frequencies!

## Key Programming Concepts You've Learned

- **Variables**: Store information
- **Functions**: Group instructions together  
- **Structs**: Bundle related data
- **Enums**: Multiple choice options
- **Arrays**: Grids of data
- **Loops**: Repeat actions
- **Conditionals**: Make decisions (if this, then that)
- **Game Loop**: Update everything constantly

## Next Steps

1. Try changing small things (colors, sizes)
2. Run the game after each change to see what happens
3. If something breaks, use Git to undo (`git checkout .`)
4. Ask questions and experiment!

Remember: Every expert programmer started by changing one small thing at a time. You're on your way!

## Glossary

- **Compile**: Turn your code into something the computer can run
- **Function**: A recipe of instructions
- **Variable**: A box that holds information
- **Array**: A grid of boxes
- **Struct**: A backpack holding many variables
- **Enum**: A multiple choice where only one answer is allowed
- **Loop**: Doing something over and over
- **Frame**: One picture on screen (games show 60 per second!)

Happy coding, future game developer! 🚀