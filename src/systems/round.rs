use crate::{Ball, Game, Player, PlayerType, PrevWinner, Velocity, WallSide};
use bevy::prelude::*;

// 1. Respond when ball collides on left half or right half
// 2. Assign a point to a player
// 3. Restart the round (somehow, probably by calling similar code to the startup system)
pub fn round_system(
    window: Res<WindowDescriptor>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut ball_query: Query<(Entity, &WallSide), With<Ball>>,
    player_query: Query<(Entity, &Player), With<Player>>,
) {
    if let Ok((ball_entity, wall_side_collision)) = ball_query.single_mut() {
        // Increment score
        match wall_side_collision {
            WallSide::Left => {
                game.right_score += 1;
                game.prev_winner = PrevWinner::Player(PlayerType::Right);
            }
            WallSide::Right => {
                game.left_score += 1;
                game.prev_winner = PrevWinner::Player(PlayerType::Left);
            }
        }
        // Remove collision side from ball
        commands.entity(ball_entity).remove::<WallSide>();
        commands.entity(ball_entity).remove::<Transform>();
        commands.entity(ball_entity).remove::<Velocity>();
        // Insert initial state
        commands
            .entity(ball_entity)
            .insert(Transform::from_xyz(0.0, 0.0, 0.0));

        // Reset position for paddles
        for (player_entity, player) in player_query.iter() {
            commands.entity(player_entity).remove::<Transform>();
            // Insert initial state
            commands
                .entity(player_entity)
                .insert(match player.player_type {
                    PlayerType::Left => Transform::from_xyz(-0.425 * window.width, 0.0, 0.0),
                    PlayerType::Right => Transform::from_xyz(0.425 * window.width, 0.0, 0.0),
                });
        }
    }
}
