use bevy::{
    color::palettes::css::RED,
    ecs::query::{QueryData, QueryFilter},
    math::bounding::{Aabb2d, Bounded2d, IntersectsVolume},
    platform::collections::HashMap,
    prelude::*,
};

use super::GameState;

#[derive(Default)]
pub struct PhysicsPlugin {
    pub debug_render: bool,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ColliderContactEvent>();
        app.add_systems(
            FixedPreUpdate,
            (update_collider_aabb_sys, collider_contact_sys).run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            FixedUpdate,
            velocity_move_sys.run_if(in_state(GameState::Game)),
        );

        if self.debug_render {
            app.add_systems(FixedUpdate, debug_collisions_sys);
        }
    }
}

#[derive(Component)]
pub struct Velocity(pub f32);

fn velocity_move_sys(
    mut cmd: Commands,
    mut entities: Query<(Entity, &mut Transform, &Velocity)>,
    windows: Query<&Window>,
    time: Res<Time>,
) {
    let height = windows.single().expect("Application should have a window").height();
    for (entity, mut tf, velocity) in entities.iter_mut() {
        let forward = tf.rotation * Vec3::Y;
        let distance = velocity.0 * time.delta_secs();
        tf.translation += forward * distance;

        if tf.translation.y < -height / 2. - 50. {
            cmd.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
#[require(ColliderAabb, ColliderIntersectionMode)]
pub enum Collider {
    Rectangle(Rectangle),
}

#[derive(Debug, Component)]
pub struct ColliderStatic;

#[derive(Component, Default)]
struct ColliderAabb(Option<Aabb2d>);

#[derive(Debug, Default, PartialEq, Component)]
pub enum ColliderIntersectionMode {
    /// Can intersect with any other collider.
    #[default]
    AllowAll,
    /// Can intersect with no other colliders
    None,
}

fn update_collider_aabb_sys(
    mut colliders: Query<(&mut ColliderAabb, &Collider, &Transform), Changed<Transform>>,
) {
    for (mut aabb, collider, tf) in colliders.iter_mut() {
        let translation = tf.translation.xy();
        let rotation = tf.rotation.to_euler(EulerRot::YXZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
        aabb.0 = match collider {
            Collider::Rectangle(rectangle) => Some(rectangle.aabb_2d(isometry)),
        }
    }
}

fn collider_contact_sys(
    mut moved_colliders: Query<
        (
            Entity,
            &ColliderAabb,
            &ColliderIntersectionMode,
            &mut Transform,
        ),
        Or<(Changed<ColliderAabb>, With<ColliderStatic>)>,
    >,
    mut contact_evw: EventWriter<ColliderContactEvent>,
    mut pre_collision_transforms: Local<HashMap<Entity, Transform>>,
    mut removed: RemovedComponents<ColliderAabb>,
) {
    let mut collisions = vec![];

    for [(a_entity, a_aabb, a_mode, _), (b_entity, b_aabb, b_mode, _)] in
        moved_colliders.iter_combinations()
    {
        if let Some((a_aabb, b_aabb)) = a_aabb.0.zip(b_aabb.0) {
            if a_aabb.intersects(&b_aabb) {
                contact_evw.send(ColliderContactEvent {
                    a: a_entity,
                    b: b_entity,
                });
                if *a_mode == ColliderIntersectionMode::None
                    && *b_mode == ColliderIntersectionMode::None
                {
                    collisions.push(a_entity);
                    collisions.push(b_entity);
                }
            }
        }
    }

    // Keep track of all entities previous positions and reset them if in a collision
    for (entity, _, _, mut tf) in moved_colliders.iter_mut() {
        if collisions.contains(&entity) {
            if let Some(prev_tf) = pre_collision_transforms.get(&entity) {
                tf.translation = prev_tf.translation;
            }
        } else {
            pre_collision_transforms.insert(entity, tf.clone());
        }
    }

    // Remove removed entities previous positions
    for removed in removed.read() {
        pre_collision_transforms.remove(&removed);
    }
}

fn debug_collisions_sys(
    mut gizmos: Gizmos,
    mut contact_evw: EventReader<ColliderContactEvent>,
    transforms: Query<&Transform>,
) {
    for ColliderContactEvent { a, b } in contact_evw.read() {
        if let Some((a_tf, b_tf)) = transforms.get(*a).ok().zip(transforms.get(*b).ok()) {
            gizmos.line_2d(a_tf.translation.xy(), b_tf.translation.xy(), RED)
        }
    }
}

#[derive(Event)]
pub struct ColliderContactEvent {
    pub a: Entity,
    pub b: Entity,
}

impl ColliderContactEvent {
    // pub fn either<'world, 'state, T>(
    //     &self,
    //     query: &'world Query<'world, 'state, T::ReadOnly>,
    // ) -> Option<<T::ReadOnly as WorldQuery>::Item<'world>>
    // where
    //     T: QueryData,
    // {
    //     if let Ok(b) = query.get(self.a) {
    //         Some(b)
    //     } else if let Ok(b) = query.get(self.b) {
    //         Some(b)
    //     } else {
    //         None
    //     }
    // }

    pub fn either_entity<T>(&self, query: &Query<T>) -> Option<Entity>
    where
        T: QueryData,
    {
        if let Ok(_) = query.get(self.a) {
            Some(self.a)
        } else if let Ok(_) = query.get(self.b) {
            Some(self.b)
        } else {
            None
        }
    }

    pub fn between<'w, 's, D1: QueryData, D2: QueryData, F1: QueryFilter, F2: QueryFilter>(
        &self,
        query_a: &Query<'w, 's, D1, F1>,
        query_b: &Query<'w, 's, D2, F2>,
    ) -> Option<(Entity, Entity)> {
        let entity_a = if query_a.get(self.a).is_ok() {
            Some(self.a)
        } else if query_a.get(self.b).is_ok() {
            Some(self.b)
        } else {
            None
        };
        let entity_b = if query_b.get(self.a).is_ok() {
            Some(self.a)
        } else if query_b.get(self.b).is_ok() {
            Some(self.b)
        } else {
            None
        };

        entity_a.zip(entity_b)
    }
}
