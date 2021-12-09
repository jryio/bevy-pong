use crate::{Particle, ParticleEmitter, Velocity};
use bevy::prelude::*;
use probability::distribution::Gaussian;
use probability::prelude::*;
use rand::distributions::{Bernoulli, Distribution};
use std::time::Duration;

/// emit new particles from entites with the `ParticleEmitter` component
pub fn particle_emission_system(
    mut commands: Commands,
    query: Query<(&ParticleEmitter, &Transform, &Sprite, &Velocity)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                    material: materials.add(Color::rgba(1.0, 1.0, 1.0, 1.0).into()),
                    sprite: sprite.clone(),
                    ..Default::default()
                })
                .insert(transform.clone())
                .insert(Velocity(Vec2::new(
                    velocity.0.x + samples[0] as f32,
                    velocity.0.y + samples[1] as f32,
                )))
                .insert(Particle {
                    ttl: Timer::new(Duration::from_secs_f64(0.5), false),
                });
        }
    }
}

/// update particles' time to live
pub fn particle_update_time_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, mut particle, handle) in query.iter_mut() {
        let color = &mut materials.get_mut(handle).unwrap().color;
        color.set_a(1. - particle.ttl.elapsed().as_secs_f32() / 0.5);
        if particle.ttl.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
