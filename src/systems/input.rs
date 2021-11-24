use crate::constants::{PADDLE_HEIGHT, PADDLE_VELOCITY};
use crate::{Player, PlayerType};
use bevy::prelude::*;

// Controls
// ----------------------------------
// Player 1 -> UP -> Up Arrow Key
// Player 1 -> DOWN -> Down Arrow Key
// Player 2 -> UP -> 'W' Key
// Player 2 -> DOWN -> 'S' Key
pub fn keyboard_input_system(
    mut players: Query<(&mut Transform, &Player)>,
    key: Res<Input<KeyCode>>,
    window: Res<WindowDescriptor>,
) {
    for (mut transform, player) in players.iter_mut() {
        match player.player_type {
            PlayerType::Left => {
                if key.pressed(KeyCode::W) {
                    transform.translation.y += PADDLE_VELOCITY
                } else if key.pressed(KeyCode::S) {
                    transform.translation.y -= PADDLE_VELOCITY
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
