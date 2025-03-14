mod projectile;
mod physics;
mod menu;
mod pause;
mod bird;

use bevy::prelude::*;
use bird::BirdPlugin;
use physics::PhysicsPlugin;
use projectile::{ProjectileLauncher, ProjectilePlugin};
use menu::MenuPlugin;
use pause::PausePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugin,
            ProjectilePlugin,
            MenuPlugin,
            PausePlugin,
            BirdPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup_sys)
        //.add_systems(OnEnter(GameState::Game), resume_game_sys)
        .add_systems(Update, (
            player_move_sys.run_if(in_state(GameState::Game)),
            pause_listener_sys.run_if(in_state(GameState::Game)),
        ))
        //.add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
        .run();
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub(crate) enum GameState {
    #[default] // for now i want it to default to game state, i.e. launch straight in with reckless abandon
    Game,
    Pause,
    Menu,
    Splash
}

// a label component to tell us which things are loaded in the Game GameState
#[derive(Component)]
struct OnGameScreen;



#[derive(Component)]
struct Player {
    health: i32
}

fn setup_sys(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>
) {
    let window_height = windows.single().height();

    cmd.spawn(Camera2d);

    cmd.spawn((
        Player { health: 100 },
        ProjectileLauncher {
            launch_key: KeyCode::Space
        },
        Mesh2d(meshes.add(Annulus::new(25.0, 50.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0., - window_height / 2. + 75., 0.),
        OnGameScreen,
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

fn pause_listener_sys(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        // change GameState
        game_state.set(GameState::Pause);
        info!("game state changed to paused!");
    }
}

// stole this directly from an example but it seems a sensible way of removing
// unneeded Entities with a given Component indiscriminantly
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
