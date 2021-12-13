use crate::components::PlayerType;
use bevy::prelude::*;

pub trait Controls {
    fn up(&self) -> KeyCode;
    fn down(&self) -> KeyCode;
}

impl Controls for PlayerType {
    fn up(&self) -> KeyCode {
        match self {
            Self::Left => KeyCode::W,
            Self::Right => KeyCode::Up,
        }
    }

    fn down(&self) -> KeyCode {
        match self {
            Self::Left => KeyCode::S,
            Self::Right => KeyCode::Down,
        }
    }
}
