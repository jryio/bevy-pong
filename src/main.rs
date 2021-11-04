use bevy::prelude::*;
use bevy::render::pass::ClearColor;

// Entities
enum PlayerType {
    TopPlayer,
    BottomPlayer,
}
struct Paddle {
    player_type: PlayerType,
}

// Components

struct Position {
    x: f32,
    y: f32,
}

struct Vector {
    x: f32,
    y: f32,
}

struct Ball;

fn initialize_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    println!("initialize_player called");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(20.0, 100.0)),
            ..Default::default()
        })
        .insert(Paddle {
            player_type: PlayerType::TopPlayer,
        })
        .insert(Position { x: 20.0, y: 50.0 });
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(20.0, 100.0)),
            ..Default::default()
        })
        .insert(Paddle {
            player_type: PlayerType::BottomPlayer,
        })
        .insert(Position { x: 50.0, y: 50.0 });
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(5.0, 5.0)),
            ..Default::default()
        })
        .insert(Ball)
        .insert(Position { x: 30.0, y: 50.0 });
}

// Convert relative position to absolute
fn render_system(mut query: Query<(&Position, &mut Transform)>, window: Res<WindowDescriptor>) {
    for (position, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x = (position.x - 50.0) / 100.0 * window.width;
        translation.y = (position.y - 50.0) / 100.0 * window.height;
    }
}

// 1. query for paddles, get components, mutate position
fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Pong".to_string(),
            width: 1000.0,
            height: 1000.0,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(initialize_player.system())
        .add_system(render_system.system())
        .run();
}
