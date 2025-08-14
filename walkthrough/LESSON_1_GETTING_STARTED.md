# Lesson 1: Getting Started - A Code Walkthrough for Logan

Hey Logan! Welcome to your journey into understanding how Armada Strike works. We're going to explore this game's code together, and by the end, you'll understand how a real game is built!

## Why This Game Uses Rust

You might be wondering - why does this code look different from Python or JavaScript you might have seen? This game is written in **Rust**, a programming language that's like a super-powered race car for computers.

### Why Rust Instead of Other Languages?

Think of programming languages like different types of vehicles:
- **Python** is like a friendly bicycle - easy to learn, great for beginners
- **JavaScript** is like a scooter - fast to start, runs in web browsers
- **Rust** is like a Formula 1 race car - incredibly fast, super safe, perfect for games!

Games need to:
- Run REALLY fast (60 updates per second!)
- Never crash or freeze
- Handle lots of things at once (graphics, sound, input)

Rust is perfect for this because it's one of the fastest languages AND it prevents crashes before they happen!

## Installing Rust - Your Development Toolkit

Let's get Rust on your computer. Think of this like installing Minecraft before you can play it.

### Step 1: Install Rust

1. Go to [rustup.rs](https://rustup.rs) in your web browser
2. Click the big download button
3. Open the downloaded file and follow the installer

**On Mac** (if that's what you have):
- Open Terminal (find it in Applications > Utilities)
- The installer might ask you to copy and paste a command - that's normal!

### Step 2: Verify It Worked

Open Terminal (Mac) or Command Prompt (Windows) and type:
```bash
rustc --version
```

You should see something like: `rustc 1.75.0`

If you see this, congratulations! You've installed Rust! 🎉

### Step 3: Install a Code Editor

You need a special program to write and read code comfortably. We recommend **Visual Studio Code** (VS Code):

1. Download from [code.visualstudio.com](https://code.visualstudio.com)
2. Install it like any other program
3. Open VS Code
4. Install the "rust-analyzer" extension:
   - Click the Extensions icon (looks like 4 squares)
   - Search for "rust-analyzer"
   - Click Install

## Understanding the Project Structure

Now let's look at how Armada Strike is organized. Open the game folder in VS Code:

```
armada-strike/
├── src/
│   └── main.rs         <- The game's brain (all the code!)
├── assets/
│   ├── sounds/         <- Sound effects
│   └── sprites/        <- Graphics
├── Cargo.toml          <- Tells Rust what tools we need
└── README.md           <- Instructions for humans
```

Think of this like a recipe book:
- `src/main.rs` is the actual recipe
- `assets/` are the ingredients (sounds and pictures)
- `Cargo.toml` is the shopping list of what we need

## Your First Look at Rust Code

Open `src/main.rs` in VS Code. Don't worry if it looks scary! Let's understand the very first lines:

```rust
use bevy::prelude::*;
```

This line is like saying "I want to use Bevy's game-making tools." Bevy is a game engine - it handles the hard stuff like drawing graphics and playing sounds.

```rust
const GRID_SIZE: usize = 10;
```

This creates a constant (something that never changes) saying our game board is 10x10. The `const` means constant, and `usize` means "a whole number with no decimals."

### Why Does Rust Look Different?

Rust has some unique symbols you'll see a lot:

- `::` (double colon) - like saying "inside of" - `bevy::prelude` means "prelude inside of bevy"
- `->` (arrow) - shows what a function gives back
- `&` (ampersand) - means "borrow this, don't take it"
- `mut` - means "mutable" or "changeable"
- `;` (semicolon) - ends a statement (like a period in English)

Don't worry about memorizing these now - you'll learn them as we go!

## Running the Game

Let's see the game in action! In Terminal/Command Prompt, navigate to the game folder:

```bash
cd path/to/armada-strike
```

Then run:
```bash
cargo run
```

`cargo` is Rust's assistant - it:
- Downloads any needed tools
- Compiles (translates) your code into computer language
- Runs the game

The first time might take a few minutes as it downloads everything. You'll see lots of text scrolling by - that's normal!

## What You've Learned

✅ Why games use Rust (it's fast and safe!)  
✅ How to install Rust on your computer  
✅ How to set up VS Code for reading code  
✅ What the project folders mean  
✅ How to run the game with `cargo run`  
✅ Some basic Rust symbols  

## Homework Challenge

1. Run the game with `cargo run`
2. Play a few rounds to understand what it does
3. Find the line that says `GRID_SIZE: usize = 10` in main.rs
4. Try changing the 10 to 8 (making a smaller board)
5. Run the game again - did the board get smaller?

## What's Next?

In Lesson 2, we'll dive into how the game board works - how the computer remembers where your ships are and what's been hit. It's like learning how the game "thinks"!

---

**Stuck?** That's normal! Every programmer gets stuck. Try:
1. Re-read the instructions
2. Make sure you saved the file after changes
3. Check that you're in the right folder
4. Google the error message (yes, real programmers do this all the time!)

See you in Lesson 2! 🚀