use std::path::PathBuf;

use bevy::{prelude::*, text::cosmic_text::Command, utils::HashMap};
use serde::Deserialize;

use crate::{util::ron_asset_loader::RonAssetLoader, GameState};

pub struct LevelPlugin {
    default_level: PathBuf
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
            .add_systems(FixedUpdate, load_level_sys.run_if(in_state(GameState::Loading)));
    }
}

/// State related to current level
#[derive(Debug, Resource, Default)]
pub struct Level {
    /// Current level, will be loaded by [load_level_sys] when [GameState::Loading].
    pub level_handle: Handle<LevelAsset>,
    /// Fallback level if none is given.
    default_level_path: PathBuf,
}

/// External level configuration.
#[derive(Asset, TypePath, Debug, Deserialize, Default)]
pub struct LevelAsset {
    pub spawn_probability: f32,
    pub spawn_cooldown: f32,
    pub birds: Vec<LevelBird>
}

/// Bird used in the level.
#[derive(Debug, Deserialize)]
pub struct LevelBird {
    pub asset: String,
    // pub spawn_probability: f32
}

/// Wait for current level asset to load then setup game and transition to [GameState::Game] when ready.
fn load_level_sys(
    mut level: ResMut<Level>,
    mut game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    level_assets: Res<Assets<LevelAsset>>,
) {
    if asset_server.is_managed(level.level_handle.id()) {
        if  let Some(level_asset) = level_assets.get(&level.level_handle) {
            info!("Level loaded: ${:?}", level_asset);
            game_state.set(GameState::Game);
        }
    }
    else {
        level.level_handle = asset_server.load(level.default_level_path.clone());
    }

}
