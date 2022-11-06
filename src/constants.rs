use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGH: f32 = 720.0;

pub const TOP_BORDER: f32 = WINDOW_HEIGH / 2.0;
pub const BOTTOM_BORDER: f32 = -WINDOW_HEIGH / 2.0;
pub const LEFT_BORDER: f32 = -WINDOW_WIDTH / 2.0;
pub const RIGHT_BORDER: f32 = WINDOW_WIDTH / 2.0;

pub const WINDOW_PADDING: f32 = 20.0;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub const PLAYER_SCALE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
pub const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const PLAYER_SPEED: f32 = 150.0;

pub const BIRD_SCALE: Vec3 = Vec3::new(5.0, 5.0, 0.0);
pub const BIRD_COLOR: Color = Color::rgb(0.3, 0.7, 0.3);

pub const BIRD_VIEW_DISTANCE: f32 = 50.0;

pub const BIRD_SEPARATION_FACTOR: f32 = 0.5;
pub const BIRD_ALIGNMENT_FACTOR: f32 = 2.0;
pub const BIRD_COHESION_FACTOR: f32 = 0.4;
pub const BIRD_ATTACK_FACTOR: f32 = 5.0;

pub const BIRD_MAX_SPEED: f32 = 300.0;
