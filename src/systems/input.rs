use crate::constants::{PADDLE_HEIGHT, PADDLE_VELOCITY};
use crate::{InputTimer, Player, PlayerType};
use bevy::prelude::*;
use std::time::Duration;

// Controls
// ----------------------------------
// Player 1 -> UP -> Up Arrow Key
// Player 1 -> DOWN -> Down Arrow Key
// Player 2 -> UP -> 'W' Key
// Player 2 -> DOWN -> 'S' Key
pub fn keyboard_input_system(
    mut commands: Commands,
    mut players_query: Query<(&mut Transform, Entity, Option<&mut InputTimer>, &Player)>,
    time: Res<Time>,
    key: Res<Input<KeyCode>>,
    window: Res<WindowDescriptor>,
) {
    for (mut transform, entity, input_timer, player) in players_query.iter_mut() {
        match player.player_type {
            PlayerType::Left => {
                if key.pressed(KeyCode::W) {
                    if let Some(mut t) = input_timer {
                        t.timer.tick(time.delta());
                        let elapsed = t.timer.elapsed_secs();
                        let d_v = get_translation(elapsed);
                        let v = d_v * PADDLE_VELOCITY;
                        transform.translation.y += v
                    } else {
                        let mut timer = Timer::new(Duration::from_secs_f32(1.0), false);
                        commands.entity(entity).insert(InputTimer {
                            timer: timer.clone(),
                        });
                        timer.tick(time.delta());
                        let elapsed = timer.elapsed_secs();
                        let d_v = get_translation(elapsed);
                        let v = d_v * PADDLE_VELOCITY;
                        transform.translation.y += v
                    }
                } else if key.pressed(KeyCode::S) {
                    if let Some(mut t) = input_timer {
                        t.timer.tick(time.delta());
                        let elapsed = t.timer.elapsed_secs();
                        let d_v = get_translation(elapsed);
                        let v = d_v * PADDLE_VELOCITY;
                        transform.translation.y -= v
                    } else {
                        let mut timer = Timer::new(Duration::from_secs_f32(1.0), false);
                        timer.tick(time.delta());
                        commands.entity(entity).insert(InputTimer {
                            timer: timer.clone(),
                        });
                        let elapsed = timer.elapsed_secs();
                        let d_v = get_translation(elapsed);
                        let v = d_v * PADDLE_VELOCITY;
                        transform.translation.y -= v
                    }
                } else if key.just_released(KeyCode::W) {
                    commands.entity(entity).remove::<Timer>();
                } else if key.just_released(KeyCode::S) {
                    commands.entity(entity).remove::<Timer>();
                }
            }
            PlayerType::Right => {
                if key.pressed(KeyCode::Up) {
                    transform.translation.y += PADDLE_VELOCITY
                } else if key.pressed(KeyCode::Down) {
                    transform.translation.y -= PADDLE_VELOCITY
                }
            }
        }
        transform.translation.y = transform
            .translation
            .y
            .max(-(window.height / 2.0) + PADDLE_HEIGHT / 2.0)
            .min((window.height / 2.0) - PADDLE_HEIGHT / 2.0);
    }
}

fn get_translation(time: f32) -> f32 {
    1.0 - (1.0 - time as f32).powi(8)
    // 1.0 - (1.0 - time) * (1.0 - time)
}
