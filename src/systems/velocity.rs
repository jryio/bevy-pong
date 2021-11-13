use crate::{Position, Velocity};
use bevy::prelude::*;

pub fn velocity_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.0.x;
        position.y += velocity.0.y;
    }
}
