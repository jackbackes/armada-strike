# Lesson 3: Input and Interaction - A Code Walkthrough for Logan

Hey Logan! Today we're learning how the game knows when you click or press keys. This is where the magic happens - turning your actions into game moves!

## The Game Loop Concept

First, understand that games work differently than most programs. Instead of waiting for you to do something, games run in a **loop** that happens 60 times per second!

Each loop:
1. Check what keys are pressed
2. Check mouse position and clicks
3. Update the game based on input
4. Draw everything on screen
5. Repeat!

This happens so fast it looks smooth to our eyes.

## How Bevy Handles Input

Bevy (our game engine) gives us special tools to detect input. Look at line 774:

```rust
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    // ... other parameters
)
```

Breaking this down:
- `keyboard: Res<ButtonInput<KeyCode>>` - Bevy gives us a keyboard watcher
- `mut game_state` - We can change (`mut` = mutable) the game state
- This function runs 60 times per second!

## Detecting Key Presses

Look at lines 778-790 (simplified):

```rust
// Move selection with arrow keys
if keyboard.just_pressed(KeyCode::ArrowUp) && game_state.selected_y > 0 {
    game_state.selected_y -= 1;  // Move up one square
}
if keyboard.just_pressed(KeyCode::ArrowDown) && game_state.selected_y < GRID_SIZE - 1 {
    game_state.selected_y += 1;  // Move down one square
}
if keyboard.just_pressed(KeyCode::ArrowLeft) && game_state.selected_x > 0 {
    game_state.selected_x -= 1;  // Move left one square
}
if keyboard.just_pressed(KeyCode::ArrowRight) && game_state.selected_x < GRID_SIZE - 1 {
    game_state.selected_x += 1;  // Move right one square
}
```

Important: `just_pressed` vs `pressed`:
- `just_pressed` = true only on the frame you push the key down
- `pressed` = true the entire time you hold the key

We use `just_pressed` so it moves one square per tap, not 60 squares per second!

## The Space Bar - Fire!

Lines 792-804 handle shooting:

```rust
if keyboard.just_pressed(KeyCode::Space) {
    let x = game_state.selected_x;
    let y = game_state.selected_y;
    
    if !game_state.is_player_board {  // Only shoot at opponent's board
        if game_state.opponent_board[y][x] == CellState::Empty {
            game_state.opponent_board[y][x] = CellState::Miss;
            // Play miss sound
        } else if game_state.opponent_board[y][x] == CellState::Ship {
            game_state.opponent_board[y][x] = CellState::Hit;
            // Play hit sound
        }
    }
}
```

The game:
1. Gets current cursor position
2. Checks you're aiming at opponent's board
3. If empty water → mark as Miss
4. If there's a ship → mark as Hit
5. Plays appropriate sound

## Ship Placement Keys

Lines 782-789 handle ship placement:

```rust
// Press 1 to place Carrier
if keyboard.just_pressed(KeyCode::Digit1) && !game_state.ships_placed.contains(&ShipType::Carrier) {
    game_state.placement_mode = PlacementMode::PlacingShip(ShipType::Carrier, true);
}

// Press 2 to place Battleship
if keyboard.just_pressed(KeyCode::Digit2) && !game_state.ships_placed.contains(&ShipType::Battleship) {
    game_state.placement_mode = PlacementMode::PlacingShip(ShipType::Battleship, true);
}
// ... and so on for keys 3, 4, 5
```

The `&&` means AND - both conditions must be true:
1. You pressed the number key
2. AND that ship hasn't been placed yet

## Rotating Ships

Line 806:

```rust
if keyboard.just_pressed(KeyCode::KeyR) {
    if let PlacementMode::PlacingShip(ship, is_horizontal) = game_state.placement_mode {
        game_state.placement_mode = PlacementMode::PlacingShip(ship, !is_horizontal);
    }
}
```

The `!` means NOT - it flips true to false or false to true. So `!is_horizontal` toggles between horizontal and vertical.

## Mouse Clicks

Look at the mouse handling function around line 850:

```rust
fn handle_mouse_click(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut game_state: ResMut<GameState>,
)
```

This is more complex because we need to:
1. Get mouse position on screen
2. Convert screen position to world position
3. Figure out which grid square that is

The conversion looks like:

```rust
if mouse.just_pressed(MouseButton::Left) {
    if let Some(position) = window.single().cursor_position() {
        // Convert mouse position to world coordinates
        let (camera, transform) = camera.single();
        if let Some(world_pos) = camera.viewport_to_world_2d(transform, position) {
            // Calculate which grid cell
            let x = ((world_pos.x + offset) / CELL_SIZE) as usize;
            let y = ((world_pos.y + offset) / CELL_SIZE) as usize;
            
            // Now we know which square was clicked!
        }
    }
}
```

Think of it like this:
- Mouse says "I'm at pixel (400, 300) on screen"
- We calculate "that's grid square (5, 4)"

## Saving and Loading

Lines 817-832 handle save/load:

```rust
// Save game
if keyboard.just_pressed(KeyCode::KeyS) && keyboard.pressed(KeyCode::ControlLeft) {
    match game_state.save_to_file(None) {
        Ok(name) => println!("Game saved as: {}", name),
        Err(e) => println!("Failed to save: {}", e),
    }
}

// Load game
if keyboard.just_pressed(KeyCode::KeyL) && keyboard.pressed(KeyCode::ControlLeft) {
    if let Some(save_name) = &settings.current_save {
        match game_state.load_from_file(save_name) {
            Ok(()) => println!("Game loaded!"),
            Err(e) => println!("Failed to load: {}", e),
        }
    }
}
```

Notice it checks for Ctrl+S and Ctrl+L (both keys at once). The `match` statement handles both success (`Ok`) and failure (`Err`) cases.

## Input State Machine

The game uses a "state machine" to know what input means:

```rust
enum PlacementMode {
    Playing,                        // Normal gameplay
    PlacingShip(ShipType, bool),   // Placing a specific ship
}
```

Based on the mode:
- In `Playing` mode: clicks fire at squares
- In `PlacingShip` mode: clicks place ships

This is like having different "modes" in a drawing program - sometimes clicking draws, sometimes it erases, depending on which tool you selected.

## The Tab Key - Switch Boards

Line 812:

```rust
if keyboard.just_pressed(KeyCode::Tab) {
    game_state.is_player_board = !game_state.is_player_board;
}
```

This toggles between viewing your board and the opponent's board. Simple but essential!

## What You've Learned

✅ How the game loop runs 60 times per second  
✅ The difference between `just_pressed` and `pressed`  
✅ How keyboard input is detected  
✅ How mouse clicks are converted to grid positions  
✅ What state machines are (different modes)  
✅ How multiple keys can be required (Ctrl+S)  

## Homework Challenge

1. Find the `handle_input` function
2. Add a new key binding: make the 'C' key clear your entire board
3. Hint: You'll need:
   ```rust
   if keyboard.just_pressed(KeyCode::KeyC) {
       game_state.clear_board();
   }
   ```
4. The `clear_board()` function already exists - find it!
5. Test it - does pressing C clear your ships?

## Extra Challenge

Try adding a "quick save" feature:
- Press F5 to save without naming
- Press F9 to load the last save

Hint: Look at how Ctrl+S works and simplify it!

## Debugging Input

Common problems:
- Forgetting `just_pressed` (using `pressed` makes things happen 60 times!)
- Wrong KeyCode name (it's `KeyCode::Space`, not `KeyCode::Spacebar`)
- Forgetting to check bounds (moving off the grid)

## What's Next?

In Lesson 4, we'll explore how the game draws everything on screen - from the blue water to the red explosions. You'll learn about sprites, colors, and visual effects!

---

**Pro Tip:** Most games spend 80% of their code handling input and UI, and only 20% on actual game logic. Good controls make games fun!

See you in Lesson 4! 🕹️