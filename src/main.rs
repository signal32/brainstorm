use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (bird_tweet_sys, player_move_sys, bird_move_sys))
        .add_systems(Startup, setup_sys)
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

fn bird_tweet_sys(birds: Query<&Bird>) {
    for bird in birds.iter() {
        bird.tweet();
    }
}

#[derive(Component)]
struct Player {
    health: i32
}

fn setup_sys(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    cmd.spawn(Camera2d);

    cmd.spawn((
        Player { health: 100 },
        Mesh2d(meshes.add(Annulus::new(25.0, 50.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0., -250., 0.),
    ));

    cmd.spawn((
        Bird { name: "greenfinch".to_string(), hunger: 50 },
        Mesh2d(meshes.add(Capsule2d::new(25.0, 50.0))),
        //MeshMaterial2d(materials.add(Color::rgb(150., 0., 0.))),
        MeshMaterial2d(materials.add(Color::linear_rgb(181. / 256., 224. / 256., 120. / 256.))),
        Transform::from_xyz(0., 250., 0.),
    ));
}

fn player_move_sys(
    mut players: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    for mut player_tf in players.iter_mut() {
        if player_tf.translation.x > 600. {
            player_tf.translation.x = -600.
        }
        if player_tf.translation.x < -600. {
            player_tf.translation.x = 600.
        }
        if keys.pressed(KeyCode::KeyA) {
            player_tf.translation.x  -= 10.
        }
        if keys.pressed(KeyCode::KeyD) {
            player_tf.translation.x  += 10.
        }
    }
}

fn bird_move_sys(
    mut birds: Query<&mut Transform, With<Bird>>,
) {
    for mut bird_tf in birds.iter_mut() {
        if bird_tf.translation.y < -1000. {
            bird_tf.translation.y = 1000.
        }
        bird_tf.translation.y -= 2.5
    }
}
