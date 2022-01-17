use bevy::prelude::Vec2;

#[derive(Default)]
pub struct Game {
    pub left_score: usize,
    pub right_score: usize,
    pub prev_winner: Option<Player>,
}

// Player Type
#[derive(Debug, Clone)]
pub enum PlayerType {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub player_type: PlayerType,
}

#[derive(Debug)]
pub struct Velocity(pub(crate) Vec2);
pub struct Ball;
pub enum Collidable {
    Reflect, // Something that is collidable but reflects the ball
    End,     // Something that is collidable but ends the balls movement
}
#[derive(Debug)]
pub enum WallSide {
    Left,
    Right,
}

pub struct ParticleEmitter;
pub struct Particle {
    ttl: Timer,
}