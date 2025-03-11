use bevy::{
    prelude::*,
    math::bounding::{Aabb2d, Bounded2d, IntersectsVolume},
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            velocity_move_sys,
            collider_contact_sys,
            update_collider_isometry_sys,
        ));
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
    let height = windows.single().height();
    for (entity, mut tf, velocity) in entities.iter_mut() {
        let forward = tf.rotation * Vec3::Y;
        let distance = velocity.0 * time.delta_secs();
        tf.translation += forward * distance;

        if tf.translation.y < - height / 2. - 50. {
            cmd.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
#[require(ColliderAabb)]
pub enum Collider {
    Rectangle(Rectangle)
}

#[derive(Component, Default)]
struct ColliderAabb(Option<Aabb2d>);

fn update_collider_isometry_sys(
    mut colliders: Query<(&mut ColliderAabb, &Collider, &Transform), Changed<Transform>>
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
    moved_colliders: Query<(Entity, &ColliderAabb), Changed<ColliderAabb>>
) {
    for [
        (a_entity, a_aabb),
        (b_entity, b_aabb)
    ] in moved_colliders.iter_combinations() {
        if let Some((a_aabb, b_aabb)) = a_aabb.0.zip(b_aabb.0) {
            if a_aabb.intersects(&b_aabb) {
                info!("COLLISION {}..{}", a_entity, b_entity)
            }
        }
    }
}
