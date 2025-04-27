use super::asset::BirdAsset;
use crate::{
    level::{Level, LevelAsset, LevelEvent, LevelRootEntity},
    util::AssetHandle
};
use bevy::{platform::collections::HashMap, prelude::*};
use rand::Rng;
use std::f32::consts::PI;

#[derive(Component)]
pub struct BirdSpawner {
    cooldown: f32,
    spawn_probability: f32,
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
    root: LevelRootEntity,
) {
    let mut rng = rand::rng();

    if let Some(level_asset) = level_assets.get(&level.level_handle) {
        for (entity, spawner, spawner_tf) in spawners.iter() {
            let time_now = time.elapsed_secs();
            let last_spawn_time = last_entity_spawn_time
                .entry(entity)
                .or_insert(time.elapsed_secs());
            let cooldown_expired = spawner.cooldown < time_now - *last_spawn_time;
            let do_we_bird_yet = rng.random_bool(spawner.spawn_probability as f64);

            if cooldown_expired && do_we_bird_yet {
                last_entity_spawn_time.insert(entity, time_now);

                // Choose a bird from the level at random based based on its `spawn_probability`
                let mut total_probability = 0.;
                let mut cumulative_probability = vec![];
                for bird in level_asset.birds.iter() {
                    total_probability += bird.spawn_probability;
                    cumulative_probability.push(total_probability);
                }
                let random_p = rng.random_range(0. ..total_probability);
                let random_index = cumulative_probability
                    .iter()
                    .position(|p| &random_p <= p)
                    .unwrap_or(0);
                let random_bird = &level_asset.birds[random_index];

                cmd.entity(*root).with_child((
                    AssetHandle::<BirdAsset>(asset_server.load(random_bird.asset.as_str())),
                    spawner_tf.clone(),
                ));
            }
        }
    }
}

/// Spawns bird spawners.
pub(super) fn setup_spawner_sys(
    mut cmd: Commands,
    mut asset_ev: EventReader<LevelEvent>,
    level_assets: Res<Assets<LevelAsset>>,
    windows: Query<&Window>,
    root: LevelRootEntity,
) {
    let window = windows.single().expect("Application should have a window.");
    let window_height = window.height();
    let window_width = window.width();
    let bird_padding = 200.;

    for ev in asset_ev.read() {
        match ev {
            &LevelEvent::Loaded { id } => {
                let level = level_assets.get(id).expect("No level");

                for i in 0..level.spawner_qty {
                    let x = ((window_width - bird_padding) / (level.spawner_qty - 1) as f32) * i as f32;

                    let mut transform = Transform::from_xyz(
                        x - (window_width - bird_padding) * 0.5,
                        (window_height * 0.5) + 100.,
                        level.spawner_z,
                    );
                    transform.rotate_local_x(PI);
                    cmd.entity(*root).with_child((
                        BirdSpawner {
                            spawn_probability: level.spawn_probability,
                            cooldown: level.spawn_cooldown,
                        },
                        transform,
                    ));
                }
            }
            // todo: handle despawn of level entities
            _ => (),
        };
    }
}
