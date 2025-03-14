use std::path::{Path, PathBuf};
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use crate::{physics::{Collider, Velocity}, Bird};

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BirdAsset>();
        app.init_asset_loader::<BirdAssetLoader>();
        app.add_systems(FixedPostUpdate, load_bird_assets_sys);
    }
}


/// Denotes an entity as being a bird, but loaded from a file.
/// [load_bird_assets_sys] will attempt to load the asset path and
/// on success add the [Bird] and other required components to the containing entity
#[derive(Debug, Component)]
pub struct BirdAssetHandle(pub Handle<BirdAsset>);

fn load_bird_assets_sys(
    mut cmd: Commands,
    bird_assets: Query<(Entity, &BirdAssetHandle), Without<Bird>>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BirdAsset>>,
) {
    for (entity, bird_asset_path) in bird_assets.iter() {
        if let Some(asset) = assets.get(&bird_asset_path.0) {
            cmd.entity(entity).insert_if_new((
                Bird {
                    name: asset.name.clone(),
                    hunger: asset.hunger,
                },
                Velocity(asset.velocity),
                Collider::Rectangle(Rectangle::from_size(asset.size)),
                Sprite {
                    image: asset_server.load(asset.sprite.clone()),
                    custom_size: Some(asset.size),
                    image_mode: SpriteImageMode::Auto,
                    flip_y: true,
                    ..default()
                },
            ));
        }
    }
}


#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct BirdAsset {
    name: String,
    hunger: i8,
    size: Vec2,
    sprite: PathBuf,
    velocity: f32,
}

#[derive(Default)]
struct BirdAssetLoader;

impl AssetLoader for BirdAssetLoader {
    type Asset = BirdAsset;
    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = ron::de::from_bytes::<BirdAsset>(&bytes)?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
