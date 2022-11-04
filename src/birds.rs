use crate::{constants::*, Game};
use bevy::{math::vec2, prelude::*};
use rand::Rng;

#[derive(Default, Clone, Copy)]
pub struct Bird {
    entity: Option<Entity>,
    position: Vec2,
    velocity: Vec2,
}

pub fn create_birds(n: usize, commands: &mut Commands, game: &mut ResMut<Game>) {
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let x_rand;
        if rand::random() {
            // left bound
            x_rand = rng.gen_range(LEFT_BORDER - 20.0..LEFT_BORDER);
        } else {
            // right bound
            x_rand = rng.gen_range(RIGHT_BORDER..RIGHT_BORDER + 20.0);
        }

        let y_rand;
        if rand::random() {
            // bottom bound
            y_rand = rng.gen_range(BOTTOM_BORDER - 20.0..BOTTOM_BORDER);
        } else {
            // top bound
            y_rand = rng.gen_range(TOP_BORDER..TOP_BORDER + 20.0);
        }

        let position = vec2(x_rand, y_rand);
        let velocity = vec2(
            game.player.position.x - position.x,
            game.player.position.y - position.y,
        );
        // let position = vec2(
        //     rng.gen_range(LEFT_BORDER..RIGHT_BORDER),
        //     rng.gen_range(BOTTOM_BORDER..TOP_BORDER),
        // );
        // let velocity = vec2(
        //     rng.gen_range(LEFT_BORDER..RIGHT_BORDER),
        //     rng.gen_range(BOTTOM_BORDER..TOP_BORDER),
        // );
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

        game.flock.push(new_bird);
    }
}

pub fn move_birds(mut game: ResMut<Game>, mut transforms: Query<&mut Transform>) {
    // calculate next velocity and position
    let new_flock = game
        .flock
        .iter()
        .map(|bird| {
            let local_flock = get_local_flockmates(bird, &game.flock);

            let separation = separation(bird, &local_flock) * BIRD_SEPARATION_FACTOR;
            println!("separation: {}", separation);
            let alignment = alignment(bird, &local_flock) * BIRD_ALIGNMENT_FACTOR;
            println!("alignment: {}", alignment);
            let cohesion = cohesion(bird, &local_flock) * BIRD_COHESION_FACTOR;
            println!("cohesion: {}", cohesion);
            let attack = (game.player.position - bird.position) * BIRD_ATTACK_FACTOR;

            let mut velocity =
                bird.velocity + (separation + alignment + cohesion + attack) * TIME_STEP;

            // limit velocity to a max
            velocity = velocity.clamp_length_max(BIRD_MAX_SPEED * TIME_STEP);

            let position = bird.position + velocity * 0.5;

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

fn separation(bird: &Bird, flockmates: &Vec<Bird>) -> Vec2 {
    let mut v = Vec2::new(0.0, 0.0);
    for mate in flockmates.iter() {
        v = v + (bird.position - mate.position)
    }
    v
}
fn alignment(bird: &Bird, flockmates: &Vec<Bird>) -> Vec2 {
    if flockmates.len() == 0 {
        return Vec2::new(0.0, 0.0);
    }
    let mut local_flock_velocity = Vec2::new(0.0, 0.0);
    for mate in flockmates.iter() {
        local_flock_velocity = (local_flock_velocity + mate.velocity) / flockmates.len() as f32;
    }
    local_flock_velocity
}
fn cohesion(bird: &Bird, flockmates: &Vec<Bird>) -> Vec2 {
    if flockmates.len() == 0 {
        return Vec2::new(0.0, 0.0);
    }
    let mut local_flock_center = Vec2::new(0.0, 0.0);
    for mate in flockmates.iter() {
        local_flock_center = local_flock_center + mate.position;
    }
    (local_flock_center / (flockmates.len() as f32)) - bird.position
}

fn get_local_flockmates(bird: &Bird, flock: &Vec<Bird>) -> Vec<Bird> {
    flock
        .iter()
        .map(|&b| b)
        .filter(|flock_bird| {
            if flock_bird.entity == bird.entity {
                return false;
            }
            let distance = flock_bird.position.distance(bird.position);
            return distance < BIRD_VIEW_DISTANCE;
        })
        .collect()
}
