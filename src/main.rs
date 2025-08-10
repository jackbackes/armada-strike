use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::audio::{PlaybackSettings, Volume};
use serde::{Deserialize, Serialize};
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::fs;
use std::path::PathBuf;

const GRID_SIZE: usize = 10;
const CELL_SIZE: f32 = 30.0;
const CELL_SPACING: f32 = 2.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ShipType {
    Carrier,    // 5
    Battleship, // 4
    Cruiser,    // 3
    Submarine,  // 3
    Destroyer,  // 2
}

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
    
    fn name(&self) -> &str {
        match self {
            ShipType::Carrier => "Carrier (5)",
            ShipType::Battleship => "Battleship (4)",
            ShipType::Cruiser => "Cruiser (3)",
            ShipType::Submarine => "Submarine (3)",
            ShipType::Destroyer => "Destroyer (2)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlacementMode {
    Playing,
    PlacingShip(ShipType, bool), // ship type, is_horizontal
}

#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
    is_player_board: bool,
}

#[derive(Component)]
struct SelectedCell;

#[derive(Component)]
struct StatusText;

#[derive(Component)]
struct CellInfoText;

#[derive(Component)]
struct SettingsMenu;

#[derive(Resource)]
struct SoundAssets {
    hit: Handle<AudioSource>,
    miss: Handle<AudioSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PlacedShip {
    ship_type: ShipType,
    x: usize,
    y: usize,
    is_horizontal: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct SaveGame {
    name: String,
    player_board: [[CellState; GRID_SIZE]; GRID_SIZE],
    opponent_board: [[CellState; GRID_SIZE]; GRID_SIZE],
    ship_positions: Vec<PlacedShip>,
    ships_placed: Vec<ShipType>,
}

#[derive(Resource)]
struct GameSettings {
    show_settings: bool,
    current_save: Option<String>,
    saves: Vec<SaveGame>,
    sound_enabled: bool,
    sound_volume: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            show_settings: false,
            current_save: None,
            saves: Vec::new(),
            sound_enabled: true,
            sound_volume: 0.7,
        }
    }
}

impl GameSettings {
    fn load_all_saves(&mut self) {
        if let Ok(save_dir) = get_save_dir() {
            self.saves.clear();
            if let Ok(entries) = fs::read_dir(&save_dir) {
                for entry in entries.flatten() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.ends_with(".json") {
                            if let Ok(json) = fs::read_to_string(entry.path()) {
                                if let Ok(save) = serde_json::from_str::<SaveGame>(&json) {
                                    self.saves.push(save);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Resource)]
struct GameState {
    player_board: [[CellState; GRID_SIZE]; GRID_SIZE],
    opponent_board: [[CellState; GRID_SIZE]; GRID_SIZE],
    selected_x: usize,
    selected_y: usize,
    is_player_board: bool,
    placement_mode: PlacementMode,
    ships_placed: Vec<ShipType>,
    ship_positions: Vec<PlacedShip>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player_board: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
            opponent_board: [[CellState::Empty; GRID_SIZE]; GRID_SIZE],
            selected_x: 0,
            selected_y: 0,
            is_player_board: true,
            placement_mode: PlacementMode::Playing,
            ships_placed: Vec::new(),
            ship_positions: Vec::new(),
        }
    }
}

impl GameState {
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
        None
    }
    
    fn save_to_file(&self, name: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
        let save_name = name.unwrap_or_else(generate_random_name);
        let save = SaveGame {
            name: save_name.clone(),
            player_board: self.player_board,
            opponent_board: self.opponent_board,
            ship_positions: self.ship_positions.clone(),
            ships_placed: self.ships_placed.clone(),
        };
        
        let save_dir = get_save_dir()?;
        fs::create_dir_all(&save_dir)?;
        let mut save_path = save_dir;
        save_path.push(format!("{}.json", sanitize_filename(&save_name)));
        let json = serde_json::to_string_pretty(&save)?;
        fs::write(save_path, json)?;
        Ok(save_name)
    }
    
    fn load_from_file(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let save_dir = get_save_dir()?;
        let mut save_path = save_dir;
        save_path.push(format!("{}.json", sanitize_filename(name)));
        let json = fs::read_to_string(save_path)?;
        let save: SaveGame = serde_json::from_str(&json)?;
        
        self.player_board = save.player_board;
        self.opponent_board = save.opponent_board;
        self.ship_positions = save.ship_positions;
        self.ships_placed = save.ships_placed;
        Ok(())
    }
    
    fn place_random_ships(&mut self) {
        let mut rng = thread_rng();
        self.clear_board();
        
        let ships = [
            ShipType::Carrier,
            ShipType::Battleship,
            ShipType::Cruiser,
            ShipType::Submarine,
            ShipType::Destroyer,
        ];
        
        for ship_type in ships.iter() {
            let mut placed = false;
            let mut attempts = 0;
            
            while !placed && attempts < 100 {
                let x = rng.gen_range(0..GRID_SIZE);
                let y = rng.gen_range(0..GRID_SIZE);
                let is_horizontal = rng.gen_bool(0.5);
                
                if can_place_ship(&self.player_board, x, y, ship_type.size(), is_horizontal) {
                    place_ship(&mut self.player_board, x, y, ship_type.size(), is_horizontal);
                    self.ship_positions.push(PlacedShip {
                        ship_type: *ship_type,
                        x,
                        y,
                        is_horizontal,
                    });
                    self.ships_placed.push(*ship_type);
                    placed = true;
                }
                attempts += 1;
            }
        }
    }
    
    fn clear_board(&mut self) {
        self.player_board = [[CellState::Empty; GRID_SIZE]; GRID_SIZE];
        self.opponent_board = [[CellState::Empty; GRID_SIZE]; GRID_SIZE];
        self.ships_placed.clear();
        self.ship_positions.clear();
    }
}

fn get_save_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = dirs::config_dir()
        .ok_or("Could not find config directory")?;
    path.push("battleship");
    path.push("saves");
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

fn generate_random_name() -> String {
    let adjectives = [
        "swift", "mighty", "brave", "silent", "golden", "silver", "crimson", "azure",
        "emerald", "fierce", "bold", "clever", "cunning", "daring", "noble", "proud",
        "ancient", "mystic", "hidden", "frozen", "burning", "stormy", "peaceful", "wild",
    ];
    
    let nouns = [
        "eagle", "shark", "wolf", "tiger", "dragon", "phoenix", "kraken", "falcon",
        "viper", "cobra", "panther", "jaguar", "lion", "bear", "raven", "hawk",
        "thunder", "storm", "wave", "tide", "wind", "fire", "ice", "shadow",
    ];
    
    let verbs = [
        "strikes", "hunts", "soars", "prowls", "guards", "watches", "waits", "stalks",
        "rises", "falls", "moves", "dances", "fights", "defends", "attacks", "charges",
        "glides", "dives", "leaps", "runs", "flies", "swims", "crawls", "hides",
    ];
    
    let mut rng = thread_rng();
    let adj = adjectives.choose(&mut rng).unwrap();
    let noun = nouns.choose(&mut rng).unwrap();
    let verb = verbs.choose(&mut rng).unwrap();
    
    format!("{}-{}-{}", adj, noun, verb)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Battleship Tracker".to_string(),
                resolution: (1000.0, 700.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .init_resource::<GameSettings>()
        .add_systems(Startup, (setup, load_sounds))
        .add_systems(Update, (
            handle_input,
            handle_mouse_click,
            update_cell_colors,
            update_selection_indicator,
            update_status_text,
            update_cell_info,
            show_ship_preview,
            handle_settings_menu,
        ))
        .run();
}

fn load_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let hit_sound = asset_server.load("sounds/hit.wav");
    let miss_sound = asset_server.load("sounds/miss.wav");
    
    commands.insert_resource(SoundAssets {
        hit: hit_sound,
        miss: miss_sound,
    });
    
    println!("Sound effects loaded");
}

fn play_sound(
    commands: &mut Commands,
    sound: Handle<AudioSource>,
    settings: &GameSettings,
) {
    if settings.sound_enabled {
        commands.spawn((
            AudioPlayer(sound),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.sound_volume)),
        ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let board_width = GRID_SIZE as f32 * (CELL_SIZE + CELL_SPACING);
    let shift_left = -board_width / 2.0;  // Shift everything left by half a board width
    let player_board_offset = -board_width / 2.0 - 40.0 + shift_left;
    let opponent_board_offset = board_width / 2.0 + 40.0 + shift_left;

    // Board labels - centered above each board
    commands.spawn((
        Text2d::new("Your Board"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(player_board_offset + board_width / 2.0, 220.0, 0.0),
    ));

    commands.spawn((
        Text2d::new("Opponent's Board"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(opponent_board_offset + board_width / 2.0, 220.0, 0.0),
    ));
    
    // Add column labels (A-J) for both boards
    for i in 0..GRID_SIZE {
        let letter = (b'A' + i as u8) as char;
        
        // Player board column labels
        commands.spawn((
            Text2d::new(letter.to_string()),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(
                player_board_offset + i as f32 * (CELL_SIZE + CELL_SPACING) + CELL_SIZE / 2.0,
                180.0,
                0.0
            ),
        ));
        
        // Opponent board column labels
        commands.spawn((
            Text2d::new(letter.to_string()),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(
                opponent_board_offset + i as f32 * (CELL_SIZE + CELL_SPACING) + CELL_SIZE / 2.0,
                180.0,
                0.0
            ),
        ));
    }
    
    // Add row labels (1-10) for both boards
    for i in 0..GRID_SIZE {
        let number = (GRID_SIZE - i).to_string();
        
        // Player board row labels
        commands.spawn((
            Text2d::new(number.clone()),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(
                player_board_offset - 25.0,
                i as f32 * (CELL_SIZE + CELL_SPACING) - 150.0 + CELL_SIZE / 2.0,
                0.0
            ),
        ));
        
        // Opponent board row labels
        commands.spawn((
            Text2d::new(number),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(
                opponent_board_offset - 25.0,
                i as f32 * (CELL_SIZE + CELL_SPACING) - 150.0 + CELL_SIZE / 2.0,
                0.0
            ),
        ));
    }

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let x_pos = player_board_offset + x as f32 * (CELL_SIZE + CELL_SPACING) + CELL_SIZE / 2.0;
            let y_pos = y as f32 * (CELL_SIZE + CELL_SPACING) - 150.0 + CELL_SIZE / 2.0;
            
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.3, 0.3, 0.3),
                    custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x_pos, y_pos, 0.0),
                Cell { x, y, is_player_board: true },
            ));
            
            let x_pos = opponent_board_offset + x as f32 * (CELL_SIZE + CELL_SPACING) + CELL_SIZE / 2.0;
            
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.3, 0.3, 0.3),
                    custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x_pos, y_pos, 0.0),
                Cell { x, y, is_player_board: false },
            ));
        }
    }

    let x_pos = player_board_offset + CELL_SIZE / 2.0;
    let y_pos = -150.0 + CELL_SIZE / 2.0;
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 1.0, 0.0, 0.3),
            custom_size: Some(Vec2::new(CELL_SIZE + 4.0, CELL_SIZE + 4.0)),
            ..default()
        },
        Transform::from_xyz(x_pos, y_pos, 1.0),
        SelectedCell,
    ));

    // Cell info status bar
    commands.spawn((
        Text2d::new("Cell: A1 | Board: Your Board | State: Empty"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.5)),
        Transform::from_xyz(0.0, -250.0, 0.0),
        CellInfoText,
    ));
    
    commands.spawn((
        Text2d::new("Press 1-5 to place ships | Arrow keys: Move | Space: Rotate | Enter: Place | ESC: Cancel"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, -280.0, 0.0),
        StatusText,
    ));
    
    commands.spawn((
        Text2d::new("Controls:\nTab: Switch boards | H: Mark hit | M: Mark miss | C: Clear cell | R: Reset all"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, -320.0, 0.0),
    ));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut settings: ResMut<GameSettings>,
    mut commands: Commands,
    sounds: Option<Res<SoundAssets>>,
) {
    // Toggle settings menu with Escape
    if keyboard.just_pressed(KeyCode::Escape) {
        if matches!(game_state.placement_mode, PlacementMode::PlacingShip(_, _)) {
            game_state.placement_mode = PlacementMode::Playing;
        } else {
            settings.show_settings = !settings.show_settings;
            if settings.show_settings {
                settings.load_all_saves();
                spawn_settings_menu(&mut commands, &settings);
            } else {
                despawn_settings_menu(&mut commands);
            }
        }
        return;
    }
    
    // Load specific save with number keys when settings menu is open
    if settings.show_settings {
        // Toggle sound with S key
        if keyboard.just_pressed(KeyCode::KeyS) {
            settings.sound_enabled = !settings.sound_enabled;
            println!("Sound {}", if settings.sound_enabled { "enabled" } else { "disabled" });
            // Refresh the settings menu to show updated state
            despawn_settings_menu(&mut commands);
            settings.load_all_saves();
            spawn_settings_menu(&mut commands, &settings);
            return;
        }
        
        for i in 1..=5 {
            let key = match i {
                1 => KeyCode::Digit1,
                2 => KeyCode::Digit2,
                3 => KeyCode::Digit3,
                4 => KeyCode::Digit4,
                5 => KeyCode::Digit5,
                _ => continue,
            };
            
            if keyboard.just_pressed(key) {
                if let Some(save) = settings.saves.get(i - 1) {
                    let save_name = save.name.clone();
                    if let Err(e) = game_state.load_from_file(&save_name) {
                        eprintln!("Failed to load save {}: {}", save_name, e);
                    } else {
                        settings.current_save = Some(save_name.clone());
                        settings.show_settings = false;
                        despawn_settings_menu(&mut commands);
                        println!("Loaded: {}", save_name);
                    }
                }
                return;
            }
        }
    }
    
    // Quick shortcuts
    if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::SuperLeft) {
        if keyboard.just_pressed(KeyCode::KeyS) {
            match game_state.save_to_file(settings.current_save.clone()) {
                Ok(name) => {
                    println!("Game saved as: {}", name);
                    settings.current_save = Some(name.clone());
                    settings.load_all_saves();
                }
                Err(e) => eprintln!("Failed to save game: {}", e),
            }
        }
        if keyboard.just_pressed(KeyCode::KeyL) {
            if let Some(ref current) = settings.current_save {
                if let Err(e) = game_state.load_from_file(current) {
                    eprintln!("Failed to load game: {}", e);
                } else {
                    println!("Game loaded: {}", current);
                }
            } else if !settings.saves.is_empty() {
                let first_save_name = settings.saves[0].name.clone();
                if let Err(e) = game_state.load_from_file(&first_save_name) {
                    eprintln!("Failed to load game: {}", e);
                } else {
                    settings.current_save = Some(first_save_name.clone());
                    println!("Game loaded: {}", first_save_name);
                }
            } else {
                println!("No saves found");
            }
        }
        if keyboard.just_pressed(KeyCode::KeyN) {
            game_state.clear_board();
            settings.current_save = None;
            println!("New game started!");
        }
        if keyboard.just_pressed(KeyCode::KeyP) {
            game_state.place_random_ships();
            println!("Ships placed randomly!");
        }
    }
    
    if keyboard.just_pressed(KeyCode::ArrowUp) && game_state.selected_y < GRID_SIZE - 1 {
        game_state.selected_y += 1;
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) && game_state.selected_y > 0 {
        game_state.selected_y -= 1;
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) && game_state.selected_x > 0 {
        game_state.selected_x -= 1;
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) && game_state.selected_x < GRID_SIZE - 1 {
        game_state.selected_x += 1;
    }

    match game_state.placement_mode {
        PlacementMode::PlacingShip(ship_type, is_horizontal) => {
            if keyboard.just_pressed(KeyCode::Space) {
                game_state.placement_mode = PlacementMode::PlacingShip(ship_type, !is_horizontal);
            }
            
            if keyboard.just_pressed(KeyCode::Enter) {
                let x = game_state.selected_x;
                let y = game_state.selected_y;
                if can_place_ship(&game_state.player_board, x, y, ship_type.size(), is_horizontal) {
                    place_ship(&mut game_state.player_board, x, y, ship_type.size(), is_horizontal);
                    game_state.ships_placed.push(ship_type);
                    game_state.ship_positions.push(PlacedShip {
                        ship_type,
                        x,
                        y,
                        is_horizontal,
                    });
                    game_state.placement_mode = PlacementMode::Playing;
                }
            }
        }
        PlacementMode::Playing => {
            if keyboard.just_pressed(KeyCode::Tab) {
                game_state.is_player_board = !game_state.is_player_board;
            }

            if keyboard.just_pressed(KeyCode::Digit1) && !game_state.ships_placed.contains(&ShipType::Carrier) {
                game_state.placement_mode = PlacementMode::PlacingShip(ShipType::Carrier, true);
                game_state.is_player_board = true;
            }
            if keyboard.just_pressed(KeyCode::Digit2) && !game_state.ships_placed.contains(&ShipType::Battleship) {
                game_state.placement_mode = PlacementMode::PlacingShip(ShipType::Battleship, true);
                game_state.is_player_board = true;
            }
            if keyboard.just_pressed(KeyCode::Digit3) && !game_state.ships_placed.contains(&ShipType::Cruiser) {
                game_state.placement_mode = PlacementMode::PlacingShip(ShipType::Cruiser, true);
                game_state.is_player_board = true;
            }
            if keyboard.just_pressed(KeyCode::Digit4) && !game_state.ships_placed.contains(&ShipType::Submarine) {
                game_state.placement_mode = PlacementMode::PlacingShip(ShipType::Submarine, true);
                game_state.is_player_board = true;
            }
            if keyboard.just_pressed(KeyCode::Digit5) && !game_state.ships_placed.contains(&ShipType::Destroyer) {
                game_state.placement_mode = PlacementMode::PlacingShip(ShipType::Destroyer, true);
                game_state.is_player_board = true;
            }

            let x = game_state.selected_x;
            let y = game_state.selected_y;

            if game_state.is_player_board {
                if keyboard.just_pressed(KeyCode::KeyH) {
                    if game_state.player_board[y][x] == CellState::Ship {
                        game_state.player_board[y][x] = CellState::Hit;
                        if let Some(ref sounds) = sounds {
                            play_sound(&mut commands, sounds.hit.clone(), &settings);
                        }
                    }
                }
                if keyboard.just_pressed(KeyCode::KeyM) {
                    if game_state.player_board[y][x] != CellState::Ship {
                        game_state.player_board[y][x] = CellState::Miss;
                        if let Some(ref sounds) = sounds {
                            play_sound(&mut commands, sounds.miss.clone(), &settings);
                        }
                    }
                }
                if keyboard.just_pressed(KeyCode::KeyC) {
                    game_state.player_board[y][x] = CellState::Empty;
                }
            } else {
                if keyboard.just_pressed(KeyCode::KeyH) {
                    game_state.opponent_board[y][x] = CellState::Hit;
                    if let Some(ref sounds) = sounds {
                        play_sound(&mut commands, sounds.hit.clone(), &settings);
                    }
                }
                if keyboard.just_pressed(KeyCode::KeyM) {
                    game_state.opponent_board[y][x] = CellState::Miss;
                    if let Some(ref sounds) = sounds {
                        play_sound(&mut commands, sounds.miss.clone(), &settings);
                    }
                }
                if keyboard.just_pressed(KeyCode::KeyC) {
                    game_state.opponent_board[y][x] = CellState::Empty;
                }
            }

            if keyboard.just_pressed(KeyCode::KeyR) {
                game_state.player_board = [[CellState::Empty; GRID_SIZE]; GRID_SIZE];
                game_state.opponent_board = [[CellState::Empty; GRID_SIZE]; GRID_SIZE];
                game_state.ships_placed.clear();
                game_state.ship_positions.clear();
            }
        }
    }
}

fn handle_mouse_click(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    q_cells: Query<(&Cell, &Transform)>,
    mut game_state: ResMut<GameState>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = q_windows.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    for (cell, transform) in q_cells.iter() {
        let half_size = CELL_SIZE / 2.0;
        let min_x = transform.translation.x - half_size;
        let max_x = transform.translation.x + half_size;
        let min_y = transform.translation.y - half_size;
        let max_y = transform.translation.y + half_size;

        if world_position.x >= min_x && world_position.x <= max_x &&
           world_position.y >= min_y && world_position.y <= max_y {
            game_state.selected_x = cell.x;
            game_state.selected_y = cell.y;
            game_state.is_player_board = cell.is_player_board;
            break;
        }
    }
}

fn can_place_ship(board: &[[CellState; GRID_SIZE]; GRID_SIZE], x: usize, y: usize, size: usize, is_horizontal: bool) -> bool {
    if is_horizontal {
        if x + size > GRID_SIZE {
            return false;
        }
        for i in 0..size {
            if board[y][x + i] != CellState::Empty {
                return false;
            }
        }
    } else {
        if y + size > GRID_SIZE {
            return false;
        }
        for i in 0..size {
            if board[y + i][x] != CellState::Empty {
                return false;
            }
        }
    }
    true
}

fn place_ship(board: &mut [[CellState; GRID_SIZE]; GRID_SIZE], x: usize, y: usize, size: usize, is_horizontal: bool) {
    if is_horizontal {
        for i in 0..size {
            board[y][x + i] = CellState::Ship;
        }
    } else {
        for i in 0..size {
            board[y + i][x] = CellState::Ship;
        }
    }
}

fn update_cell_colors(
    game_state: Res<GameState>,
    mut query: Query<(&Cell, &mut Sprite)>,
) {
    for (cell, mut sprite) in query.iter_mut() {
        let state = if cell.is_player_board {
            game_state.player_board[cell.y][cell.x]
        } else {
            game_state.opponent_board[cell.y][cell.x]
        };

        sprite.color = match state {
            CellState::Empty => Color::srgb(0.3, 0.3, 0.3),
            CellState::Ship => Color::srgb(0.5, 0.5, 0.5),
            CellState::Hit => Color::srgb(1.0, 0.0, 0.0),
            CellState::Miss => Color::srgb(0.0, 0.0, 1.0),
        };
    }
}

fn update_selection_indicator(
    game_state: Res<GameState>,
    mut query: Query<&mut Transform, With<SelectedCell>>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let board_width = GRID_SIZE as f32 * (CELL_SIZE + CELL_SPACING);
        let shift_left = -board_width / 2.0;
        let board_offset = if game_state.is_player_board {
            -board_width / 2.0 - 40.0 + shift_left
        } else {
            board_width / 2.0 + 40.0 + shift_left
        };

        let x_pos = board_offset + game_state.selected_x as f32 * (CELL_SIZE + CELL_SPACING) + CELL_SIZE / 2.0;
        let y_pos = game_state.selected_y as f32 * (CELL_SIZE + CELL_SPACING) - 150.0 + CELL_SIZE / 2.0;
        
        transform.translation.x = x_pos;
        transform.translation.y = y_pos;
    }
}

fn update_status_text(
    game_state: Res<GameState>,
    mut query: Query<&mut Text2d, With<StatusText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        text.0 = match game_state.placement_mode {
            PlacementMode::PlacingShip(ship_type, is_horizontal) => {
                format!(
                    "Placing {} - {} | Space: Rotate | Enter: Place | ESC: Cancel",
                    ship_type.name(),
                    if is_horizontal { "Horizontal" } else { "Vertical" }
                )
            }
            PlacementMode::Playing => {
                let mut ships_to_place = Vec::new();
                if !game_state.ships_placed.contains(&ShipType::Carrier) {
                    ships_to_place.push("1:Carrier(5)");
                }
                if !game_state.ships_placed.contains(&ShipType::Battleship) {
                    ships_to_place.push("2:Battleship(4)");
                }
                if !game_state.ships_placed.contains(&ShipType::Cruiser) {
                    ships_to_place.push("3:Cruiser(3)");
                }
                if !game_state.ships_placed.contains(&ShipType::Submarine) {
                    ships_to_place.push("4:Submarine(3)");
                }
                if !game_state.ships_placed.contains(&ShipType::Destroyer) {
                    ships_to_place.push("5:Destroyer(2)");
                }
                
                if ships_to_place.is_empty() {
                    "All ships placed! Game ready.".to_string()
                } else {
                    format!("Ships to place: {}", ships_to_place.join(" | "))
                }
            }
        };
    }
}

fn update_cell_info(
    game_state: Res<GameState>,
    mut query: Query<&mut Text2d, With<CellInfoText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        let col = (b'A' + game_state.selected_x as u8) as char;
        let row = GRID_SIZE - game_state.selected_y;
        let coord = format!("{}{}", col, row);
        
        let board_name = if game_state.is_player_board {
            "Your Board"
        } else {
            "Opponent's Board"
        };
        
        let state = if game_state.is_player_board {
            game_state.player_board[game_state.selected_y][game_state.selected_x]
        } else {
            game_state.opponent_board[game_state.selected_y][game_state.selected_x]
        };
        
        let state_str = match state {
            CellState::Empty => "Empty",
            CellState::Ship => "Ship",
            CellState::Hit => "Hit",
            CellState::Miss => "Miss",
        };
        
        let mut info = format!("Cell: {} | Board: {} | State: {}", coord, board_name, state_str);
        
        if game_state.is_player_board && state == CellState::Ship {
            if let Some(ship_type) = game_state.get_ship_at(game_state.selected_x, game_state.selected_y) {
                info.push_str(&format!(" | Ship: {}", ship_type.name()));
            }
        }
        
        text.0 = info;
    }
}

fn spawn_settings_menu(commands: &mut Commands, settings: &GameSettings) {
    // Background panel
    commands.spawn((
        Node {
            width: Val::Px(400.0),
            height: Val::Px(500.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            margin: UiRect::all(Val::Px(-200.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
        SettingsMenu,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("Game Settings"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
        
        // Instructions
        parent.spawn((
            Text::new("Keyboard Shortcuts:\n\nCtrl+S: Save Game\nCtrl+L: Load Game\nCtrl+N: New Game\nCtrl+P: Random Ship Placement\nS: Toggle Sound\n\nESC: Close Settings"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
        
        // Current save info
        let current_save_text = if let Some(ref save) = settings.current_save {
            format!("Current Save: {}", save)
        } else {
            "Current Save: None (New Game)".to_string()
        };
        
        parent.spawn((
            Text::new(current_save_text),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.8, 1.0)),
            Node {
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
        ));
        
        // Saved games list
        parent.spawn((
            Text::new("Saved Games:"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        ));
        
        if settings.saves.is_empty() {
            parent.spawn((
                Text::new("No saved games found"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        } else {
            for (i, save) in settings.saves.iter().take(5).enumerate() {
                let prefix = if Some(&save.name) == settings.current_save.as_ref() {
                    "→ "
                } else {
                    "  "
                };
                parent.spawn((
                    Text::new(format!("{}{}. {}", prefix, i + 1, save.name)),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(if Some(&save.name) == settings.current_save.as_ref() {
                        Color::srgb(0.6, 1.0, 0.6)
                    } else {
                        Color::srgb(0.7, 0.7, 0.7)
                    }),
                ));
            }
            
            if settings.saves.len() > 5 {
                parent.spawn((
                    Text::new(format!("  ... and {} more", settings.saves.len() - 5)),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                ));
            }
        }
        
        // Sound settings
        parent.spawn((
            Text::new("\nSound Settings:"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
        ));
        
        let sound_status = if settings.sound_enabled {
            format!("Sound: ON (Volume: {}%)", (settings.sound_volume * 100.0) as i32)
        } else {
            "Sound: OFF".to_string()
        };
        
        parent.spawn((
            Text::new(sound_status),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(if settings.sound_enabled {
                Color::srgb(0.6, 1.0, 0.6)
            } else {
                Color::srgb(0.7, 0.7, 0.7)
            }),
        ));
        
        parent.spawn((
            Text::new("Press 'S' to toggle sound on/off"),
            TextFont {
                font_size: 10.0,
                ..default()
            },
            TextColor(Color::srgb(0.5, 0.5, 0.5)),
        ));
    });
}

fn despawn_settings_menu(_commands: &mut Commands) {
    // Will be handled by a separate system
}

fn handle_settings_menu(
    settings: Res<GameSettings>,
    query: Query<Entity, With<SettingsMenu>>,
    mut commands: Commands,
) {
    if !settings.show_settings {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn show_ship_preview(
    game_state: Res<GameState>,
    mut query: Query<(&Cell, &mut Sprite)>,
) {
    if let PlacementMode::PlacingShip(ship_type, is_horizontal) = game_state.placement_mode {
        let size = ship_type.size();
        let can_place = can_place_ship(&game_state.player_board, game_state.selected_x, game_state.selected_y, size, is_horizontal);
        
        for (cell, mut sprite) in query.iter_mut() {
            if !cell.is_player_board {
                continue;
            }
            
            let is_preview = if is_horizontal {
                cell.y == game_state.selected_y && 
                cell.x >= game_state.selected_x && 
                cell.x < game_state.selected_x + size
            } else {
                cell.x == game_state.selected_x && 
                cell.y >= game_state.selected_y && 
                cell.y < game_state.selected_y + size
            };
            
            if is_preview && game_state.player_board[cell.y][cell.x] == CellState::Empty {
                sprite.color = if can_place {
                    Color::srgba(0.0, 1.0, 0.0, 0.5)
                } else {
                    Color::srgba(1.0, 0.0, 0.0, 0.5)
                };
            }
        }
    }
}