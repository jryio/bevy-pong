use crate::{Player, PlayerType, Position};
use bevy::prelude::*;

// Controls
// ----------------------------------
// Player 1 -> UP -> Up Arrow Key
// Player 1 -> DOWN -> Down Arrow Key
// Player 2 -> UP -> 'W' Key
// Player 2 -> DOWN -> 'S' Key
pub fn keyboard_input_system(
    mut players: Query<(&mut Position, &Player)>,
    key: Res<Input<KeyCode>>,
) {
    for (mut position, player) in players.iter_mut() {
        match player.player_type {
            PlayerType::Left => {
                if key.pressed(KeyCode::W) {
                    position.y += 1.0
                } else if key.pressed(KeyCode::S) {
                    position.y -= 1.0
                }
            }
            PlayerType::Right => {
                if key.pressed(KeyCode::Up) {
                    position.y += 1.0
                } else if key.pressed(KeyCode::Down) {
                    position.y -= 1.0
                }
            }
        }
    }
}
