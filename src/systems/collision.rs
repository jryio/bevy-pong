use crate::{Ball, Player, Velocity};
use bevy::prelude::*;

pub fn collision_system(
    mut ball_query: Query<(&Transform, &mut Velocity, &Ball)>,
    player_query: Query<(&Transform, &Player)>,
) {
    if let Ok((ball_transform, mut ball_velocity, _)) = ball_query.single_mut() {
        // TODO: check if the next frame, instead of the current frame, is a collision
        for (player_transform, _) in player_query.iter() {
            if ball_transform
                .translation
                .distance(player_transform.translation)
                < 100.0
            {
                ball_velocity.0.x *= -1.0;
                ball_velocity.0.y *= -1.0;
            }
        }
    }
}
