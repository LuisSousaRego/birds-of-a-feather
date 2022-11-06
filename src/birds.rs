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
        let side: u8 = rng.gen_range(0..4);
        let spawn_position = match side {
            // top side
            0 => vec2(
                rng.gen_range(LEFT_BORDER - WINDOW_PADDING..RIGHT_BORDER + WINDOW_PADDING),
                rng.gen_range(TOP_BORDER..TOP_BORDER + WINDOW_PADDING),
            ),
            // right side
            1 => vec2(
                rng.gen_range(RIGHT_BORDER..RIGHT_BORDER + WINDOW_PADDING),
                rng.gen_range(BOTTOM_BORDER - WINDOW_PADDING..TOP_BORDER + WINDOW_PADDING),
            ),
            // bottom side
            2 => vec2(
                rng.gen_range(LEFT_BORDER - WINDOW_PADDING..RIGHT_BORDER + WINDOW_PADDING),
                rng.gen_range(BOTTOM_BORDER - WINDOW_PADDING..BOTTOM_BORDER),
            ),
            // left side
            3 => vec2(
                rng.gen_range(LEFT_BORDER - WINDOW_PADDING..LEFT_BORDER),
                rng.gen_range(BOTTOM_BORDER - WINDOW_PADDING..TOP_BORDER + WINDOW_PADDING),
            ),
            _ => vec2(LEFT_BORDER - WINDOW_PADDING, 0.0),
        };

        let spawn_velocity = vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize();

        let new_bird = Bird {
            entity: Some(
                commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(spawn_position.x, spawn_position.y, 0.0),
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
            position: spawn_position,
            velocity: spawn_velocity,
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
            let mut velocity = bird.velocity;

            let local_flock = get_local_flockmates(bird, &game.flock);

            let separation = separation(bird, &local_flock) * BIRD_SEPARATION_FACTOR;
            let alignment = alignment(bird, &local_flock) * BIRD_ALIGNMENT_FACTOR;
            let cohesion = cohesion(bird, &local_flock) * BIRD_COHESION_FACTOR;
            let attack = (game.player.position - bird.position).normalize() * BIRD_ATTACK_FACTOR;

            velocity = velocity + (separation + alignment + cohesion + attack) * TIME_STEP;

            // limit velocity to a max
            velocity = velocity.clamp_length_max(BIRD_MAX_SPEED * TIME_STEP);

            Bird {
                entity: bird.entity,
                position: bird.position + velocity,
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
    if flockmates.len() == 0 {
        return vec2(0.0, 0.0);
    }
    let mut v = Vec2::new(0.0, 0.0);
    for mate in flockmates.iter() {
        v = v + (bird.position - mate.position);
    }
    v / flockmates.len() as f32
}
fn alignment(bird: &Bird, flockmates: &Vec<Bird>) -> Vec2 {
    if flockmates.len() == 0 {
        return vec2(0.0, 0.0);
    }

    let mut local_flock_velocity = bird.velocity;
    for mate in flockmates.iter() {
        local_flock_velocity = local_flock_velocity + mate.velocity;
    }
    local_flock_velocity / (flockmates.len() + 1) as f32
}
fn cohesion(bird: &Bird, flockmates: &Vec<Bird>) -> Vec2 {
    if flockmates.len() == 0 {
        return vec2(0.0, 0.0);
    }
    let mut local_flock_center = bird.position / (flockmates.len() + 1) as f32;
    for mate in flockmates.iter() {
        local_flock_center = local_flock_center + mate.position / (flockmates.len() + 1) as f32;
    }
    local_flock_center - bird.position
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
