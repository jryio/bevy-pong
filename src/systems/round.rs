use crate::constants::*;
use crate::{Ball, Game, Player, PlayerType, Velocity, WallSide};
use bevy::prelude::*;
use rand::Rng;

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
                game.prev_winner = Some(Player {
                    player_type: PlayerType::Right,
                });
            }
            WallSide::Right => {
                game.left_score += 1;
                game.prev_winner = Some(Player {
                    player_type: PlayerType::Left,
                });
            }
        }
        // Remove collision side from ball
        commands.entity(ball_entity).remove::<WallSide>();
        commands.entity(ball_entity).remove::<Transform>();
        commands.entity(ball_entity).remove::<Velocity>();

        // Reset position for paddles
        for (player_entity, player) in player_query.iter() {
            commands.entity(player_entity).remove::<Transform>();
            // Insert initial state
            commands
                .entity(player_entity)
                .insert(match player.player_type {
                    PlayerType::Left => {
                        Transform::from_xyz(-PADDLE_X_OFFSET * window.width, 0.0, 0.0)
                    }
                    PlayerType::Right => {
                        Transform::from_xyz(PADDLE_X_OFFSET * window.width, 0.0, 0.0)
                    }
                });
        }

        let ball_new_pos = {
            let start_pos = {
                let rand_height = gen_rand_height(&window, 0.4 /*offset*/);
                Vec2::new(0.0, rand_height)
            };
            start_pos
        };

        let ball_new_velocity = {
            if let Some(player) = &game.prev_winner {
                let rand_height = gen_rand_height(&window, 0.25 /*offset*/);
                let dest_pos = match player {
                    // Aim towards the Right player if Left won
                    Player {
                        player_type: PlayerType::Left,
                    } => Vec2::new(PADDLE_X_OFFSET * window.height, rand_height),
                    // Aim towards the right player if right won
                    Player {
                        player_type: PlayerType::Right,
                    } => Vec2::new(-PADDLE_X_OFFSET * window.height, rand_height),
                };
                let x = dest_pos[0] - &ball_new_pos[0];
                let y = dest_pos[1] - &ball_new_pos[1];
                let new_pos = Vec2::new(x, y).normalize() * Vec2::new(BALL_SPEED, BALL_SPEED);
                Velocity(new_pos)
            } else {
                Velocity(Vec2::new(1.0, 0.0).normalize() * Vec2::new(BALL_SPEED, BALL_SPEED))
            }
        };

        commands
            .entity(ball_entity)
            .insert(Transform::from_xyz(ball_new_pos.x, ball_new_pos.y, 0.0))
            .insert(ball_new_velocity);
    }
}

fn gen_rand_height(window: &WindowDescriptor, offset: f32) -> f32 {
    let range = offset * window.height;
    let min_height = 0.0 - range;
    let max_height = 0.0 + range;
    let rand_height: f32 = rand::thread_rng().gen_range(min_height..=max_height);
    rand_height
}
