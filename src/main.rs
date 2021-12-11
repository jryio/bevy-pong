mod components;
mod constants;
mod keymaps;
mod systems;

use crate::components::*;
use crate::constants::*;
use crate::systems::{
    collision::collision_system,
    input::keyboard_input_system,
    round::{randomize_ball_direction, round_system},
    startup::startup_system,
    velocity::velocity_system,
};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::{prelude::*, render::pass::ClearColor};

fn main() {
    App::build()
        // Clear Color is the background color
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Game::default())
        .insert_resource(WindowDescriptor {
            title: "Bevy Pong".to_string(),
            width: 1000.0,
            height: 800.0,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
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
                .with_system(
                    velocity_system
                        .system()
                        .label("velocity")
                        .after("collision"),
                ),
        )
        // .add_system(render_system.system().after("physics"))
        .run();
}
