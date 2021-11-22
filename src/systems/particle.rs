use crate::{Particle, ParticleEmitter, Position};
use bevy::prelude::*;
use std::time::Duration;

/// emit new particles from entites with the `ParticleEmitter` component
pub fn particle_emission_system(
    mut commands: Commands,
    query: Query<(&ParticleEmitter, &Sprite, &Position)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (_particle_emmiter, sprite, position) in query.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                sprite: sprite.clone(),
                ..Default::default()
            })
            .insert(Position { ..*position })
            .insert(Particle {
                ttl: Timer::new(Duration::from_secs_f64(0.5), false),
            });
    }
}

/// update particles' time to live
pub fn particle_update_time_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle)>,
) {
    for (entity, mut particle) in query.iter_mut() {
        if particle.ttl.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
