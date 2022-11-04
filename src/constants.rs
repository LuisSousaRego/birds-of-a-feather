use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGH: f32 = 720.0;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub const PLAYER_SCALE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
pub const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const PLAYER_SPEED: f32 = 150.0;

pub const BIRD_SCALE: Vec3 = Vec3::new(5.0, 5.0, 0.0);
pub const BIRD_COLOR: Color = Color::rgb(0.3, 0.7, 0.3);
