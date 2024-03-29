use bevy::{prelude::*, window::PrimaryWindow};

use crate::events::GameOver;
use crate::game::enemy::components::*;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::player::components::*;
use crate::game::score::resources::*;
use crate::game::star::components::*;
use crate::game::star::STAR_SIZE;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; //Player sprite size

pub fn spawn_player(
    mut cmd: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    cmd.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn despawn_player(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        cmd.entity(player_entity).despawn();
    }
}

pub fn player_movement(
    kbd_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if kbd_input.pressed(KeyCode::ArrowLeft) || kbd_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if kbd_input.pressed(KeyCode::ArrowRight) || kbd_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if kbd_input.pressed(KeyCode::ArrowUp) || kbd_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if kbd_input.pressed(KeyCode::ArrowDown) || kbd_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation: Vec3 = transform.translation;
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut cmd: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                cmd.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings::ONCE,
                });
                cmd.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut cmd: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = transform.translation.distance(star_transform.translation);
            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                score.value += 1;
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                cmd.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings::ONCE,
                });
                cmd.entity(star_entity).despawn();
            }
        }
    }
}
