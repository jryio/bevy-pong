use crate::{Particle, ParticleEmitter, Velocity};
use bevy::prelude::*;
use probability::distribution::Gaussian;
use probability::prelude::*;
use rand::distributions::{Bernoulli, Distribution};
use std::time::Duration;

const PARTICLE_TTL: f32 = 0.5; // seconds

/// emit new particles from entites with the `ParticleEmitter` component
pub fn particle_emission_system(
    mut commands: Commands,
    query: Query<(&ParticleEmitter, &Transform, &Sprite, &Velocity)>,
) {
    for (_particle_emmiter, transform, sprite, velocity) in query.iter() {
        let d = Bernoulli::new(0.3).unwrap();
        if d.sample(&mut rand::thread_rng()) {
            let mut source = source::default();
            let random_direction_dist = Gaussian::new(0.0, 1.0);
            let sampler = Independent(&random_direction_dist, &mut source);
            let samples = sampler.take(2).collect::<Vec<f64>>();
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: sprite.clone(),
                    transform: transform.clone(),
                    ..Default::default()
                })
                .insert(Velocity(Vec2::new(
                    velocity.0.x / 2.0 + samples[0] as f32,
                    velocity.0.y / 2.0 + samples[1] as f32,
                )))
                .insert(Particle {
                    ttl: Timer::new(Duration::from_secs_f32(PARTICLE_TTL), false),
                });
        }
    }
}

/// update particles' time to live
pub fn particle_update_time_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Sprite)>,
) {
    
    for (entity, mut particle, mut sprite) in query.iter_mut() {
        sprite.color.set_a(1. - particle.ttl.elapsed().as_secs_f32() / PARTICLE_TTL);
        if particle.ttl.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
