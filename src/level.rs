use std::path::PathBuf;

use bevy::prelude::*;
use serde::Deserialize;

use crate::{util::ron_asset_loader::RonAssetLoader, GameState};

pub struct LevelPlugin {
    pub default_level: PathBuf
}

impl Default for LevelPlugin {
    fn default() -> Self {
        Self { default_level: PathBuf::from("levels").join("level1.ron") }
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<LevelAsset>()
            .init_asset_loader::<RonAssetLoader<LevelAsset>>()
            .insert_resource(Level {
                default_level_path: self.default_level.clone(),
                ..default()
            })
            .add_systems(OnEnter(GameState::Game), load_level_sys);
    }
}

/// State related to current level
#[derive(Debug, Resource, Default)]
pub struct Level {
    /// Current level, loading of which will be triggered by [load_level_sys]
    /// when entering [GameState::Game].
    ///
    /// TODO: would make sense to differentiate current level from next level.
    pub level_handle: Handle<LevelAsset>,
    pub score: u32,
    /// Fallback level if none is given.
    default_level_path: PathBuf,
}

/// External level configuration.
#[derive(Asset, TypePath, Debug, Deserialize, Default)]
pub struct LevelAsset {
    pub spawn_probability: f32,
    pub spawn_cooldown: f32,
    pub spawner_qty: i32,
    pub birds: Vec<LevelBird>
}

/// Bird used in the level.
#[derive(Debug, Deserialize)]
pub struct LevelBird {
    pub asset: String,
    pub spawn_probability: f32
}

/// Wait for current level asset to load then setup game and transition to [GameState::Game] when ready.
fn load_level_sys(
    mut level: ResMut<Level>,
    asset_server: Res<AssetServer>,
) {
    level.level_handle = asset_server.load(level.default_level_path.clone());
}
