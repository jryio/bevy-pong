use crate::{Particle, ParticleEmitter, Velocity};
use bevy::prelude::*;
use probability::distribution::Gaussian;
use probability::prelude::*;
use rand::distributions::{Bernoulli, Distribution};
use std::time::Duration;

const PARTICLE_TTL: f32 = 0.5; // seconds
const COLD: (f32, f32, f32) = (251.0 / 255.0, 215.0 / 255.0, 43.0 / 255.0);
const HOT: (f32, f32, f32) = (0.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0);

/// emit new particles from entites with the `ParticleEmitter` component
pub fn particle_emission_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut ParticleEmitter, &Transform, &Sprite, &Velocity)>,
) {
    for (mut particle_emmiter, transform, sprite, velocity) in query.iter_mut() {
        let d =
            Bernoulli::new(particle_emmiter.ttl.tick(time.delta()).percent_left() as f64).unwrap();
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
                    velocity.0.x / 2.5 + samples[0] as f32,
                    velocity.0.y / 2.5 + samples[1] as f32,
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
        let progress =
            0.5 * (3.141 * particle.ttl.elapsed().as_secs_f32() / PARTICLE_TTL + 3.141).cos() + 0.5;
        sprite.color = Color::Rgba {
            red: HOT.0 * progress + COLD.0 * (1.0 - progress),
            green: HOT.1 * progress + COLD.1 * (1.0 - progress),
            blue: HOT.2 * progress + COLD.2 * (1.0 - progress),
            alpha: 1.0 - progress,
        };

        if particle.ttl.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
