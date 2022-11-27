use bevy::{math::vec2, prelude::*};

use crate::{constants::*, Game};

pub fn check_collisions(mut commands: Commands, mut game: ResMut<Game>) {
    for bird in game.flock.iter() {
        let distance_to_player = bird.position.distance(game.player.position);
        if distance_to_player < PLAYER_SIZE / 2.0 {
            // reset game
            for bird in game.flock.iter() {
                // despawn bird
                commands.entity(bird.entity.unwrap()).despawn();
            }
            game.flock = Vec::new();

            game.player.position = vec2(0.0, 0.0);
            break;
        }
    }
}

pub fn update_score(mut game: ResMut<Game>, mut query: Query<&mut Text>) {
    if game.flock.len() > game.max_score {
        game.max_score = game.flock.len();
    }
    for mut text in &mut query {
        text.sections[1].value = format!("{}", game.max_score);
    }
}
