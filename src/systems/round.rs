use crate::{
    Ball, Game, Player, PlayerType, Position, PrevWinner, Velocity, WallSide, BALL_ORIGIN,
    LEFT_PLAYER_ORIGIN, RIGHT_PLAYER_ORIGIN,
};
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
        commands.entity(ball_entity).remove::<Position>();
        commands.entity(ball_entity).remove::<Transform>();
        commands.entity(ball_entity).remove::<Velocity>();
        println!("REMOVED BALL WALLSIDE, POSITION, TRANSFORM");
        commands.entity(ball_entity).insert(BALL_ORIGIN);
        commands.entity(ball_entity).insert(Transform::from_xyz(
            BALL_ORIGIN.x / 100.0 * window.width,
            BALL_ORIGIN.y / 100.0 * window.height,
            0.0,
        ));

        // Reset position for paddles
        for (player_entity, player) in player_query.iter() {
            println!("REMOVED PLAYER POSITION, TRANSFORM");
            commands.entity(player_entity).remove::<Position>();
            commands.entity(player_entity).remove::<Transform>();
            let origin = match player.player_type {
                PlayerType::Left => LEFT_PLAYER_ORIGIN,
                PlayerType::Right => RIGHT_PLAYER_ORIGIN,
            };
            commands.entity(player_entity).insert(origin.clone());
            commands.entity(player_entity).insert(Transform::from_xyz(
                origin.x / 100.0 * window.width,
                origin.y / 100.0 * window.height,
                0.0,
            ));
        }
    }
}
