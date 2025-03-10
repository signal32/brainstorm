use bevy::prelude::*;
use crate::physics::Velocity;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            launch_projectiles_sys,
        ));
    }
}

#[derive(Component)]
struct Projectile {
    // payload:
}

#[derive(Component)]
pub struct ProjectileLauncher {
    pub launch_key: KeyCode,
}

fn launch_projectiles_sys(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    launchers: Query<(&ProjectileLauncher, &Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (launcher, launcher_tf) in launchers.iter() {
        if keys.just_pressed(launcher.launch_key) {
            cmd.spawn((
                Projectile {},
                Velocity(200.),
                launcher_tf.clone(),
                Mesh2d(meshes.add(Circle::new(10.))),
                MeshMaterial2d(materials.add(Color::srgb_u8(127, 0, 100))),
            ));
        }
    }
}
