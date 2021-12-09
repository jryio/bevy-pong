use bevy::prelude::Vec2;

pub struct Game {
    pub left_score: usize,
    pub right_score: usize,
    pub prev_winner: Option<Player>,
}
impl Default for Game {
    fn default() -> Self {
        Self {
            left_score: 0,
            right_score: 0,
            prev_winner: None,
        }
    }
}
// Player Type
#[derive(Debug)]
pub enum PlayerType {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Player {
    pub player_type: PlayerType,
}

#[derive(Debug)]
pub struct Velocity(pub(crate) Vec2);
pub struct Ball;
pub struct LostRound;

pub enum Collidable {
    Reflect, // Something that is collidable but reflects the ball
    End,     // Something that is collidable but ends the balls movement
}
#[derive(Debug)]
pub enum WallSide {
    Left,
    Right,
}
