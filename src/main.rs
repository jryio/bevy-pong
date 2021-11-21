mod systems;

use crate::systems::{collision::collision_system, round::round_system, velocity::velocity_system};
use bevy::{prelude::*, render::pass::ClearColor};

fn main() {
    App::build()
        // Clear Color is the background color
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Scoreboard::default())
        .insert_resource(WindowDescriptor {
            title: "Bevy Pong".to_string(),
            width: 1000.0,
            height: 800.0,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system.system())
        .add_system(round_system.system())
        .add_system_set(
            SystemSet::new()
                .label("input")
                .with_system(keyboard_input_system.system()),
        )
        .add_system_set(
            SystemSet::new()
                .label("physics")
                .with_system(collision_system.system().label("collision"))
                .with_system(velocity_system.system().after("collision")),
        )
        .add_system(render_system.system().after("physics"))
        .run();
}

/*
 * Components
 */

// Player Type
pub enum PlayerType {
    LeftPlayer,
    RightPlayer,
}
pub struct Player {
    player_type: PlayerType,
}

// Positions are percentage based
pub struct Position {
    x: f32,
    y: f32,
}

pub struct Velocity(Vec2);
pub struct Ball;
pub struct LostRound;
pub enum WallSide {
    Left,
    Right,
}
pub struct Wall {
    side: WallSide,
}

#[derive(Default)]
pub struct Scoreboard {
    left_score: usize,
    right_score: usize,
}

const LEFT_PLAYER_ORIGIN: Position = Position { x: -42.5, y: 0.0 };
const RIGHT_PLAYER_ORIGIN: Position = Position { x: 42.5, y: 0.0 };
const BALL_ORIGIN: Position = Position { x: 0.0, y: 0.0 };
const BALL_SIZE: [f32; 2] = [5.0, 5.0];
const PADDLE_HEIGHT: f32 = 36.0;
const PADDLE_SIZE: [f32; 2] = [6.0, PADDLE_HEIGHT];
const DASH_WIDTH: f32 = 1.0;
const DASH_HEIGHT: f32 = 8.0;
const DASH_SIZE: [f32; 2] = [DASH_WIDTH, DASH_HEIGHT];
const DASH_PADDING: f32 = 20.0;

// The origin (0,0) of bevy's coordinate system is in the center of the screen
fn startup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Res<WindowDescriptor>,
) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Left Player
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::from(PADDLE_SIZE)),
            ..Default::default()
        })
        .insert(Player {
            player_type: PlayerType::LeftPlayer,
        })
        .insert(Size::new(PADDLE_SIZE[0], PADDLE_SIZE[1]))
        .insert(LEFT_PLAYER_ORIGIN);

    // Right Player
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::from(PADDLE_SIZE)),
            ..Default::default()
        })
        .insert(Player {
            player_type: PlayerType::RightPlayer,
        })
        .insert(Size::new(PADDLE_SIZE[0], PADDLE_SIZE[1]))
        .insert(RIGHT_PLAYER_ORIGIN);

    // Ball
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::from(BALL_SIZE)),
            ..Default::default()
        })
        .insert(Ball)
        .insert(BALL_ORIGIN)
        .insert(Size::new(BALL_SIZE[0], BALL_SIZE[1]))
        .insert(Velocity(Vec2::new(0.25, 0.0)));

    // Invisible walls for collision detection
    let wall_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let wall_thickess = 1.0;
    // Left Side -> Top Wall
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            sprite: Sprite::new(Vec2::new(window.width / 2.0, wall_thickess)),
            transform: Transform::from_xyz(-window.width / 4.0, (window.height / 2.0) - 1.0, 0.0),
            ..Default::default()
        })
        .insert(Wall {
            side: WallSide::Left,
        });
    // Left Side -> Left Wall
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            sprite: Sprite::new(Vec2::new(wall_thickess, window.height)),
            transform: Transform::from_xyz((-window.width / 2.0) + 1.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Wall {
            side: WallSide::Left,
        });
    // Left Side -> Bottom Wall
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            sprite: Sprite::new(Vec2::new(window.width / 2.0, wall_thickess)),
            transform: Transform::from_xyz(-window.width / 5.0, -(window.height / 2.0) + 1.0, 0.0),
            ..Default::default()
        })
        .insert(Wall {
            side: WallSide::Left,
        });

    // Dashes
    let window_top = (window.height / 2.0).abs();
    let dash_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let dash_sprite = Sprite::new(Vec2::from(DASH_SIZE));
    let dash_count = ((window.height) / (DASH_HEIGHT + DASH_PADDING)).floor() as u16;
    let mut dashes = vec![];
    for i in 0..=dash_count {
        let y = window_top - (i as f32 * (DASH_PADDING + DASH_HEIGHT));
        dashes.push(SpriteBundle {
            material: dash_material.clone(),
            sprite: dash_sprite.clone(),
            transform: Transform::from_xyz(0.0, y, 0.0),
            ..Default::default()
        });
    }

    commands.spawn_batch(dashes);
}

// Controls
// ----------------------------------
// Player 1 -> UP -> Up Arrow Key
// Player 1 -> DOWN -> Down Arrow Key
// Player 2 -> UP -> 'W' Key
// Player 2 -> DOWN -> 'S' Key
#[allow(clippy::type_complexity)]
fn keyboard_input_system(mut players: Query<(&mut Position, &Player)>, key: Res<Input<KeyCode>>) {
    for (mut position, player) in players.iter_mut() {
        match player.player_type {
            PlayerType::LeftPlayer => {
                if key.pressed(KeyCode::W) {
                    position.y += 1.0
                } else if key.pressed(KeyCode::S) {
                    position.y -= 1.0
                }
            }
            PlayerType::RightPlayer => {
                if key.pressed(KeyCode::Up) {
                    position.y += 1.0
                } else if key.pressed(KeyCode::Down) {
                    position.y -= 1.0
                }
            }
        }
    }
}

// Convert relative position to absolute
fn render_system(mut query: Query<(&Position, &mut Transform)>, window: Res<WindowDescriptor>) {
    for (position, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x = position.x / 100.0 * window.width;
        translation.y = position.y / 100.0 * window.height;
    }
}
