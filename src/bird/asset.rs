use std::path::PathBuf;
use bevy::{
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use crate::{physics::{Collider, Velocity}, util::ron_asset_loader::RonAssetLoader};

use super::{Bird, BirdHungerBar};

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BirdAsset>();
        app.init_asset_loader::<RonAssetLoader<BirdAsset>>();
        app.add_systems(FixedPostUpdate, load_bird_assets_sys);
    }
}


/// Denotes an entity as being a bird, but loaded from a file.
/// [load_bird_assets_sys] will attempt to load the asset path and
/// on success add the [Bird] and other required components to the containing entity
#[derive(Debug, Component)]
pub struct BirdAssetHandle(pub Handle<BirdAsset>);

/// Loads asset file and spawns remaining [Bird] components
/// on entities with a [BirdAssetHandle].
pub(super) fn load_bird_assets_sys(
    mut cmd: Commands,
    bird_assets: Query<(Entity, &BirdAssetHandle), Without<Bird>>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BirdAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, bird_asset_path) in bird_assets.iter() {
        if let Some(asset) = assets.get(&bird_asset_path.0) {
            cmd.entity(entity)
                .insert_if_new((
                    Bird {
                        name: asset.name.clone(),
                        hunger: asset.hunger,
                        initial_hunger: asset.hunger,
                        on_feed_points: asset.on_feed_points,
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
                ))
                .with_child((
                    BirdHungerBar,
                    Transform::from_xyz(asset.size.x * 0.6, 0., 200.),
                    Mesh2d(meshes.add(Rectangle::new(20., 10.))),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                ));
        }
    }
}


#[derive(Asset, TypePath, Debug, Deserialize, Default)]
pub struct BirdAsset {
    name: String,
    hunger: i8,
    size: Vec2,
    sprite: PathBuf,
    velocity: f32,
    on_feed_points: u32,
}
