use crate::{Ball, Player, PlayerType, Velocity};
use bevy::{prelude::*, sprite};

type Collision = sprite::collide_aabb::Collision;

pub fn collision_system(
    mut ball_query: Query<(&Transform, &Size, &mut Velocity, &Ball)>,
    player_query: Query<(&Transform, &Size, &Player)>,
) {
    if let Ok((ball_transform, ball_size, mut ball_velocity, _)) = ball_query.single_mut() {
        // TODO: check if the next frame, instead of the current frame, is a collision
        for (player_transform, player_size, player_type) in player_query.iter() {
            let ball_pos = ball_transform.translation;
            let ball_size = Vec2::from((ball_size.width, ball_size.height));
            let player_pos = player_transform.translation;
            let player_size = Vec2::from((player_size.width, player_size.height));
            if let Some(collision) =
                sprite::collide_aabb::collide(ball_pos, ball_size, player_pos, player_size)
            {
                match (player_type, collision) {
                    // Right Player
                    (
                        Player {
                            player_type: PlayerType::RightPlayer,
                        },
                        Collision::Left,
                    ) => {
                        println!("Ball collided with the right paddle on the left size");
                        ball_velocity.0.x *= -1.0;
                        ball_velocity.0.y *= -1.0;
                    }
                    (
                        Player {
                            player_type: PlayerType::RightPlayer,
                        },
                        Collision::Top,
                    ) => {
                        println!("Ball collided with the right paddle on the top size");
                        ball_velocity.0.x *= -1.0;
                        ball_velocity.0.y *= -1.0;
                    }
                    (
                        Player {
                            player_type: PlayerType::RightPlayer,
                        },
                        Collision::Bottom,
                    ) => {
                        println!("Ball collided with the right paddle on the bottom size");
                        ball_velocity.0.x *= -1.0;
                        ball_velocity.0.y *= -1.0;
                    }
                    (
                        Player {
                            player_type: PlayerType::RightPlayer,
                        },
                        _,
                    ) => {
                        println!("Ball collided with the right paddle on top, right, or bottom");
                    }
                    // Left Player
                    (
                        Player {
                            player_type: PlayerType::LeftPlayer,
                        },
                        Collision::Right,
                    ) => {
                        println!("Ball collided with the left paddle on the right side");
                        ball_velocity.0.x *= -1.0;
                        ball_velocity.0.y *= -1.0;
                    }
                    (
                        Player {
                            player_type: PlayerType::LeftPlayer,
                        },
                        Collision::Top,
                    ) => {
                        println!("Ball collided with the left paddle on the top side");
                        ball_velocity.0.x *= -1.0;
                        ball_velocity.0.y *= -1.0;
                    }
                    (
                        Player {
                            player_type: PlayerType::LeftPlayer,
                        },
                        Collision::Bottom,
                    ) => {
                        println!("Ball collided with the left paddle on the bottom side");
                        ball_velocity.0.x *= -1.0;
                        ball_velocity.0.y *= -1.0;
                    }
                    (
                        Player {
                            player_type: PlayerType::LeftPlayer,
                        },
                        _,
                    ) => {
                        println!("Ball collided with the left paddle on the top, left, or bottom")
                    }
                }
            }
        }
    }
}
