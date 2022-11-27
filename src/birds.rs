use crate::{constants::*, BirdSpawnTimer, Game};
use bevy::{
    math::vec2, prelude::*, render::render_resource::encase::rts_array::Length,
};
use rand::Rng;

#[derive(Default, Clone, Copy, Debug)]
pub struct Bird {
    pub entity: Option<Entity>,
    pub position: Vec2,
    velocity: Vec2,
}

pub fn create_mini_flock(commands: &mut Commands, game: &mut ResMut<Game>) {
    let mut rng = rand::thread_rng();
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

    for _ in 0..MINI_FLOCK_SIZE {
        let bird_spawn_position = Vec2::new(
            spawn_position.x + rng.gen_range(-MINI_FLOCK_PADDING..MINI_FLOCK_PADDING),
            spawn_position.y + rng.gen_range(-MINI_FLOCK_PADDING..MINI_FLOCK_PADDING),
        );

        let new_bird = Bird {
            entity: Some(
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::hex(BIRD_COLOR_HEX).unwrap(),
                            custom_size: Some(Vec2::new(BIRD_SIZE, BIRD_SIZE)),
                            ..default()
                        },
                        ..default()
                    })
                    .id(),
            ),
            position: bird_spawn_position,
            velocity: spawn_velocity,
        };

        game.flock.push(new_bird);
    }
}

pub fn update_birds(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut timer: ResMut<BirdSpawnTimer>,
    mut commands: Commands,
    mut transforms: Query<&mut Transform>,
) {
    // calculate next velocity and position
    let new_flock = game
        .flock
        .iter()
        .map(|bird| {
            let local_flock = get_local_flockmates(bird, &game.flock);

            let separation = separation(bird, &local_flock) * BIRD_SEPARATION_FACTOR;
            let alignment = alignment(bird, &local_flock) * BIRD_ALIGNMENT_FACTOR;
            let cohesion = cohesion(bird, &local_flock) * BIRD_COHESION_FACTOR;
            let recall = recall(bird) * BIRD_RECALL_FACTOR;

            let acceleration = (separation + alignment + cohesion + recall)
                .clamp_length_max(BIRD_MAX_ACCELERATION)
                * time.delta_seconds();

            let velocity = bird.velocity + acceleration;
            let position = bird.position + velocity;

            Bird {
                entity: bird.entity,
                position: position,
                velocity: velocity,
            }
        })
        .collect();

    game.flock = new_flock;

    // update on screen
    for bird in game.flock.iter_mut() {
        // position
        transforms
            .get_mut(bird.entity.unwrap())
            .unwrap()
            .translation = Vec3::new(bird.position.x, bird.position.y, 1.0);
    }

    // add new bird
    if timer.0.tick(time.delta()).just_finished() {
        create_mini_flock(&mut commands, &mut game)
    }
}

fn separation(bird: &Bird, flockmates: &Vec<Bird>) -> Vec2 {
    if flockmates.len() == 0 {
        return vec2(0.0, 0.0);
    }
    let mut v = Vec2::new(0.0, 0.0);
    for mate in flockmates.iter() {
        let diff = bird.position - mate.position;
        v += diff / bird.position.distance(mate.position).powi(2);
    }

    v /= flockmates.length() as f32;

    // steering
    v.normalize_or_zero() * BIRD_MAX_SPEED - bird.velocity
}

fn alignment(bird: &Bird, flockmates: &Vec<Bird>) -> Vec2 {
    if flockmates.len() == 0 {
        return vec2(0.0, 0.0);
    }

    let mut average_flock_velocity = vec2(0.0, 0.0);
    for mate in flockmates.iter() {
        average_flock_velocity += mate.velocity;
    }
    average_flock_velocity /= flockmates.length() as f32;

    // steering
    average_flock_velocity.normalize_or_zero() * BIRD_MAX_SPEED - bird.velocity
}

fn cohesion(bird: &Bird, flockmates: &Vec<Bird>) -> Vec2 {
    if flockmates.len() == 0 {
        return vec2(0.0, 0.0);
    }
    let mut average_flock_position = vec2(0.0, 0.0);
    for mate in flockmates.iter() {
        average_flock_position += mate.position;
    }
    average_flock_position /= flockmates.length() as f32;

    let v = average_flock_position - bird.position;

    v.normalize_or_zero() * BIRD_MAX_SPEED - bird.velocity //steering
}

fn recall(bird: &Bird) -> Vec2 {
    let mut v = vec2(0.0, 0.0);
    let recall_force = 3.0;

    if bird.position.x > RIGHT_BORDER - WINDOW_PADDING {
        v.x = -recall_force;
    }
    if bird.position.x < LEFT_BORDER + WINDOW_PADDING {
        v.x = recall_force;
    }
    if bird.position.y > TOP_BORDER - WINDOW_PADDING {
        v.y = -recall_force;
    }
    if bird.position.y < BOTTOM_BORDER + WINDOW_PADDING {
        v.y = recall_force;
    }
    v
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
