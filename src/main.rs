mod birds;
mod constants;
mod control;
mod player;

use bevy::prelude::*;
use constants::*;

#[derive(Default, Resource)]
pub struct Game {
    player: player::Player,
    flock: Vec<birds::Bird>,
    max_score: usize,
}

#[derive(Resource)]
pub struct BirdSpawnTimer(Timer);

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::hex(BACKGROUND_COLOR_HEX).unwrap()))
        .insert_resource(BirdSpawnTimer(Timer::from_seconds(
            BIRD_SPAWN_SECONDS,
            TimerMode::Repeating,
        )))
        .add_startup_system(setup)
        .add_system(control::check_collisions)
        .add_system(control::update_score)
        .add_system(player::move_player)
        .add_system(birds::update_birds)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // create camera
    commands.spawn(Camera2dBundle::default());

    // setup window
    let window = windows.get_primary_mut().unwrap();
    window.set_title("Birds of a feather flock together".to_string());
    window.set_resolution(WINDOW_WIDTH, WINDOW_HEIGH);

    // create player
    player::create_player(&mut commands, &mut game, &mut meshes, &mut materials);

    // create top score text
    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "max birds: ",
            TextStyle {
                font: asset_server.load("fonts/RussoOne-Regular.ttf"),
                font_size: FONT_SIZE,
                color: Color::hex(TEXT_COLOR_HEX).unwrap(),
            },
        ),
        TextSection::from_style(TextStyle {
            font: asset_server.load("fonts/RussoOne-Regular.ttf"),
            font_size: FONT_SIZE,
            color: Color::hex(SCORE_COLOR_HEX).unwrap(),
        }),
    ]),));
}
