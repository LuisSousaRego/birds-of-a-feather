use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGH: f32 = 720.0;

pub const TOP_BORDER: f32 = WINDOW_HEIGH / 2.0;
pub const BOTTOM_BORDER: f32 = -WINDOW_HEIGH / 2.0;
pub const LEFT_BORDER: f32 = -WINDOW_WIDTH / 2.0;
pub const RIGHT_BORDER: f32 = WINDOW_WIDTH / 2.0;

pub const WINDOW_PADDING: f32 = 100.0;

pub const BACKGROUND_COLOR_HEX: &str = "fdf6e3";

pub const PLAYER_SIZE: f32 = 15.0;
pub const PLAYER_SCALE: Vec3 = Vec3::new(PLAYER_SIZE, PLAYER_SIZE, 0.0);
pub const PLAYER_COLOR_HEX: &str = "cb4b16";
pub const PLAYER_SPEED: f32 = 300.0;

pub const BIRD_SIZE: f32 = 5.0;
pub const BIRD_COLOR_HEX: &str = "2aa198";

pub const MINI_FLOCK_SIZE: usize = 5;
pub const MINI_FLOCK_PADDING: f32 = 10.0;

pub const BIRD_SPAWN_SECONDS: f32 = 5.0;

pub const BIRD_VIEW_DISTANCE: f32 = 50.0;

pub const BIRD_SEPARATION_FACTOR: f32 = 1.1;
pub const BIRD_ALIGNMENT_FACTOR: f32 = 1.0;
pub const BIRD_COHESION_FACTOR: f32 = 1.0;
pub const BIRD_RECALL_FACTOR: f32 = 1.0;

pub const BIRD_MAX_SPEED: f32 = 8.0;
pub const BIRD_MAX_ACCELERATION: f32 = 30.0;

pub const TEXT_COLOR_HEX: &str = "93a1a1";
pub const SCORE_COLOR_HEX: &str = "b58900";

pub const FONT_SIZE: f32 = 30.0;