mod projectile;
mod physics;
mod menu;
mod pause;
mod bird;
mod util;
mod level;

use bevy::{prelude::*, window::WindowResolution};
use bird::BirdPlugin;
use clap::Parser;
use level::LevelPlugin;
use physics::PhysicsPlugin;
use projectile::{ProjectileLauncher, ProjectilePlugin};
use menu::MenuPlugin;
use pause::PausePlugin;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    window_width: Option<f32>,

    #[arg(long)]
    window_height: Option<f32>,

    #[arg(long)]
    window_monitor_index: Option<usize>,
}

fn main() {
    let args = Args::parse();
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("Bird Invaders {}", env!("CARGO_PKG_VERSION")),
                        position: WindowPosition::Centered(match args.window_monitor_index {
                            Some(index) => MonitorSelection::Index(index),
                            None => MonitorSelection::Current,
                        }),
                        resolution: WindowResolution::new(
                            args.window_width.unwrap_or(1600.),
                            args.window_height.unwrap_or(900.),
                        ),
                        ..default()
                    }),
                    ..default()
                },),
            PhysicsPlugin,
            ProjectilePlugin,
            MenuPlugin,
            PausePlugin,
            BirdPlugin,
            LevelPlugin::default(),
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup_sys)
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
    #[default]
    Menu,
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
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
