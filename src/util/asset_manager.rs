use crate::util::ron_asset_loader::RonAssetLoader;
use bevy::{prelude::*, utils::HashMap};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// Plugin to manage assets of type T.
///
/// ### Loading
/// The asset type shall be registered with the [App].
/// Assets can be loaded from `.ron` files or any format with bevy built in support.
///
/// ### Management
/// Management is optional.
/// Entities to be managed require the [AssetHandle] component.
///
/// The plugin shall monitor which entities use each asset within [EntityAssetRegistry].
/// An [EntityAssetReadyEvent] will be sent when the asset is loaded or updated.
/// This is designed to alert the entity that the asset it uses has been changed and is now ready for us.
#[derive(Debug, Default)]
pub struct AssetManagerPlugin<T>(PhantomData<T>);

impl<T: Asset + Default + DeserializeOwned> Plugin for AssetManagerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_asset::<T>();
        app.init_asset_loader::<RonAssetLoader<T>>();
        app.init_resource::<EntityAssetRegistry<T>>();
        app.add_event::<EntityAssetReadyEvent<T>>();
        app.add_systems(
            FixedUpdate,
            (
                update_entity_asset_registry_sys::<T>,
                remove_removed_entities_from_registry::<T>,
                send_asset_ready_event_sys::<T>,
                send_asset_ready_when_changed::<T>,
            ),
        );
    }
}

/// Holds an asset handle related to the containing entity.
#[derive(Debug, Component)]
pub struct AssetHandle<T: Asset>(pub Handle<T>);

/// Resource to keep track of any entities associated with each asset.
///
/// An [Entity] is associated with an [Asset] through the [AssetHandle] component.
/// When this component is changed, [update_entity_asset_registry_sys] updates the registry
/// with a mapping between the entity and the asset it references.
///
/// This is helpful for managing entities that depend on an asset.
#[derive(Debug, Default, Resource)]
pub struct EntityAssetRegistry<T: bevy::prelude::Asset> {
    pub handle_entities: HashMap<AssetId<T>, Vec<Entity>>,
    entity_handles: HashMap<Entity, AssetId<T>>,
}

/// Updates the list of entities associated with each [AssetId] whenever an entities [AssetHandle] component is changed/added.
///
/// Also updated a private reverse lookup for each entities current [AssetId].
fn update_entity_asset_registry_sys<T: Asset>(
    mut registry: ResMut<EntityAssetRegistry<T>>,
    changed_handle_entities: Query<(Entity, &AssetHandle<T>), Changed<AssetHandle<T>>>,
) {
    for (entity, handle) in changed_handle_entities.iter() {
        // try to avoid leaking memory when assets are held in perpetuity.
        // remove the entity containing the BirdAssetHandle when the handle is updated (i.e. references a new asset).
        //
        // once the handle is not used by any entities it can be removed from the registry altogether.
        // this will decrease the asset ref count and (assuming it is not used elsewhere) allow bevy to clear it from memory.
        if let Some(entity_asset_id) = registry.entity_handles.get(&entity) {
            let entity_handle = entity_asset_id.clone(); // do not increase the ref count for this!
            if let Some(handle_entities) = registry.handle_entities.get_mut(&entity_handle) {
                if let Some(i) = handle_entities.iter().position(|e| *e == entity) {
                    handle_entities.remove(i);

                    if handle_entities.is_empty() {
                        registry.handle_entities.remove(&entity_handle);
                    }
                }
            }
        }

        // insert the entity as a user of the new reference and add the reverse entity lookup
        registry
            .handle_entities
            .entry(handle.0.id())
            .or_default()
            .push(entity);
        registry.entity_handles.insert(entity, handle.0.id());
    }
}

#[derive(Debug, Event)]
pub struct EntityAssetReadyEvent<T: Asset>(pub (Vec<Entity>, AssetId<T>));

/// Keeps track of entities with changed [AssetHandle] components and sends an [EntityAssetReadyEvent]
/// once the asset the handle points to is ready.
///
/// This could be immediately, in the case of assets already loaded by the [AssetServer],
/// or some time later if it needs to be brought into memory first.
fn send_asset_ready_event_sys<T: Asset>(
    mut asset_ready_evtw: EventWriter<EntityAssetReadyEvent<T>>,
    mut waiting_entities: Local<HashMap<AssetId<T>, Vec<Entity>>>,
    new_entities: Query<(Entity, &AssetHandle<T>), Changed<AssetHandle<T>>>,
    assets: Res<Assets<T>>,
) {
    for (entity, handle) in new_entities.iter() {
        waiting_entities
            .entry(handle.0.id())
            .or_default()
            .push(entity);
    }

    let mut empty_entries = vec![];
    for (asset_id, entities) in waiting_entities.iter() {
        if assets.get(*asset_id).is_some() {
            asset_ready_evtw.send(EntityAssetReadyEvent((entities.clone(), *asset_id)));
            empty_entries.push(asset_id.clone())
        }
    }

    for entry in empty_entries {
        waiting_entities.remove(&entry);
    }
}

/// Sends an [EntityAssetReadyEvent] event when an entity is modified.
/// Allows entities that depend on assets to be rebuilt/updated when the asset is updated.
///
/// TODO When bevy cargo feature `file_watcher` is disabled, this should have no effect and could be disabled.
fn send_asset_ready_when_changed<T: Asset>(
    mut asset_events: EventReader<AssetEvent<T>>,
    mut asset_ready_evtw: EventWriter<EntityAssetReadyEvent<T>>,
    registry: Res<EntityAssetRegistry<T>>,
) {
    for event in asset_events.read() {
        match event {
            AssetEvent::Modified { id } => {
                let entities = registry
                    .handle_entities
                    .get(id)
                    .cloned()
                    .unwrap_or_default();
                asset_ready_evtw.send(EntityAssetReadyEvent((entities, *id)));
            }
            _ => (),
        }
    }
}

/// Removes [Entity] from the [EntityAssetRegistry] when either:
/// - An entity with a [AssetHandle] component is removed from the world
/// - The [AssetHandle] is removed from an entity
///
/// Keeping removed entities in the registry can cause panics in code that
/// expects the entities to exist, so clearing them out is important.
fn remove_removed_entities_from_registry<T: Asset>(
    mut registry: ResMut<EntityAssetRegistry<T>>,
    mut removed: RemovedComponents<AssetHandle<T>>,
) {
    for entity in removed.read() {
        if let Some(&asset_id) = registry.entity_handles.get(&entity) {
            if let Some(eh) = registry.handle_entities.get_mut(&asset_id) {
                let index = eh.iter().position(|e| *e == entity);
                if let Some(index) = index {
                    eh.remove(index);
                }
            }
        }
    }
}
