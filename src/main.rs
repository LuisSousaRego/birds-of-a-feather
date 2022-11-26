mod birds;
mod constants;
mod player;

use bevy::prelude::*;
use constants::{BACKGROUND_COLOR, BIRD_SPAWN_SECONDS, WINDOW_HEIGH, WINDOW_WIDTH};

#[derive(Default, Resource)]
pub struct Game {
    player: player::Player,
    flock: Vec<birds::Bird>,
    score: usize,
}

#[derive(Resource)]
pub struct BirdSpawnTimer(Timer);

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(BirdSpawnTimer(Timer::from_seconds(
            BIRD_SPAWN_SECONDS,
            TimerMode::Repeating,
        )))
        .add_startup_system(setup)
        .add_system(player::move_player)
        .add_system(birds::update_birds)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>, mut game: ResMut<Game>) {
    // create camera
    commands.spawn(Camera2dBundle::default());

    // setup window
    let window = windows.get_primary_mut().unwrap();
    window.set_title("Birds of a feather flock together".to_string());
    window.set_resolution(WINDOW_WIDTH, WINDOW_HEIGH);

    // create player
    player::create_player(&mut commands, &mut game);
}
