# Lesson 2: The Game Board - A Code Walkthrough for Logan

Welcome back, Logan! Now that you have Rust installed and the game running, let's understand how the computer keeps track of the game board.

## How Computers Think About Grids

Remember playing tic-tac-toe? You have a 3x3 grid. Armada Strike is similar, but with a 10x10 grid. The computer needs a way to remember what's in each square.

In real life, you might say "put my ship at B4" (like in the board game). Computers prefer numbers, so they think of it as "put my ship at position (1, 3)" where the first number is the column and second is the row.

## Arrays: The Computer's Grid Paper

Look at line 156-157 in `main.rs`:

```rust
struct GameState {
    player_board: [[CellState; GRID_SIZE]; GRID_SIZE],
```

This creates a grid! Let's break it down:
- `[[` means "array of arrays" (a grid!)
- `CellState` is what can be in each square
- `GRID_SIZE` is 10 (we defined this earlier)
- So this creates a 10x10 grid where each spot holds a `CellState`

Think of it like this:
```
[
  [⬜,⬜,⬜,⬜,⬜,⬜,⬜,⬜,⬜,⬜],  <- Row 0
  [⬜,⬜,⬜,⬜,⬜,⬜,⬜,⬜,⬜,⬜],  <- Row 1
  [⬜,⬜,🚢,🚢,🚢,⬜,⬜,⬜,⬜,⬜],  <- Row 2 (ship here!)
  [⬜,⬜,⬜,⬜,⬜,⬜,⬜,⬜,⬜,⬜],  <- Row 3
  ... and so on
]
```

## What Can Be in Each Square?

Look at lines 14-20:

```rust
enum CellState {
    Empty,   // Just water
    Ship,    // Part of a ship is here
    Hit,     // You hit a ship here (💥)
    Miss,    // You shot here but hit water (💦)
}
```

An `enum` (short for enumeration) is like a multiple choice question where only ONE answer can be picked. Each square MUST be either Empty, Ship, Hit, or Miss - it can't be two things at once!

## The Five Ships

Lines 22-29 define our ships:

```rust
enum ShipType {
    Carrier,    // 5 squares long (biggest)
    Battleship, // 4 squares long
    Cruiser,    // 3 squares long
    Submarine,  // 3 squares long  
    Destroyer,  // 2 squares long (smallest)
}
```

And lines 31-40 tell us how long each ship is:

```rust
impl ShipType {
    fn size(&self) -> usize {
        match self {
            ShipType::Carrier => 5,
            ShipType::Battleship => 4,
            ShipType::Cruiser => 3,
            ShipType::Submarine => 3,
            ShipType::Destroyer => 2,
        }
    }
}
```

The `match` statement is like a switchboard operator - "if it's a Carrier, return 5; if it's a Battleship, return 4..." and so on.

## How Ships Remember Their Position

Look at lines 96-103:

```rust
struct PlacedShip {
    ship_type: ShipType,    // What kind of ship
    x: usize,               // Column position (0-9)
    y: usize,               // Row position (0-9)
    is_horizontal: bool,    // true = → | false = ↓
}
```

This is like a note card for each ship that says:
- What type of ship it is
- Where it starts (x, y)
- Which direction it goes

Example: A Carrier at position (2, 3) going horizontal would occupy squares:
- (2, 3), (3, 3), (4, 3), (5, 3), (6, 3)

## Checking if a Ship Fits

Look at the function around line 313 (simplified):

```rust
fn can_place_ship(board: &[[CellState; 10]; 10], x: usize, y: usize, 
                  size: usize, is_horizontal: bool) -> bool {
    if is_horizontal {
        // Check if ship goes off right edge
        if x + size > 10 { return false; }
        
        // Check each square the ship would occupy
        for i in 0..size {
            if board[y][x + i] != CellState::Empty {
                return false;  // Something's already there!
            }
        }
    } else {
        // Same thing but checking downward
        if y + size > 10 { return false; }
        
        for i in 0..size {
            if board[y + i][x] != CellState::Empty {
                return false;
            }
        }
    }
    true  // Ship fits!
}
```

This function is like a puzzle piece checker - before placing a ship, it makes sure:
1. The ship won't go off the board
2. All squares where it would go are empty

## Finding What Ship is at a Position

Lines 183-196 show how to find which ship is at any square:

```rust
fn get_ship_at(&self, x: usize, y: usize) -> Option<ShipType> {
    for ship in &self.ship_positions {
        if ship.is_horizontal {
            if y == ship.y && x >= ship.x && x < ship.x + ship.ship_type.size() {
                return Some(ship.ship_type);
            }
        } else {
            if x == ship.x && y >= ship.y && y < ship.y + ship.ship_type.size() {
                return Some(ship.ship_type);
            }
        }
    }
    None  // No ship at this position
}
```

This is like asking "what's at square (5, 3)?" The function checks each ship to see if that square is part of it.

## The Magic of `Option`

Notice the function returns `Option<ShipType>`. In Rust, `Option` means "maybe something, maybe nothing":
- `Some(ShipType::Carrier)` means "Yes, there's a Carrier here!"
- `None` means "Nope, no ship here"

This is Rust being extra safe - it forces you to handle both cases (found a ship or didn't).

## Coordinate System

The game uses (x, y) coordinates where:
- x = column (0 is leftmost, 9 is rightmost)
- y = row (0 is top, 9 is bottom)

```
   0 1 2 3 4 5 6 7 8 9  <- x coordinates
0  ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
1  ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
2  ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
↑
y coordinates
```

## What You've Learned

✅ How computers store grids using arrays  
✅ What `enum` means (multiple choice with one answer)  
✅ How ships remember their position  
✅ How the game checks if ships fit  
✅ What `Option` means (maybe something, maybe nothing)  
✅ The coordinate system (x, y)  

## Homework Challenge

1. Find the `ShipType` enum in the code
2. Add a new ship type called `PatrolBoat` that's 1 square long
3. Add it to the `size()` function to return 1
4. Add it to the `name()` function to return "Patrol Boat (1)"
5. Run the game - you won't see it yet (we need to add it to the ship list) but it should still compile!

## Debugging Tips

If you get errors:
- Check that every `{` has a matching `}`
- Make sure you have commas between enum items
- Remember the arrow `=>` in match statements
- Save the file before running `cargo run`

## What's Next?

In Lesson 3, we'll learn how the game handles your mouse clicks and keyboard presses. You'll understand how pressing 'S' saves your game and how clicking places ships!

---

**Fun Fact:** Real game developers often spend more time planning their data structures (like our grid and ship positions) than writing the actual game logic. Good organization makes everything else easier!

See you in Lesson 3! 🎮