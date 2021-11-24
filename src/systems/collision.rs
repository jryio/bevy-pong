use crate::{Ball, Collidable, Player, PlayerType, Velocity, WallSide};
use bevy::{prelude::*, sprite};

type Collision = sprite::collide_aabb::Collision;

#[allow(clippy::type_complexity)]
pub fn collision_system(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &Transform, &Sprite, &mut Velocity, &Ball)>,
    collidables_query: Query<(&Collidable, Option<&WallSide>, &Transform, &Sprite)>,
) {
    if let Ok((ball_entity, ball_transform, ball_sprite, mut ball_velocity, _)) =
        ball_query.single_mut()
    {
        for (collide_type, wallside, collide_transform, collide_sprite) in collidables_query.iter()
        {
            let ball_pos = ball_transform.translation;
            let ball_size = ball_sprite.size;
            let collide_pos = collide_transform.translation;
            let collide_size = collide_sprite.size;
            if let Some(collision) =
                sprite::collide_aabb::collide(ball_pos, ball_size, collide_pos, collide_size)
            {
                match (collide_type, collision, wallside) {
                    (Collidable::Reflect, _, _) => {
                        ball_velocity.0.x *= -1.0;
                        ball_velocity.0.y *= -1.0;
                    }
                    (Collidable::End, _, Some(WallSide::Left)) => {
                        commands.entity(ball_entity).insert(WallSide::Left);
                    }
                    (Collidable::End, _, Some(WallSide::Right)) => {
                        commands.entity(ball_entity).insert(WallSide::Left);
                    }
                    (_, _, None) => (),
                }
            }
        }
    }
}
