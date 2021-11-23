use crate::Position;

pub const LEFT_PLAYER_ORIGIN: Position = Position { x: -42.5, y: 0.0 };
pub const RIGHT_PLAYER_ORIGIN: Position = Position { x: 42.5, y: 0.0 };
pub const BALL_ORIGIN: Position = Position { x: 0.0, y: 0.0 };
pub const BALL_SIZE: [f32; 2] = [5.0, 5.0];
pub const PADDLE_HEIGHT: f32 = 36.0;
pub const PADDLE_SIZE: [f32; 2] = [6.0, PADDLE_HEIGHT];
pub const DASH_WIDTH: f32 = 1.0;
pub const DASH_HEIGHT: f32 = 8.0;
pub const DASH_SIZE: [f32; 2] = [DASH_WIDTH, DASH_HEIGHT];
pub const DASH_PADDING: f32 = 20.0;
