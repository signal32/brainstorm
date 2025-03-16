mod projectile;
mod physics;
mod menu;
mod pause;
mod splash;
mod bird;
mod util;
mod level;
mod ui;

use bevy::prelude::*;
use bird::BirdPlugin;
use level::LevelPlugin;
use physics::PhysicsPlugin;
use projectile::{ProjectileLauncher, ProjectilePlugin};
use menu::MenuPlugin; // TODO: remove these once its all handled in UIPlugin
use pause::PausePlugin;
use splash::SplashPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugin,
            ProjectilePlugin,
            MenuPlugin, // TODO: remove these once its all handled in UIPlugin
            PausePlugin,
            SplashPlugin,
            BirdPlugin,
            UIPlugin,
            LevelPlugin::default(),
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup_sys)
        .add_systems(OnExit(GameState::Splash), player_spawn_sys)// this is not a good fix for this,,, a bit janky,, it needs moving into its own PlayerPlugin tbh
        .add_systems(Update, ((
            player_move_sys,
            pause_menu_listener_sys
        ).run_if(in_state(GameState::Game)),
        ))
        .run();
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub(crate) enum GameState {
    Game,
    Pause,
    Menu,
    #[default]
    Splash,
    Loading,
}

// a label component to tell us which things are loaded in the Game GameState
#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct Player {
    health: i32
}

fn setup_sys(
    mut cmd: Commands   
) { 
    cmd.spawn(Camera2d);
}

fn player_spawn_sys(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>
) {
    let window_height = windows.single().height();

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

fn pause_menu_listener_sys(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if keys.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Pause);
        info!("game paused");
    }
}

// fn pause_menu_listener_sys(
//     keys: Res<ButtonInput<KeyCode>>,
//     mut game_state: ResMut<NextState<GameState>>,
//     mut menu_state: ResMut<NextState<MenuState>>
// ) {
//     if keys.just_pressed(KeyCode::Escape) {
//         match game_state {
//             GameState::Game => {
//                 game_state.set(GameState::Pause);
//                 info!("game state changed to paused!");
//             }
//             GameState::Menu => {
//                 match menu_state {
//                     MenuState::MainMenu => {
//                         menu_state.set(MenuState::Disabled);
//                         game_state.set(GameState::Game);
//                         info!("menu state is now diabled, and game state is game");
//                     }
//                     MenuState::Settings => {
//                         menu_state.set(MenuState::MainMenu);
//                         info!("menu state is now main menu");
//                     }
//                     _ => {
//                         panic!("HOW DID WE GET HERE???");
//                     }
//                 }
//             }
//             GameState::Pause => {
//                 info!("THIS NEEDS DOING YAS COME ON");
//             }
//             GameState::Splash => {
//                 // do nothing lol
//                 info!("HAH silly, u can't exit the splash screen, just wait.");
//             }
//         }
//     }
// }

// stole this directly from an example but it seems a sensible way of removing
// unneeded Entities with a given Component indiscriminantly
fn despawn_entities<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
