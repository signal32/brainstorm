use std::f32::consts::PI;

use bevy::{prelude::*, utils::HashMap};
use rand::Rng;

use crate::level::{Level, LevelAsset};

use super::asset::BirdAssetHandle;

#[derive(Component)]
pub struct BirdSpawner {
    cooldown: f32,
    spawn_probability: f64,
}

/// Spawns birds from spawners.
pub(super) fn bird_spawn_sys(
    spawners: Query<(Entity, &BirdSpawner, &Transform)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
    level_assets: Res<Assets<LevelAsset>>,
    mut last_entity_spawn_time: Local<HashMap<Entity, f32>>,
    mut cmd: Commands,
) {
    let mut rng = rand::rng();
    let level_asset = level_assets.get(&level.level_handle).unwrap();

    for (entity, spawner, spawner_tf) in spawners.iter() {
        let time_now = time.elapsed_secs();
        let last_spawn_time = last_entity_spawn_time
            .entry(entity)
            .or_insert(time.elapsed_secs());
        let cooldown_expired = spawner.cooldown < time_now - *last_spawn_time;
        let do_we_bird_yet = rng.random_bool(spawner.spawn_probability);

        if cooldown_expired && do_we_bird_yet {
            last_entity_spawn_time.insert(entity, time_now);

            // Choose a bird from the level at random based based on its `spawn_probability`
            let mut total_probability= 0.;
            let mut cumulative_probability = vec![];
            for bird in level_asset.birds.iter() {
                total_probability += bird.spawn_probability;
                cumulative_probability.push(total_probability);
            }
            let random_p = rng.random_range(0. .. total_probability);
            let random_index = cumulative_probability.iter().position(|p| &random_p <= p).unwrap_or(0);
            let random_bird = &level_asset.birds[random_index];

            cmd.spawn((
                BirdAssetHandle(asset_server.load(random_bird.asset.as_str())),
                spawner_tf.clone()
            ));
        }

    }
}

/// Spawns bird spawners.
pub(super) fn setup_spawner_sys(
    mut cmd: Commands,
    windows: Query<&Window>
) {
    let window_height = windows.single().height();
    let window_width = windows.single().width();
    let bird_count = 10;
    let bird_padding = 200.;

    for i in 0..bird_count {
        let x = ((window_width - bird_padding) / (bird_count - 1) as f32) * i as f32;

        let mut transform = Transform::from_xyz(
            x - (window_width - bird_padding) * 0.5 ,
            (window_height * 0.5) + 100.,
            50.
        );
        transform.rotate_local_x(PI);

        cmd.spawn((
            BirdSpawner { spawn_probability: 0.001, cooldown: 2. },
            transform,
        ));
    }
}
