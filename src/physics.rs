use bevy::{
    math::bounding::{Aabb2d, Bounded2d, IntersectsVolume}, prelude::*, utils::hashbrown::HashMap
};
use bevy::math::curve::Curve;

use super::GameState;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ColliderContactEvent>();
        app.add_systems(FixedUpdate, (
            velocity_move_sys,
            update_collider_aabb_sys,
            lerp_transform_sys,
        ).run_if(in_state(GameState::Game)));
        app.add_systems(PreUpdate, collider_contact_sys.run_if(in_state(GameState::Game)));
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

fn update_collider_aabb_sys(
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
    moved_colliders: Query<(Entity, &ColliderAabb), Changed<ColliderAabb>>,
    mut contact_evw: EventWriter<ColliderContactEvent>,
) {
    for [
        (a_entity, a_aabb),
        (b_entity, b_aabb)
    ] in moved_colliders.iter_combinations() {
        if let Some((a_aabb, b_aabb)) = a_aabb.0.zip(b_aabb.0) {
            if a_aabb.intersects(&b_aabb) {
                contact_evw.send(ColliderContactEvent { a: a_entity, b: b_entity });
            }
        }
    }
}

#[derive(Event)]
pub struct ColliderContactEvent {
    pub a: Entity,
    pub b: Entity,
}

#[derive(Debug, Component)]
pub struct TargetTransform {
    transform: Transform,
    s: f32,
    easing_curve: EasingCurve<f32>,

    pub lerp_transform: bool,
    pub lerp_rotation: bool,
    pub lerp_scale: bool,
}

impl TargetTransform {
    pub fn new(target: Transform, ease_function: EaseFunction) -> Self {
        TargetTransform {
            transform: target,
            s: 1.1,
            easing_curve: EasingCurve::new(0., 1., ease_function),
            lerp_transform: true,
            lerp_rotation: true,
            lerp_scale: true,
        }
    }

    pub fn update(&mut self, target: Transform) -> &Self {
        self.transform = target;
        self.reset()
    }

    pub fn reset(&mut self) -> &Self {
        self.s = 0.;
        return self
    }

    pub fn transform(&self) -> Transform {
        self.transform
    }

    pub fn sample_s(&self) -> f32 {
        self.easing_curve.sample_clamped(self.s)
    }
}

fn lerp_transform_sys(
    mut transforms_with_targets: Query<(&mut Transform, &mut TargetTransform)>
) {
    for (mut tf, mut target) in transforms_with_targets.iter_mut() {
        if target.s > 1. {
            continue;
        }

        let s = target.sample_s();
        if target.lerp_rotation {
            tf.rotation = tf.rotation.lerp(target.transform.rotation, s);
        }
        if target.lerp_transform {
            tf.translation = tf.translation.lerp(target.transform.translation, s);
        }
        if target.lerp_scale {
            tf.scale = tf.scale.lerp(target.transform.scale, s);
        }

        target.s += 0.01;
    }
}
