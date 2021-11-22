use crate::{Ball, Player, PlayerType, Velocity, WallSide};
use bevy::{prelude::*, sprite};

type Collision = sprite::collide_aabb::Collision;

// TODO: Bevy provides a mechanism to create sub-systems. I imagine we would group together
// multiple different collision systems together in order to compose everything.
pub fn collision_system(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &Transform, &Sprite, &mut Velocity, &Ball)>,
    player_query: Query<(&Transform, &Sprite, &Player)>,
    wall_query: Query<(&Transform, &Sprite, &WallSide)>,
) {
    if let Ok((entity, ball_transform, ball_sprite, mut ball_velocity, _)) = ball_query.single_mut()
    {
        for (wall_transform, wall_size, wall_side) in wall_query.iter() {
            if let Some(side) = wall_collision(
                ball_transform,
                ball_sprite,
                wall_transform,
                wall_size,
                wall_side,
            ) {
                // https://bevy-cheatbook.github.io/programming/commands.html
                commands.entity(entity).insert(side);
                println!("JRY INSERTED COLLISION SIDE TO BALL");
                break;
            }
        }

        println!("JRY BALL POS = {:?}", ball_transform.translation);

        for (player_transform, player_size, player_type) in player_query.iter() {
            paddle_collision(
                ball_transform,
                ball_sprite,
                &mut ball_velocity,
                player_transform,
                player_size,
                player_type,
            );
        }
    }
}

fn wall_collision(
    ball_transform: &Transform,
    ball_sprite: &Sprite,
    wall_transform: &Transform,
    wall_sprite: &Sprite,
    wall_side: &WallSide,
) -> Option<WallSide> {
    let ball_pos = ball_transform.translation;
    let ball_size = ball_sprite.size; //Vec2::from((ball_size.width, ball_size.height));
    let wall_pos = wall_transform.translation;
    let wall_size = wall_sprite.size; // Vec2::from((wall_.size.x, wall_size.size.y));

    // let collision = sprite::collide_aabb::collide(ball_pos, ball_size, wall_pos, wall_size);
    match (
        wall_side,
        sprite::collide_aabb::collide(ball_pos, ball_size, wall_pos, wall_size),
    ) {
        (WallSide::Left, Some(_)) => {
            // println!(
            //     "JRY --------- ball_pos = {:?} wall_pos = {:?} wall_size = {:?}",
            //     ball_pos, wall_pos, wall_size
            // );
            // println!("JRY --------- collision ={:?}", collision);
            Some(WallSide::Left)
        }
        (WallSide::Right, Some(_)) => {
            // println!(
            //     "JRY --------- ball_pos = {:?} wall_pos = {:?} wall_size = {:?}",
            //     ball_pos, wall_pos, wall_size
            // );
            // println!("JRY --------- collision ={:?}", collision);
            Some(WallSide::Right)
        }
        (_, None) => None,
    }
}

fn paddle_collision(
    ball_transform: &Transform,
    ball_sprite: &Sprite,
    ball_velocity: &mut Velocity,
    player_transform: &Transform,
    player_sprite: &Sprite,
    player_type: &Player,
) {
    let ball_pos = ball_transform.translation;
    let ball_size = ball_sprite.size; // Vec2::from((ball_size.width, ball_size.height));
    let player_pos = player_transform.translation;
    let player_size = player_sprite.size; // Vec2::from((player_size.width, player_size.height));
    if let Some(collision) =
        sprite::collide_aabb::collide(ball_pos, ball_size, player_pos, player_size)
    {
        match (player_type, collision) {
            // Right Player
            (
                Player {
                    player_type: PlayerType::Right,
                },
                Collision::Left,
            ) => {
                println!("Ball collided with the right paddle on the left size");
                ball_velocity.0.x *= -1.0;
                ball_velocity.0.y *= -1.0;
            }
            (
                Player {
                    player_type: PlayerType::Right,
                },
                Collision::Top,
            ) => {
                println!("Ball collided with the right paddle on the top size");
                ball_velocity.0.x *= -1.0;
                ball_velocity.0.y *= -1.0;
            }
            (
                Player {
                    player_type: PlayerType::Right,
                },
                Collision::Bottom,
            ) => {
                println!("Ball collided with the right paddle on the bottom size");
                ball_velocity.0.x *= -1.0;
                ball_velocity.0.y *= -1.0;
            }
            (
                Player {
                    player_type: PlayerType::Right,
                },
                _,
            ) => {
                println!("Ball collided with the right paddle on top, right, or bottom");
            }
            // Left Player
            (
                Player {
                    player_type: PlayerType::Left,
                },
                Collision::Right,
            ) => {
                println!("Ball collided with the left paddle on the right side");
                ball_velocity.0.x *= -1.0;
                ball_velocity.0.y *= -1.0;
            }
            (
                Player {
                    player_type: PlayerType::Left,
                },
                Collision::Top,
            ) => {
                println!("Ball collided with the left paddle on the top side");
                ball_velocity.0.x *= -1.0;
                ball_velocity.0.y *= -1.0;
            }
            (
                Player {
                    player_type: PlayerType::Left,
                },
                Collision::Bottom,
            ) => {
                println!("Ball collided with the left paddle on the bottom side");
                ball_velocity.0.x *= -1.0;
                ball_velocity.0.y *= -1.0;
            }
            (
                Player {
                    player_type: PlayerType::Left,
                },
                _,
            ) => {
                println!("Ball collided with the left paddle on the top, left, or bottom")
            }
        }
    }
}
