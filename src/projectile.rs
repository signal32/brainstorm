use bevy::prelude::*;
use crate::physics::{Collider, ColliderContactEvent, Velocity};
use super::GameState;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            launch_projectiles_sys,
            projectile_hit_sys,
        ).run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct Projectile {
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
    asset_server: Res<AssetServer>,
) {
    for (launcher, launcher_tf) in launchers.iter() {
        if keys.just_pressed(launcher.launch_key) {
            cmd.spawn((
                Projectile {},
                Velocity(200.),
                Collider::Rectangle(Rectangle::new(100., 10.)),
                launcher_tf.clone(),
                Sprite {
                    image: asset_server.load("sprites/seeds.png"),
                    custom_size: Some(Vec2::splat(32.)),
                    image_mode: SpriteImageMode::Auto,
                    ..default()
                }
            ));
        }
    }
}

fn projectile_hit_sys(
    mut cmd: Commands,
    mut contact_ev: EventReader<ColliderContactEvent>,
    projectiles: Query<(Entity, &Projectile)>,
) {
    for ev in contact_ev.read() {
        if projectiles.get(ev.a).is_ok() {
            cmd.entity(ev.a).despawn()
        }
        if projectiles.get(ev.b).is_ok() {
            cmd.entity(ev.b).despawn()
        }
    }
}
