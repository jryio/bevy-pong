use crate::{Ball, Collidable, ParticleEmitter, Player, Velocity, WallSide};
use bevy::{prelude::*, sprite};

#[allow(clippy::type_complexity)]
pub fn collision_system(
    mut commands: Commands,
    mut particle_emitter_query: Query<&mut ParticleEmitter>,
    mut ball_query: Query<(Entity, &Transform, &Sprite, &mut Velocity, &Ball)>,
    collidables_query: Query<(
        &Collidable,
        Option<&WallSide>,
        &Transform,
        &Sprite,
        Option<&Player>,
    )>,
) {
    let mut particle_emitter = particle_emitter_query.single_mut();
    if let Ok((ball_entity, ball_transform, ball_sprite, mut ball_velocity, _)) =
        ball_query.get_single_mut()
    {
        for (collide_type, wallside, collide_transform, collide_sprite, player) in
            collidables_query.iter()
        {
            let ball_pos = ball_transform.translation;
            let ball_size = ball_sprite.custom_size.unwrap();
            let collide_pos = collide_transform.translation;
            let collide_size = collide_sprite.custom_size.unwrap();
            if let Some(collision) =
                sprite::collide_aabb::collide(ball_pos, ball_size, collide_pos, collide_size)
            {
                match (collide_type, collision, wallside, player) {
                    (Collidable::Reflect, _, _, Some(_player)) => {
                        ball_velocity.0.x *= -1.0;
                        particle_emitter.ttl.reset();
                    }
                    (Collidable::Reflect, _, _, None) => {
                        ball_velocity.0.y *= -1.0;
                        particle_emitter.ttl.reset();
                    }
                    (Collidable::End, _, Some(WallSide::Left), _) => {
                        commands.entity(ball_entity).insert(WallSide::Left);
                    }
                    (Collidable::End, _, Some(WallSide::Right), _) => {
                        commands.entity(ball_entity).insert(WallSide::Left);
                    }
                    (_, _, None, _) => (),
                }
            }
        }
    }
}
