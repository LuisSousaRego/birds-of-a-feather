use crate::{constants::*, Game};
use bevy::prelude::*;

#[derive(Default)]
pub struct Player {
    entity: Option<Entity>,
    pub position: Vec2,
}

pub fn create_player(commands: &mut Commands, game: &mut ResMut<Game>) {
    game.player.entity = Some(
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(game.player.position.x, game.player.position.y, 0.0),
                    scale: PLAYER_SCALE,
                    ..default()
                },
                sprite: Sprite {
                    color: PLAYER_COLOR,
                    ..default()
                },
                ..default()
            })
            .id(),
    );
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let mut horizontal_direction = 0.0;
    let mut vertical_direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        horizontal_direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        horizontal_direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        vertical_direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        vertical_direction -= 1.0;
    }

    let new_x_position = game.player.position.x + horizontal_direction * PLAYER_SPEED * TIME_STEP;
    let new_y_position = game.player.position.y + vertical_direction * PLAYER_SPEED * TIME_STEP;

    game.player.position.x = new_x_position.clamp(LEFT_BORDER, RIGHT_BORDER);
    game.player.position.y = new_y_position.clamp(BOTTOM_BORDER, TOP_BORDER);

    // update on screen
    transforms
        .get_mut(game.player.entity.unwrap())
        .unwrap()
        .translation = Vec3::new(game.player.position.x, game.player.position.y, 0.0);
}
