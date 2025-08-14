# Lesson 5: Sound Effects and Save Games - A Code Walkthrough for Logan

Hey Logan! In our final lesson, we'll explore two cool features: the retro 8-bit sounds and the creative save game system with random three-word names!

## How Games Play Sounds

Games need to:
1. Load sound files when starting
2. Keep them in memory (so they play instantly)
3. Play the right sound at the right time
4. Control volume

## Loading Sound Assets

Look at lines 88-94:

```rust
#[derive(Resource)]
struct SoundAssets {
    hit: Handle<AudioSource>,     // Explosion sound
    miss: Handle<AudioSource>,    // Splash sound
    place: Handle<AudioSource>,   // Ship placement sound
    sink: Handle<AudioSource>,    // Ship sinking sound
}
```

A `Handle` is like a ticket at a coat check - it doesn't hold the actual sound, just a way to get it when needed.

The `load_sounds` function (around line 400):

```rust
fn load_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(SoundAssets {
        hit: asset_server.load("sounds/hit.wav"),
        miss: asset_server.load("sounds/miss.wav"),
        place: asset_server.load("sounds/place.wav"),
        sink: asset_server.load("sounds/sink.wav"),
    });
}
```

This tells Bevy: "Load these sound files and give me handles to them."

## Playing Sounds

When something happens, we play sounds (simplified):

```rust
// When hitting a ship
if target == CellState::Ship {
    commands.spawn(AudioBundle {
        source: sounds.hit.clone(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Once,
            volume: Volume::new(0.7),  // 70% volume
            ..default()
        },
    });
}
```

Breaking this down:
- `spawn` creates a new sound player
- `source` is which sound to play
- `PlaybackMode::Once` means play once and stop
- `Volume::new(0.7)` sets volume (0.0 = mute, 1.0 = full)

## The 8-Bit Sound Generator

There's a separate Python script that creates the retro sounds! Here's how 8-bit sounds work:

```python
def create_hit_sound():
    # 8-bit sounds use square waves (on/off, no smooth curves)
    frequency = 150  # Hz (how high/low the pitch is)
    duration = 0.3   # seconds
    
    # Create square wave (jumps between -1 and 1)
    # This creates that classic "beep" sound
```

8-bit sounds are called that because old computers could only use 8 bits (256 values) for sound. Modern sounds use 16 or 24 bits (65,536+ values) for smoother audio.

## The Save Game System

The save system is really clever! Look at lines 104-111:

```rust
#[derive(Serialize, Deserialize)]
struct SaveGame {
    name: String,                    // "swift-eagle-strikes"
    player_board: [[CellState; GRID_SIZE]; GRID_SIZE],
    opponent_board: [[CellState; GRID_SIZE]; GRID_SIZE],
    ship_positions: Vec<PlacedShip>,
    ships_placed: Vec<ShipType>,
}
```

`Serialize` means "turn into text" and `Deserialize` means "turn text back into data". This lets us save games as JSON files!

## Random Three-Word Names

The coolest part - generating save names (lines 352-377):

```rust
fn generate_random_name() -> String {
    let adjectives = [
        "swift", "mighty", "brave", "silent", "golden", 
        "silver", "crimson", "azure", "emerald", "fierce",
        // ... more adjectives
    ];
    
    let nouns = [
        "eagle", "shark", "wolf", "tiger", "dragon",
        "phoenix", "kraken", "falcon", "viper", "cobra",
        // ... more nouns
    ];
    
    let verbs = [
        "strikes", "hunts", "soars", "prowls", "guards",
        "watches", "waits", "stalks", "rises", "falls",
        // ... more verbs
    ];
    
    // Pick one random word from each list
    let adj = adjectives[random_number(0..adjectives.len())];
    let noun = nouns[random_number(0..nouns.len())];
    let verb = verbs[random_number(0..verbs.len())];
    
    format!("{}-{}-{}", adj, noun, verb)
    // Results: "swift-eagle-strikes" or "golden-dragon-soars"
}
```

This is way cooler than "Save1", "Save2"! With 24 adjectives, 24 nouns, and 24 verbs, there are 13,824 possible combinations!

## Saving to a File

The save process (lines 259-276):

```rust
fn save_to_file(&self, name: Option<String>) -> Result<String, Box<dyn Error>> {
    // Generate name if not provided
    let save_name = name.unwrap_or_else(generate_random_name);
    
    // Pack everything into a SaveGame struct
    let save = SaveGame {
        name: save_name.clone(),
        player_board: self.player_board,
        opponent_board: self.opponent_board,
        ship_positions: self.ship_positions.clone(),
        ships_placed: self.ships_placed.clone(),
    };
    
    // Convert to JSON text
    let json = serde_json::to_string_pretty(&save)?;
    
    // Save to file
    let path = get_save_dir()?;  // Usually ~/.config/armada-strike/saves/
    let filename = format!("{}.json", save_name);
    fs::write(path.join(filename), json)?;
    
    Ok(save_name)
}
```

The `?` operator means "if this fails, stop and return the error". It's Rust's way of handling things that might go wrong.

## What a Save File Looks Like

If you open a save file, you'll see JSON:

```json
{
  "name": "mighty-shark-hunts",
  "player_board": [
    ["Empty", "Empty", "Ship", "Ship", "Ship", "Empty", ...],
    ["Empty", "Hit", "Empty", "Empty", "Empty", "Empty", ...],
    ...
  ],
  "opponent_board": [
    ["Miss", "Empty", "Empty", "Hit", "Empty", ...],
    ...
  ],
  "ship_positions": [
    {
      "ship_type": "Carrier",
      "x": 2,
      "y": 0,
      "is_horizontal": true
    },
    ...
  ],
  "ships_placed": ["Carrier", "Battleship", "Cruiser"]
}
```

It's human-readable! You could even edit it by hand (but that would be cheating 😉).

## Loading a Save

Loading reverses the process (lines 278-290):

```rust
fn load_from_file(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
    // Build file path
    let path = get_save_dir()?;
    let filename = format!("{}.json", name);
    
    // Read file
    let json = fs::read_to_string(path.join(filename))?;
    
    // Convert JSON back to SaveGame
    let save: SaveGame = serde_json::from_str(&json)?;
    
    // Update game state
    self.player_board = save.player_board;
    self.opponent_board = save.opponent_board;
    self.ship_positions = save.ship_positions;
    self.ships_placed = save.ships_placed;
    
    Ok(())
}
```

## The Save Directory

The game stores saves in a standard location (lines 337-344):

```rust
fn get_save_dir() -> Result<PathBuf, Box<dyn Error>> {
    let mut path = dirs::config_dir()
        .ok_or("Could not find config directory")?;
    path.push("armada-strike");
    path.push("saves");
    fs::create_dir_all(&path)?;  // Create if doesn't exist
    Ok(path)
}
```

On different systems:
- Mac: `~/Library/Application Support/armada-strike/saves/`
- Windows: `C:\Users\Logan\AppData\Roaming\armada-strike\saves\`
- Linux: `~/.config/armada-strike/saves/`

## Sound Settings

The game has volume control (lines 114-132):

```rust
struct GameSettings {
    sound_enabled: bool,
    sound_volume: f32,  // 0.0 to 1.0
    // ... other settings
}
```

When playing sounds, it checks these settings:

```rust
if settings.sound_enabled {
    let volume = Volume::new(settings.sound_volume);
    // Play sound with this volume
}
```

## What You've Learned

✅ How games load and play sound files  
✅ What 8-bit sounds are  
✅ How data is serialized to JSON  
✅ The creative three-word naming system  
✅ Where save files are stored  
✅ How loading reverses the save process  
✅ Volume control and settings  

## Homework Challenge

1. Find the word lists in `generate_random_name()`
2. Add your own words:
   - Add "legendary" to adjectives
   - Add "leviathan" to nouns  
   - Add "conquers" to verbs
3. Save a few games and see if you get your new combinations!

## Extra Challenge - Custom Save Names

Try adding the ability to type custom save names:
1. Add a text input field
2. If user types a name, use it
3. If empty, use random name

This is harder - you'll need to handle text input!

## Final Project Ideas

Now that you understand the whole game, try:

1. **New Ship Type**: Add a 1-square "Spy Boat"
2. **Power-Ups**: Special shots that reveal a 3x3 area
3. **AI Opponent**: Make the computer place ships and shoot back
4. **Network Play**: Play against friends online (advanced!)
5. **Custom Themes**: Space ships instead of boats

## Debugging Save Issues

Common problems:
- No permission to save directory → Run as admin
- Corrupted save file → Delete it and save again
- Can't find saves → Check the right directory for your OS

## Congratulations, Logan! 🎉

You've completed all 5 lessons and now understand:
- How a real game is structured
- How Rust code works
- Game loops and graphics
- Input handling
- Sound and save systems

You've looked at the code of a complete, working game. That's a huge achievement!

## Your Journey From Here

1. **Experiment**: Change things, break things, fix things
2. **Build**: Start with tiny projects (Tic-Tac-Toe, Snake)
3. **Learn**: Try the Rust Book (doc.rust-lang.org/book)
4. **Share**: Show others what you create!

Remember: Every expert was once a beginner. The difference is they kept coding!

---

**Final Thought:** You now know more about how games work than 99% of people who play them. You're not just a player anymore - you're beginning to understand the magic behind the screen. Keep learning, keep building, and most importantly, have fun!

Good luck on your programming journey! 🚀

— Your guide through Armada Strike