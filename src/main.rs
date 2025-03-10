mod projectile;
mod physics;

use std::{f32::consts::PI, path::PathBuf};
use bevy::{prelude::*, utils::HashMap};
use physics::{PhysicsPlugin, Velocity};
use projectile::{ProjectileLauncher, ProjectilePlugin};
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugin,
            ProjectilePlugin,
        ))
        .add_event::<BirdSpawnEvent>()
        .add_systems(Startup, setup_sys)
        .add_systems(Update, (
            update_bird_tweet_sys,
            player_move_sys,
            bird_spawn_sys,
        ))
        .run();
}

#[derive(Component)]
struct Bird {
    name: String,
    hunger: i8,
}

impl Bird {
    fn tweet(&self) {
        println!("tweet i am a {}", self.name)
    }
}

fn update_bird_tweet_sys(
    birds: Query<&Bird>,
    mut bird_spawn_ev: EventReader<BirdSpawnEvent>,
    mut ui_texts: Query<&mut Text, With<UIText>>
) {
    for mut text in ui_texts.iter_mut() {
        for BirdSpawnEvent(entity) in bird_spawn_ev.read() {
            if let Ok(bird) = birds.get(*entity) {
                **text = format!("tweet i am a {}", bird.name);
            }
        }
    }
}

#[derive(Component)]
struct Player {
    health: i32
}

#[derive(Component)]
struct UIText;

fn setup_sys(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>
) {
    let window_height = windows.single().height();
    let window_width = windows.single().width();
    let bird_count = 10;
    let bird_padding = 200.;

    cmd.spawn(Camera2d);

    cmd.spawn((
        Player { health: 100 },
        ProjectileLauncher {
            launch_key: KeyCode::Space
        },
        Mesh2d(meshes.add(Annulus::new(25.0, 50.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0., - window_height / 2. + 75., 0.),
    ));

    let mut rng = rand::rng();
    for i in 0..bird_count {
        let x = ((window_width - bird_padding) / (bird_count - 1) as f32) * i as f32;

        let mut transform = Transform::from_xyz(
            x - (window_width - bird_padding) * 0.5 ,
            window_height * 0.5,
            50.
        );
        transform.rotate_local_x(PI);

        cmd.spawn((
            BirdSpawner { spawn_probability: 0.001, cooldown: 2. },
            Mesh2d(meshes.add(Rhombus::new(25.0, 50.0))),
            MeshMaterial2d(materials.add(Color::linear_rgb(
                rng.random_range(0. .. 1.),
                rng.random_range(0. .. 1.),
                rng.random_range(0. .. 1.),
            ))),
            transform,
        ));
    }

    cmd.spawn((
        UIText,
        Text::new("howdy!".to_string()), // initial greeting before any birds show up
        TextFont {
            // lolz we are gonna have issues with the \ / windows/linux issue so i need to get myself on linux right this second lmfao
            font: asset_server.load(PathBuf::from("fonts").join("NewHiScore.ttf")),
            font_size: 50.,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            right: Val::Px(5.),
            ..default()
        }
    ));
}

fn player_move_sys(
    mut players: Query<&mut Transform, With<Player>>,
    windows: Query<&Window>,
    keys: Res<ButtonInput<KeyCode>>
) {
    for mut player_tf in players.iter_mut() {
        let width = windows.single().width();
        let move_distance = if keys.pressed(KeyCode::ShiftLeft) { 25. } else { 10. };

        if keys.pressed(KeyCode::KeyA) {
            player_tf.translation.x = (player_tf.translation.x -move_distance).max(-width / 2.)
        }
        if keys.pressed(KeyCode::KeyD) {
            player_tf.translation.x = (player_tf.translation.x + move_distance).min(width / 2.)
        }
    }
}

#[derive(Component)]
struct BirdSpawner {
    cooldown: f32,
    spawn_probability: f64,
}

#[derive(Event)]
struct BirdSpawnEvent(Entity);

fn bird_spawn_sys(
    spawners: Query<(Entity, &BirdSpawner, &Transform)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut bird_spawn_ev: EventWriter<BirdSpawnEvent>,
    mut last_entity_spawn_time: Local<HashMap<Entity, f32>>,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    let bird_types: Vec<&str> = vec!["greenfinch", "jackdaw", "robin", "swallow", "magpie"];

    for (entity, spawner, spawner_tf) in spawners.iter() {
        let time_now = time.elapsed_secs();
        let last_spawn_time = last_entity_spawn_time
            .entry(entity)
            .or_insert(time.elapsed_secs());
        let cooldown_expired = spawner.cooldown < time_now - *last_spawn_time;
        let do_we_bird_yet = rng.random_bool(spawner.spawn_probability);

        if cooldown_expired && do_we_bird_yet {
            last_entity_spawn_time.insert(entity, time_now);

            let bird_entity = cmd.spawn((
                Bird{
                    name: ( bird_types[rng.random_range(0..bird_types.len())]).to_string(),   // give birds random names
                    hunger: 50,
                },
                Velocity(100.),
                Mesh2d(meshes.add(Capsule2d::new(25.0, 50.0))),
                MeshMaterial2d(materials.add(Color::linear_rgb(
                    rng.random_range(0. .. 1.),
                    rng.random_range(0. .. 1.),
                    rng.random_range(0. .. 1.),
                ))),
                Sprite::from_image(asset_server.load(PathBuf::from("sprites").join("blue_bird.png"))),
                spawner_tf.clone() // birbs will clip into spawners but spawners are only rendered for debugging
            )).id();

            bird_spawn_ev.send(BirdSpawnEvent(bird_entity));
        }

    }
}
