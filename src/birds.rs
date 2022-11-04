use crate::{constants::*, Game};
use bevy::{math::vec2, prelude::*};

#[derive(Default)]
pub struct Bird {
    entity: Option<Entity>,
    position: Vec2,
    velocity: Vec2,
}

pub fn create_bird(commands: &mut Commands, game: &mut ResMut<Game>) {
    let position = vec2(-500.0, 100.0);
    let velocity = vec2(
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
        position,
        velocity,
    };

    game.flock.push(new_bird)
}

pub fn move_birds(mut game: ResMut<Game>, mut transforms: Query<&mut Transform>) {
    // calculate next velocity and position
    let new_flock = game
        .flock
        .iter()
        .map(|bird| {
            let separation = separation(bird, &game.flock);
            let alignment = alignment(bird, &game.flock);
            let cohesion = cohesion(bird, &game.flock);
            let attack = game.player.position - bird.position;

            let velocity = (bird.velocity + separation + alignment + cohesion + attack) * TIME_STEP;
            let position = bird.position + velocity;

            Bird {
                entity: bird.entity,
                position,
                velocity,
            }
        })
        .collect();

    game.flock = new_flock;

    // update on screen
    for bird in game.flock.iter_mut() {
        // update on screen
        transforms
            .get_mut(bird.entity.unwrap())
            .unwrap()
            .translation = Vec3::new(bird.position.x, bird.position.y, 0.0);
    }
}

fn separation(bird: &Bird, flock: &Vec<Bird>) -> Vec2 {
    Vec2::new(0.0, 0.0)
}
fn alignment(bird: &Bird, flock: &Vec<Bird>) -> Vec2 {
    Vec2::new(0.0, 0.0)
}
fn cohesion(bird: &Bird, flock: &Vec<Bird>) -> Vec2 {
    Vec2::new(0.0, 0.0)
}
