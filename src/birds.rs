use crate::{constants::*, player, Game};
use bevy::{math::vec2, prelude::*};

#[derive(Default)]
pub struct Bird {
    entity: Option<Entity>,
    position: Vec2,
    direction: Vec2,
}

pub fn create_bird(commands: &mut Commands, game: &mut ResMut<Game>) {
    let position = vec2(-500.0, 100.0);
    let direction = vec2(
        game.player.position.x - position.x,
        game.player.position.y - position.y,
    );

    let new_bird = Bird {
        entity: Some(
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(position.x, position.y, 0.0),
                        scale: BIRD_SCALE,
                        ..default()
                    },
                    sprite: Sprite {
                        color: BIRD_COLOR,
                        ..default()
                    },
                    ..default()
                })
                .id(),
        ),
        position: vec2(-100.0, 100.0),
        direction: vec2(-100.0, 100.0),
    };

    game.flock.push(new_bird)
}

pub fn move_birds(mut game: ResMut<Game>) {
    print!("move");
}
