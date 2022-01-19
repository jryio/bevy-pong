use bevy::prelude::{Component, Timer, Vec2};

#[derive(Default, Component)]
pub struct Game {
    pub left_score: usize,
    pub right_score: usize,
    pub prev_winner: Option<Player>,
}

#[derive(Debug, Clone, Component)]
pub enum PlayerType {
    Left,
    Right,
}

#[derive(Debug, Clone, Component)]
pub struct Player {
    pub player_type: PlayerType,
}

#[derive(Debug, Component)]
pub struct Velocity(pub(crate) Vec2);

#[derive(Debug, Component)]
pub struct Ball;

#[derive(Debug, Component)]
pub enum Collidable {
    Reflect, // Something that is collidable but reflects the ball
    End,     // Something that is collidable but ends the balls movement
}

#[derive(Debug, Component)]
pub enum WallSide {
    Left,
    Right,
}

#[derive(Debug, Component)]
pub struct ParticleEmitter;

#[derive(Debug, Component)]
pub struct Particle {
    pub ttl: Timer,
}
