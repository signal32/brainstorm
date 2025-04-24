use std::path::PathBuf;

use bevy::{math::f32, prelude::*};
use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
};
use serde::Deserialize;

use super::{Bird, asset::BirdAsset};
use crate::{
    level::LevelRootEntity,
    physics::{Collider, ColliderContactEvent, ColliderIntersectionMode, Velocity},
    player::Player,
    util::{AssetHandle, AssetManagerPlugin, EntityAssetReadyEvent},
};

/// Birds occasionally drop things.
/// Those things move until they hit the ground.
/// They stay on the ground for a bit until they are (depending on their type)
/// - Picked up by the player (powerup)
/// - Walked over by the player (poop)
pub struct BirdDroppingPlugin;

impl Plugin for BirdDroppingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetManagerPlugin::<BirdDroppingAsset>::default());
        app.add_systems(
            FixedUpdate,
            (
                load_dropping_sys,
                bird_spawn_dropping_sys,
                dropping_fall_sys,
                dropping_despawn_sys,
                poop_dropping_player_hit_sys,
            ),
        );
    }
}

#[derive(Asset, TypePath, Debug, Deserialize, Default)]
pub struct BirdDroppingAsset {
    sprite: PathBuf,
}

#[derive(Debug, Component)]
struct BirdDropping;

#[derive(Debug, Component)]
struct BirdDroppingOnGround {
    /// Elapsed time in seconds when dropping hit the ground.
    on_ground_time_seconds: f32,
}

fn load_dropping_sys(
    mut cmd: Commands,
    mut asset_events: EventReader<EntityAssetReadyEvent<BirdDroppingAsset>>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BirdDroppingAsset>>,
) {
    for EntityAssetReadyEvent((entities, asset_id)) in asset_events.read() {
        let asset = assets.get(*asset_id).expect("Asset should exist");
        for entity in entities {
            cmd.entity(*entity).clear_children().insert((
                BirdDropping,
                Collider::Rectangle(Rectangle::new(50., 50.)),
                ColliderIntersectionMode::AllowAll,
                Sprite { image: asset_server.load(asset.sprite.clone()), ..default() },
            ));
        }
    }
}

/// Spawns [BirdDropping] at random for each hungry [Bird] in the level.
fn bird_spawn_dropping_sys(
    mut cmd: Commands,
    birds: Query<(&Bird, &Velocity, &Transform, &AssetHandle<BirdAsset>)>,
    level: LevelRootEntity,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BirdAsset>>,
) {
    let mut rng = rand::rng();
    for (bird, velocity, tf, asset_handle) in birds.iter() {
        if bird.hunger == 0 || !rng.random_bool(bird.drop_probability as f64) {
            continue;
        }

        let mut dropping_tf = tf.clone();
        dropping_tf.translation.z -= 1.;

        if let Some(asset) = assets.get(&asset_handle.0)
            && let Some(droppings) = &asset.droppings
        {
            let dist = WeightedIndex::new(droppings.iter().map(|d| d.probability)).unwrap();
            let dropping_index = dist.sample(&mut rng);

            cmd.entity(*level).with_child((
                dropping_tf,
                Velocity(velocity.0),
                AssetHandle::<BirdDroppingAsset>(
                    asset_server.load(droppings[dropping_index].asset.clone()),
                ),
            ));
        }
    }
}

/// Reduces [BirdDropping] velocities until they reach zero.
/// The [Velocity] for spawned droppings initially matches that of the bird that 'dropped' them.
///
/// Once this reaches zero, they are considered to be on the ground.
/// The Z transform is set to that of the ground at the point they have stopped.
fn dropping_fall_sys(
    mut cmd: Commands,
    mut droppings: Query<(Entity, &mut Velocity, &mut Transform), With<BirdDropping>>,
    time: Res<Time>,
) {
    for (entity, mut velocity, mut tf) in droppings.iter_mut() {
        if velocity.0 > 0. {
            velocity.0 = (velocity.0 - 1.).max(0.);

            if velocity.0 == 0. {
                tf.translation.z = 10.; // IDK we should probably set a ground Z value somewhere shared
                cmd.entity(entity)
                    .insert(BirdDroppingOnGround { on_ground_time_seconds: time.elapsed_secs() });
            }
        }
    }
}

fn dropping_despawn_sys(
    mut cmds: Commands,
    droppings: Query<(Entity, &BirdDroppingOnGround)>,
    time: Res<Time>,
) {
    let elapsed = time.elapsed_secs();
    for (entity, dropping) in droppings.iter() {
        if elapsed - dropping.on_ground_time_seconds >= 3. {
            cmds.entity(entity).despawn();
        }
    }
}

fn poop_dropping_player_hit_sys(
    mut cmd: Commands,
    droppings: Query<(&BirdDropping)>,
    mut players: Query<&mut Player>,
    mut contact_ev: EventReader<ColliderContactEvent>,
) {
    for contact in contact_ev.read() {
        if let Some((dropping, player)) = contact.between(&droppings, &players) {
            cmd.entity(dropping).despawn();

            let mut player = players.get_mut(player).unwrap();
            player.health -= 10;
        }
    }
}
