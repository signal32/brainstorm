use std::path::PathBuf;
use bevy::prelude::*;
use serde::Deserialize;
use crate::{
    physics::{Collider, Velocity}, 
    util::{EntityAssetReadyEvent, TargetTransform, AnimationIndices, AnimationTimer}
};
use super::{Bird, BirdHungerBar};

/// Loads asset file and spawns remaining [Bird] components
/// on entities with a [BirdAssetHandle].
pub(super) fn load_bird_assets_sys(
    mut cmd: Commands,
    mut asset_events: EventReader<EntityAssetReadyEvent<BirdAsset>>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BirdAsset>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for EntityAssetReadyEvent((entities, asset_id)) in asset_events.read() {
        let asset = assets.get(*asset_id).expect("asset does not exist");
        for entity in entities {
            let mut target_tf = TargetTransform::new(Transform::IDENTITY, EaseFunction::ExponentialOut);
            target_tf.duration_factor = asset.velocity * 0.015;
            target_tf.lerp_transform = false; // conflicts with movement if enabled
            target_tf.finish();

            // FIXME: if atlas_dimensions are not given, and it gets a sprite sheet with multiple sprites
            // then it just kinda,, displays all of them squished into one sprite
            // instead of the intended behaviour which is to just take the first sprite
            // of the sheet and display that static image

            // columns x rows in the sprite sheet
            let dimensions = asset.atlas_dimensions.unwrap_or(UVec2 { x: 1, y: 1 });
            // layout of the sprite sheet
            let texture_atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                UVec2 { x: 64, y: 32 }, // width x height of each sprite in sheet
                dimensions[0],          // columns
                dimensions[1],          // rows
                None,                   // padding
                None,                   // offset
            ));

            let last = usize::try_from((dimensions[0] * dimensions[1]) - 1).unwrap();
            let animation_indices = AnimationIndices { first: 0, last };

            let mut sprite = Sprite::from_atlas_image(
                asset_server.load(asset.sprite.clone()),
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            );
            sprite.custom_size = Some(asset.size);
            sprite.flip_y = true; // birds render upside down if disabled

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
                    sprite,
                    animation_indices,
                    AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
                    target_tf,
                ))
                .with_child((
                    BirdHungerBar,
                    Transform::from_xyz(asset.size.x * 0.6, 0., 200.),
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
    /// Columns and rows in sprite sheet
    atlas_dimensions: Option<UVec2>,
}
