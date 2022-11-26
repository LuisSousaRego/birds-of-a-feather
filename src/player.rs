use crate::{constants::*, Game};
use bevy::{math::vec2, prelude::*};

#[derive(Default, Debug)]
pub struct Player {
    entity: Option<Entity>,
    pub position: Vec2,
}

pub fn create_player(commands: &mut Commands, game: &mut ResMut<Game>) {
    game.player.entity = Some(
        commands
            .spawn(SpriteBundle {
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
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let mut direction = vec2(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }

    let speed = vec2(direction.x, direction.y) * PLAYER_SPEED * time.delta_seconds();
    let position = game.player.position + speed;

    game.player.position.x = position.x.clamp(LEFT_BORDER, RIGHT_BORDER);
    game.player.position.y = position.y.clamp(BOTTOM_BORDER, TOP_BORDER);

    // update on screen
    transforms
        .get_mut(game.player.entity.unwrap())
        .unwrap()
        .translation = Vec3::new(game.player.position.x, game.player.position.y, 0.0);
}
