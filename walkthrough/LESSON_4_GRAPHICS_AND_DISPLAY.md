# Lesson 4: Graphics and Display - A Code Walkthrough for Logan

Welcome back, Logan! Today we're diving into how the game creates everything you see on screen - from the ocean grid to the explosions when you hit a ship!

## How Games Draw Things

Remember flip books? Games work similarly - they draw pictures really fast (60 per second) to create smooth motion. Each picture is called a **frame**.

In Armada Strike, every frame we:
1. Clear the screen
2. Draw the ocean/grid
3. Draw ships (on your board)
4. Draw hits and misses
5. Draw the cursor
6. Show it all at once

## Bevy's Entity Component System (ECS)

Bevy uses something called ECS. Think of it like this:
- **Entity**: A thing in the game (a grid square, text, sound)
- **Component**: A property of that thing (position, color, "is this a cell?")
- **System**: A function that acts on entities with certain components

It's like labeling everything with sticky notes, then having rules about what to do with things that have certain labels.

## Creating the Game Board

Look at the `setup` function around line 425:

```rust
fn setup(mut commands: Commands) {
    // Create camera (the "eye" that sees the game)
    commands.spawn(Camera2dBundle::default());
    
    // Create player board
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let position = Vec3::new(
                x as f32 * (CELL_SIZE + CELL_SPACING) - 150.0,
                y as f32 * (CELL_SIZE + CELL_SPACING) - 150.0,
                0.0
            );
            
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(position),
                    sprite: Sprite {
                        color: Color::rgb(0.1, 0.3, 0.5),  // Ocean blue
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..default()
                    },
                    ..default()
                },
                Cell { x, y, is_player_board: true },  // Mark as a cell
            ));
        }
    }
}
```

Breaking this down:
1. We loop through each grid position (0-9 for x, 0-9 for y)
2. Calculate where on screen it should be
3. Create a square sprite (image) at that position
4. Color it ocean blue
5. Tag it as a `Cell` so we can find it later

## Colors in RGB

The game uses RGB (Red, Green, Blue) colors. Each value goes from 0.0 to 1.0:

```rust
Color::rgb(0.1, 0.3, 0.5)  // Ocean blue
```

- Red: 0.1 (very little red)
- Green: 0.3 (some green)
- Blue: 0.5 (half blue)

Think of it like mixing paint:
- `(1.0, 0.0, 0.0)` = Pure red
- `(0.0, 1.0, 0.0)` = Pure green
- `(0.0, 0.0, 1.0)` = Pure blue
- `(1.0, 1.0, 1.0)` = White (all colors)
- `(0.0, 0.0, 0.0)` = Black (no colors)

## Updating Cell Colors

The `update_cell_colors` function (around line 911) runs every frame:

```rust
fn update_cell_colors(
    game_state: Res<GameState>,
    mut query: Query<(&Cell, &mut Sprite)>,
) {
    for (cell, mut sprite) in query.iter_mut() {
        let board = if cell.is_player_board {
            &game_state.player_board
        } else {
            &game_state.opponent_board
        };
        
        sprite.color = match board[cell.y][cell.x] {
            CellState::Empty => Color::rgb(0.1, 0.3, 0.5),      // Ocean blue
            CellState::Ship => Color::rgb(0.5, 0.5, 0.5),       // Gray ship
            CellState::Hit => Color::rgb(0.8, 0.2, 0.2),        // Red explosion
            CellState::Miss => Color::rgb(0.7, 0.7, 0.9),       // Light blue splash
        };
    }
}
```

This function:
1. Looks at every cell
2. Checks what's in that grid position
3. Changes the color based on what's there

It's like having a painter who repaints every square every frame based on what should be there!

## The Selection Cursor

Lines 957-975 show how we highlight the selected square:

```rust
fn update_selection_indicator(
    game_state: Res<GameState>,
    mut query: Query<(&Cell, &mut Sprite)>,
) {
    for (cell, mut sprite) in query.iter_mut() {
        if cell.x == game_state.selected_x && 
           cell.y == game_state.selected_y &&
           cell.is_player_board == game_state.is_player_board {
            // Make it brighter
            sprite.color = sprite.color.with_a(1.0);
            // Could also add a border or make it pulse!
        } else {
            // Normal opacity
            sprite.color = sprite.color.with_a(0.9);
        }
    }
}
```

The `.with_a()` changes the alpha (transparency):
- 1.0 = fully visible
- 0.5 = half transparent
- 0.0 = invisible

## Ship Preview While Placing

When you're placing a ship, it shows a preview (lines 990-1010):

```rust
fn show_ship_preview(
    game_state: Res<GameState>,
    mut query: Query<(&Cell, &mut Sprite)>,
) {
    if let PlacementMode::PlacingShip(ship_type, is_horizontal) = game_state.placement_mode {
        let size = ship_type.size();
        
        for (cell, mut sprite) in query.iter_mut() {
            if would_ship_cover_this_cell(cell, ship_position, size, is_horizontal) {
                // Show preview in green if valid, red if invalid
                if can_place_ship_here() {
                    sprite.color = Color::rgb(0.2, 0.7, 0.2);  // Green
                } else {
                    sprite.color = Color::rgb(0.7, 0.2, 0.2);  // Red
                }
            }
        }
    }
}
```

This gives immediate feedback - green means "you can place here", red means "you can't".

## Text Display

The game shows text for status and information (lines 1068-1090):

```rust
commands.spawn((
    Text2dBundle {
        text: Text::from_section(
            "Arrows: Move | Space: Fire | 1-5: Place Ships",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        transform: Transform::from_xyz(0.0, 320.0, 1.0),
        ..default()
    },
    StatusText,  // Tag so we can find and update it later
));
```

The `Transform::from_xyz(0.0, 320.0, 1.0)` sets position:
- x: 0.0 (centered horizontally)
- y: 320.0 (near top of screen)
- z: 1.0 (in front of other things)

## Visual Effects

The game has special effects for hits and misses (lines 79-86):

```rust
#[derive(Component)]
struct HitEffect {
    timer: Timer,
}

#[derive(Component)]
struct MissEffect {
    timer: Timer,
}
```

When you hit or miss, it creates an expanding circle effect:

```rust
// Create explosion effect
commands.spawn((
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(1.0, 0.3, 0.0, 0.8),  // Orange explosion
            custom_size: Some(Vec2::new(10.0, 10.0)),  // Starts small
            ..default()
        },
        transform: Transform::from_xyz(x, y, 2.0),  // Above other sprites
        ..default()
    },
    HitEffect {
        timer: Timer::from_seconds(0.5, TimerMode::Once),
    },
));
```

Then every frame, the effect updates:

```rust
fn update_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut effects: Query<(Entity, &mut HitEffect, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut effect, mut transform, mut sprite) in effects.iter_mut() {
        effect.timer.tick(time.delta());
        
        // Make it grow
        let scale = 1.0 + effect.timer.fraction() * 2.0;
        transform.scale = Vec3::splat(scale);
        
        // Make it fade
        sprite.color.set_a(1.0 - effect.timer.fraction());
        
        // Remove when done
        if effect.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

This creates an expanding, fading explosion - game magic!

## Coordinate Systems

The game uses Cartesian coordinates with (0, 0) at the center:
```
        ^ +y
        |
<-------+-------> +x
   -x   |
        v -y
```

That's why we subtract 150 when positioning - to center the grid.

## Layers (Z-Order)

Notice the `z` values in positions:
- 0.0 = Background (ocean)
- 1.0 = Normal sprites (ships)
- 2.0 = Effects (explosions)
- 3.0 = UI text

Higher numbers appear on top of lower numbers.

## What You've Learned

✅ How games draw 60 frames per second  
✅ What RGB colors are and how to mix them  
✅ How Entity Component System works  
✅ How sprites are positioned on screen  
✅ How to create visual effects  
✅ What alpha/transparency does  
✅ How z-order creates layers  

## Homework Challenge

1. Find the ocean blue color `Color::rgb(0.1, 0.3, 0.5)`
2. Change it to make the ocean different:
   - Purple ocean: `Color::rgb(0.3, 0.1, 0.5)`
   - Green ocean: `Color::rgb(0.1, 0.5, 0.3)`
   - Dark ocean: `Color::rgb(0.05, 0.1, 0.2)`
3. Find the ship gray color and make ships a different color
4. Run the game - does it look cool?

## Extra Challenge

Try making the selected square pulse:
1. Add a timer to track pulsing
2. Use `sin(time)` to make the brightness go up and down
3. Apply it to the selected cell

Hint: Look at how `HitEffect` uses a timer!

## What's Next?

In Lesson 5, we'll explore the sound system - how the game plays those retro 8-bit sounds when you hit or miss!

---

**Fun Fact:** Modern games can have millions of triangles on screen, but retro-style games like ours use simple colored squares (sprites) for that classic look!

See you in Lesson 5! 🎨