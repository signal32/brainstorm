use std::path::PathBuf;

use bevy::{
    color::palettes::css::{GREEN, ORANGE},
    prelude::*,
};
use serde::Deserialize;

use crate::{
    GameState,
    physics::{Collider, ColliderContactEvent, ColliderIntersectionMode, ColliderStatic},
    util::ron_asset_loader::RonAssetLoader,
};

pub struct LevelPlugin {
    pub default_level: PathBuf,
}

impl Default for LevelPlugin {
    fn default() -> Self {
        Self { default_level: PathBuf::from("levels").join("level1.ron") }
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LevelAsset>()
            .init_asset_loader::<RonAssetLoader<LevelAsset>>()
            .insert_resource(Level { default_level_path: self.default_level.clone(), ..default() })
            .add_event::<LevelEvent>()
            .add_systems(Startup, setup_level_plugin_sys)
            .add_systems(OnEnter(GameState::Game), load_level_sys)
            // .add_systems(OnEnter(GameState::Menu), unload_level_sys)
            .add_systems(FixedUpdate, (on_level_load_sys, despawn_entities));
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
    pub spawner_z: f32,
    pub birds: Vec<LevelBird>,
    pub players: Vec<LevelPlayer>,
    pub layers: Vec<LevelLayer>,
    pub scenery: Vec<LevelScenery>,
}

/// Bird used in the level.
#[derive(Debug, Deserialize)]
pub struct LevelBird {
    pub asset: String,
    pub spawn_probability: f32,
}

#[derive(Debug, Deserialize)]
pub struct LevelPlayer {
    pub asset: PathBuf,
    pub initial_position: Vec3,
}

#[derive(Debug, Deserialize)]
pub struct LevelLayer {
    image: PathBuf,
    tiled: bool,
    z: f32,
}

#[derive(Debug, Deserialize)]
pub struct LevelScenery {
    image: PathBuf,
    size: Vec2,
    translation: Vec3,
    collision: bool,
}

/// Wait for current level asset to load then setup game and transition to [GameState::Game] when ready.
fn load_level_sys(mut level: ResMut<Level>, asset_server: Res<AssetServer>) {
    level.level_handle = asset_server.load(level.default_level_path.clone());
}

fn unload_level_sys(mut level: ResMut<Level>) {
    level.level_handle = Handle::default();
}

fn setup_level_plugin_sys(mut cmd: Commands) {
    cmd.spawn((LevelRoot, Transform::default(), InheritedVisibility::default()));
}

#[derive(Component)]
struct Despawner;

/// Marks the loaded levels root [Entity].
///
/// There should only ever be one such entity in the world at any one time.
/// Using [LevelRootEntity] as a SystemParam in queries can help with this.
#[derive(Component)]
pub struct LevelRoot;

/// System Parameter that provides the Level Root Entity.
///
/// Entities belonging to the [Level] should be added as children of the [LevelRootEntity].
/// They will then benefit from automatic cleanup along with the level.
///
/// It is assumed that only one [LevelRoot] exists.
pub type LevelRootEntity<'a> = Single<'a, Entity, With<LevelRoot>>;

fn on_level_load_sys(
    mut cmd: Commands,
    mut level_asset_evts: EventReader<AssetEvent<LevelAsset>>,
    mut level_evtw: EventWriter<LevelEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut level: ResMut<Level>,
    root: LevelRootEntity,
    windows: Query<&Window>,
    levels: Res<Assets<LevelAsset>>,
    asset_server: Res<AssetServer>,
) {
    for evt in level_asset_evts.read() {
        match evt {
            AssetEvent::LoadedWithDependencies { id } => {
                let level_asset = levels.get(*id).expect("Level should exist");
                let mut root_cmds = cmd.entity(*root);

                // Make sure any old level entities have been removed
                root_cmds.try_despawn_descendants();

                // Reset level
                level.score = 0;

                let width = windows.single().width();
                let height = windows.single().height();

                // Hit boxes to prevent player leaving play area
                let play_area_hit_boxes = enclosing_rectangles(width, height);
                for (rect, tf) in play_area_hit_boxes {
                    root_cmds.with_child((
                        Collider::Rectangle(rect),
                        ColliderIntersectionMode::None,
                        ColliderStatic,
                        Transform::from_translation(tf),
                        Mesh2d(meshes.add(rect)),
                        MeshMaterial2d(materials.add(ColorMaterial::from_color(GREEN))),
                    ));
                }

                // Hit boxes to trigger despairing of entities that have left the play area
                let despawn_area_hit_boxes = enclosing_rectangles(width * 2., height * 3.);
                for (rect, tf) in despawn_area_hit_boxes {
                    root_cmds.with_child((
                        Despawner,
                        Collider::Rectangle(rect),
                        ColliderIntersectionMode::None,
                        ColliderStatic,
                        Transform::from_translation(tf),
                        Mesh2d(meshes.add(rect)),
                        MeshMaterial2d(materials.add(ColorMaterial::from_color(ORANGE))),
                    ));
                }

                // Spawn layers
                for layer in &level_asset.layers {
                    root_cmds.with_child((
                        Sprite {
                            image: asset_server.load(layer.image.clone()),
                            image_mode: if layer.tiled {
                                SpriteImageMode::Tiled {
                                    tile_x: true,
                                    tile_y: true,
                                    stretch_value: 2.,
                                }
                            }
                            else {
                                SpriteImageMode::Auto
                            },
                            custom_size: Some(Vec2::new(width, height)),
                            ..default()
                        },
                        Transform::from_xyz(0., 0., layer.z),
                    ));
                }

                // Spawn scenery
                for scenery in &level_asset.scenery {
                    root_cmds.with_children(|builder| {
                        let mut scenery_cmds = builder.spawn((
                            Transform::from_translation(scenery.translation),
                            Sprite {
                                image: asset_server.load(scenery.image.clone()),
                                custom_size: Some(scenery.size),
                                ..default()
                            },
                        ));

                        if scenery.collision {
                            scenery_cmds.insert((
                                Collider::Rectangle(Rectangle::from_size(scenery.size)),
                                ColliderIntersectionMode::None,
                                ColliderStatic,
                            ));
                        }
                    });
                }

                level_evtw.send(LevelEvent::Loaded { id: *id });
            }
            AssetEvent::Unused { id } => {
                info!("Clearing up level");
                level_evtw.send(LevelEvent::Unloaded { id: *id });
                cmd.entity(*root).try_despawn_descendants();
            }
            _ => (),
        }
    }
}

#[derive(Debug, Event)]
pub enum LevelEvent {
    Loaded { id: AssetId<LevelAsset> },
    Unloaded { id: AssetId<LevelAsset> },
}

fn enclosing_rectangles(width: f32, height: f32) -> Vec<(Rectangle, Vec3)> {
    let bb_size = 100.;
    let h_width = (width + bb_size + 1.) * 0.5;
    let h_height = (height + bb_size + 1.) * 0.5;

    vec![
        (Rectangle::new(width, bb_size), Vec3::new(0., h_height, 0.)),
        (Rectangle::new(width, bb_size), Vec3::new(0., -h_height, 0.)),
        (Rectangle::new(bb_size, height + bb_size * 2.), Vec3::new(h_width, 0., 0.)),
        (Rectangle::new(bb_size, height + bb_size * 2.), Vec3::new(-h_width, 0., 0.)),
    ]
}

fn despawn_entities(
    mut cmd: Commands,
    mut collision_evts: EventReader<ColliderContactEvent>,
    despawners: Query<(Entity, &Despawner)>,
) {
    for event in collision_evts.read() {
        if let Some(_) = event.either_entity(&despawners) {
            if !despawners.contains(event.a) {
                cmd.entity(event.a).try_despawn_recursive()
            }
            if !despawners.contains(event.b) {
                cmd.entity(event.b).try_despawn_recursive()
            }
        };
    }
}
