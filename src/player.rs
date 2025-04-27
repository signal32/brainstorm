use std::path::PathBuf;

use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    GameState,
    level::{LevelAsset, LevelEvent, LevelRootEntity},
    physics::{Collider, ColliderIntersectionMode},
    projectile::ProjectileLauncher,
    util::{AssetHandle, AssetManagerPlugin, EntityAssetReadyEvent},
};

const PLAYER_SPRINT_MULTIPLIER: f32 = 3.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetManagerPlugin::<PlayerAsset>::default());
        app.add_systems(
            Update,
            (
                setup_player_sys,
                on_player_asset_ready_sys,
                player_move_sys.run_if(in_state(GameState::Game)),
            ),
        );
    }
}

#[derive(Debug, Component)]
pub struct Player {
    pub health: i32,
    speed: f32,
}

#[derive(Debug, Component)]
struct PlayerName(String);

#[derive(Debug, Component)]
struct PlayerControls {
    move_left: KeyCode,
    move_right: KeyCode,
    move_up: KeyCode,
    move_down: KeyCode,
    sprint: KeyCode,
    fire: KeyCode,
}

fn setup_player_sys(
    mut cmd: Commands,
    mut level_asset_evts: EventReader<LevelEvent>,
    level_assets: Res<Assets<LevelAsset>>,
    asset_server: Res<AssetServer>,
    root: LevelRootEntity,
) {
    for evt in level_asset_evts.read() {
        match evt {
            &LevelEvent::Loaded { id } => {
                let level = level_assets.get(id).expect("Level should exist");
                let player_count: usize = 1; //TODO get this from game state

                for (player_index, player) in level.players[0..player_count].iter().enumerate() {
                    cmd.entity(*root).with_child((
                        AssetHandle::<PlayerAsset>(asset_server.load(player.asset.clone())),
                        PlayerName(format!("Player {}", player_index + 1)),
                        PlayerControls {
                            move_left: KeyCode::KeyA,
                            move_right: KeyCode::KeyD,
                            move_up: KeyCode::KeyW,
                            move_down: KeyCode::KeyS,
                            sprint: KeyCode::ShiftLeft,
                            fire: KeyCode::Space,
                        },
                        Transform::from_translation(player.initial_position),
                        Collider::Rectangle(Rectangle::new(100., 100.)),
                        ColliderIntersectionMode::None,
                    ));
                }
            }
            _ => (),
        }
    }
}

#[derive(Asset, TypePath, Debug, Deserialize, Default)]
struct PlayerAsset {
    sprite: PathBuf,
    speed: f32,
    health: i32,
}

fn on_player_asset_ready_sys(
    mut cmd: Commands,
    mut asset_ready_evts: EventReader<EntityAssetReadyEvent<PlayerAsset>>,
    assets: Res<Assets<PlayerAsset>>,
    asset_server: Res<AssetServer>,
) {
    for EntityAssetReadyEvent((entities, asset_id)) in asset_ready_evts.read() {
        let asset = assets.get(*asset_id).expect("Asset should exist");
        for entity in entities {
            cmd.entity(*entity).insert((
                Player { health: asset.health, speed: asset.speed },
                ProjectileLauncher { launch_key: KeyCode::Space },
                Sprite {
                    image: asset_server.load(asset.sprite.clone()),
                    custom_size: Some(Vec2::splat(100.)),
                    image_mode: SpriteImageMode::Auto,
                    flip_y: true,
                    ..default()
                },
            ));
        }
    }
}

fn player_move_sys(
    mut players: Query<(&mut Transform, &Player, &PlayerControls)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut player_tf, player, controls) in players.iter_mut() {
        let is_sprinting = keys.pressed(controls.sprint);
        let move_distance =
            if is_sprinting { player.speed * PLAYER_SPRINT_MULTIPLIER } else { player.speed };

        if keys.pressed(controls.move_left) {
            player_tf.translation.x -= move_distance
        }
        if keys.pressed(controls.move_right) {
            player_tf.translation.x += move_distance
        }
        if keys.pressed(controls.move_up) {
            player_tf.translation.y += move_distance;
        }
        if keys.pressed(controls.move_down) {
            player_tf.translation.y -= move_distance;
        }
    }
}
