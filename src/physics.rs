use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, velocity_move_sys);
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
