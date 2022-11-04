mod birds;
mod constants;
mod player;

use bevy::prelude::*;
use constants::{BACKGROUND_COLOR, WINDOW_HEIGH, WINDOW_WIDTH};

#[derive(Default)]
pub struct Game {
    player: player::Player,
    flock: Vec<birds::Bird>,
    score: usize,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(player::move_player)
        .add_system(birds::move_birds)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>, mut game: ResMut<Game>) {
    // create camera
    commands.spawn_bundle(Camera2dBundle::default());

    // setup window
    let window = windows.get_primary_mut().unwrap();
    window.set_title("Birds of a feather flock together".to_string());
    window.set_resolution(WINDOW_WIDTH, WINDOW_HEIGH);

    // create player
    player::create_player(&mut commands, &mut game);

    // create birds
    birds::create_birds(400, &mut commands, &mut game);
}
