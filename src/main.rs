use bevy::prelude::*;

const GRID_SIZE: usize = 10;
const CELL_SIZE: f32 = 30.0;
const CELL_SPACING: f32 = 2.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Resource)]
struct GameState {
    player_board: [[CellState; GRID_SIZE]; GRID_SIZE],
    opponent_board: [[CellState; GRID_SIZE]; GRID_SIZE],
    selected_x: usize,
    selected_y: usize,
    is_player_board: bool,
    placement_mode: PlacementMode,
    ships_placed: Vec<ShipType>,
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
        }
    }
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
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            update_cell_colors,
            update_selection_indicator,
            update_status_text,
            show_ship_preview,
        ))
        .run();
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
) {
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
            
            if keyboard.just_pressed(KeyCode::Escape) {
                game_state.placement_mode = PlacementMode::Playing;
            }
            
            if keyboard.just_pressed(KeyCode::Enter) {
                let x = game_state.selected_x;
                let y = game_state.selected_y;
                if can_place_ship(&game_state.player_board, x, y, ship_type.size(), is_horizontal) {
                    place_ship(&mut game_state.player_board, x, y, ship_type.size(), is_horizontal);
                    game_state.ships_placed.push(ship_type);
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
                    }
                }
                if keyboard.just_pressed(KeyCode::KeyM) {
                    if game_state.player_board[y][x] != CellState::Ship {
                        game_state.player_board[y][x] = CellState::Miss;
                    }
                }
                if keyboard.just_pressed(KeyCode::KeyC) {
                    game_state.player_board[y][x] = CellState::Empty;
                }
            } else {
                if keyboard.just_pressed(KeyCode::KeyH) {
                    game_state.opponent_board[y][x] = CellState::Hit;
                }
                if keyboard.just_pressed(KeyCode::KeyM) {
                    game_state.opponent_board[y][x] = CellState::Miss;
                }
                if keyboard.just_pressed(KeyCode::KeyC) {
                    game_state.opponent_board[y][x] = CellState::Empty;
                }
            }

            if keyboard.just_pressed(KeyCode::KeyR) {
                game_state.player_board = [[CellState::Empty; GRID_SIZE]; GRID_SIZE];
                game_state.opponent_board = [[CellState::Empty; GRID_SIZE]; GRID_SIZE];
                game_state.ships_placed.clear();
            }
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