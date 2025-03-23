use bevy::prelude::*;

pub struct TransformInterpolationPlugin;

impl Plugin for TransformInterpolationPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, interpolate_target_transform_sys);
    }
}

/// Describes the intended [Transform] for an entity.
/// Entities with this component shall have their transform mutated over time
/// until the target transform has been reached by [interpolate_target_transform_sys].
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
            s: 0.,
            easing_curve: EasingCurve::new(0., 1., ease_function),
            lerp_transform: true,
            lerp_rotation: true,
            lerp_scale: true,
        }
    }

    /// Set a new target transform and rest transition to start
    pub fn update(&mut self, target: Transform) -> &Self {
        self.transform = target;
        self.start()
    }

    /// Resets transition to start
    pub fn start(&mut self) -> &Self {
        self.s = 0.;
        return self
    }

    /// Sets transition to end
    pub fn finish(&mut self) -> &Self {
        self.s = 1.1;
        return self
    }

    pub fn transform(&self) -> Transform {
        self.transform
    }

    fn sample_s(&self) -> f32 {
        self.easing_curve.sample_clamped(self.s)
    }
}

fn interpolate_target_transform_sys(
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
