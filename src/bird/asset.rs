use std::path::PathBuf;
use bevy::prelude::*;
use serde::Deserialize;
use crate::{physics::{Collider, Velocity}, util::{EntityAssetReadyEvent, TargetTransform}};
use super::{Bird, BirdHungerBar};

/// Loads asset file and spawns remaining [Bird] components
/// on entities with a [BirdAssetHandle].
pub(super) fn load_bird_assets_sys(
    mut cmd: Commands,
    mut asset_events: EventReader<EntityAssetReadyEvent<BirdAsset>>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BirdAsset>>,
) {

    for EntityAssetReadyEvent((entities, asset_id)) in asset_events.read() {
        let asset = assets.get(*asset_id).expect("asset does not exist");
        for entity in entities {
            let mut target_tf = TargetTransform::new(Transform::IDENTITY, EaseFunction::ExponentialOut);
            target_tf.duration_factor = asset.velocity * 0.015;
            target_tf.lerp_transform = false; // conflicts with movement if enabled
            target_tf.finish();

            cmd.entity(*entity)
                .clear_children()
                .insert((
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
                    target_tf,
                ))
                .with_child((
                    BirdHungerBar,
                    Transform::from_xyz(asset.size.x * 0.6, 0., 2.),
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
