use bevy::{core::FixedTimestep, prelude::*, render::pass::ClearColor};

fn main() {
    App::build()
        // Clear Color is the background color
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
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
        .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)))
        .add_system(render_system.system())
        .run();
}

/*
 * Entities
 */

enum PlayerType {
    LeftPlayer,
    RightPlayer,
}
struct Paddle {
    player_type: PlayerType,
}

/*
 * Components
 */

// Positions are percentage based
struct Position {
    x: f32,
    y: f32,
}

// Vectors components are described as scalars between [0,1]
// with two decimal places of precision
struct Vector {
    x: f32,
    y: f32,
}

struct Ball;

// The origin (0,0) of bevy's coordinate system is in the center of the screen
fn startup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Res<WindowDescriptor>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::from(PADDLE_SIZE)),
            ..Default::default()
        })
        .insert(Paddle {
            player_type: PlayerType::LeftPlayer,
        })
        .insert(LEFT_PLAYER_ORIGIN);
    // Right Player
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::from(PADDLE_SIZE)),
            ..Default::default()
        })
        .insert(Paddle {
            player_type: PlayerType::RightPlayer,
        })
        .insert(RIGHT_PLAYER_ORIGIN);
    // Ball
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(5.0, 5.0)),
            ..Default::default()
        })
        .insert(Ball)
        .insert(BALL_ORIGIN);

    // Compute how many dashes can fit evenly vertically in the window with DASH_PADDING inbetween
    let window_top = (window.height / 2.0).abs();
    let dash_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let dash_sprite = Sprite::new(Vec2::from(DASH_SIZE));
    let dash_count = ((window.height) / (DASH_HEIGHT + DASH_PADDING)).floor() as u16;
    let mut dashes = vec![];
    for i in 0..=dash_count {
        // let padding = if i == 0 { 0.0 } else { DASH_PADDING };
        let y = window_top - (i as f32 * (DASH_PADDING + DASH_HEIGHT));
        dashes.push(SpriteBundle {
            material: dash_material.clone(),
            sprite: dash_sprite.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, y, 0.0)),
            ..Default::default()
        });
    }

    commands.spawn_batch(dashes);
}

// Convert relative position to absolute
fn render_system(mut query: Query<(&Position, &mut Transform)>, window: Res<WindowDescriptor>) {
    for (position, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x = position.x / 100.0 * window.width;
        translation.y = position.y / 100.0 * window.height;
    }
}

const TIME_STEP: f32 = 1.0 / 60.0;
const LEFT_PLAYER_ORIGIN: Position = Position { x: -25.0, y: 0.0 };
const RIGHT_PLAYER_ORIGIN: Position = Position { x: 25.0, y: 0.0 };
const BALL_ORIGIN: Position = Position { x: 0.0, y: 0.0 };
const PADDLE_SIZE: [f32; 2] = [6.0, 36.0];
const DASH_WIDTH: f32 = 1.0;
const DASH_HEIGHT: f32 = 8.0;
const DASH_SIZE: [f32; 2] = [DASH_WIDTH, DASH_HEIGHT];
const DASH_PADDING: f32 = 20.0;
