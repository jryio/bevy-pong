use crate::components::Player;
use crate::constants::{PADDLE_HEIGHT, PADDLE_VELOCITY};
use crate::keymaps::Controls;

use bevy::prelude::*;
use std::time::Duration;

pub fn keyboard_input_system(
    mut commands: Commands,
    mut players_query: Query<(&mut Transform, Entity, Option<&mut Timer>, &Player)>,
    time: Res<Time>,
    key: Res<Input<KeyCode>>,
    window: Res<WindowDescriptor>,
) {
    for (mut transform, entity, timer, player) in players_query.iter_mut() {
        let up = player.player_type.up();
        let down = player.player_type.down();

        if key.pressed(up) || key.pressed(down) {
            let v = velocity(&mut commands, entity, &time, timer);
            let direction = match (key.pressed(up), key.pressed(down)) {
                (true, false) => 1.0,
                (false, true) => -1.0,
                _ => 0.0,
            };
            transform.translation.y += v * direction;
        } else if key.just_released(up) || key.just_released(down) {
            println!("JRY -> clear timer -> entity = {:?}", entity);
            commands.entity(entity).remove::<Timer>();
        }

        transform.translation.y = transform
            .translation
            .y
            .max(-(window.height / 2.0) + PADDLE_HEIGHT / 2.0)
            .min((window.height / 2.0) - PADDLE_HEIGHT / 2.0);
    }
}

fn acceleration(time: f32) -> f32 {
    1.0 - (1.0 - time as f32).powi(5) // quintic root at 0
                                      // 1.0 - (1.0 - time as f32).powi(8) // octtic root at 0,2
                                      // 1.0 - (1.0 - time) * (1.0 - time) // quadratic
}

fn velocity(
    commands: &mut Commands,
    entity: Entity,
    time: &Res<Time>,
    timer: Option<Mut<'_, Timer>>,
) -> f32 {
    if let Some(mut timer) = timer {
        timer.tick(time.delta());
        let elapsed = timer.elapsed_secs();
        let d_v = acceleration(elapsed);
        d_v * PADDLE_VELOCITY
    } else {
        let mut timer = Timer::new(Duration::from_secs_f32(1.0), false);
        timer.tick(time.delta());
        let elapsed = timer.elapsed_secs();
        commands.entity(entity).insert(timer);
        let d_v = acceleration(elapsed);
        d_v * PADDLE_VELOCITY
    }
}
