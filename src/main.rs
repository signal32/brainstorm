mod projectile;
mod physics;
mod bird;
mod util;
mod level;
mod ui;
mod player;

use std::path::PathBuf;

use bevy::{prelude::*, window::WindowResolution};
use bird::BirdPlugin;
use clap::{Parser, ValueEnum};
use level::LevelPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use projectile::{ProjectileLauncher, ProjectilePlugin};
use ui::UiPlugin;
use util::TransformInterpolationPlugin;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    window_width: Option<f32>,

    #[arg(long)]
    window_height: Option<f32>,

    #[arg(long)]
    window_monitor_index: Option<usize>,

    #[arg(long)]
    level: Option<String>,

    #[arg(long)]
    initial_state: Option<GameState>,

    #[arg(long)]
    debug_render: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let mut app = App::new();
    app.add_plugins((
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
            PhysicsPlugin { debug_render: args.debug_render.unwrap_or_default()},
            ProjectilePlugin,
            BirdPlugin,
            UiPlugin,
            TransformInterpolationPlugin,
            PlayerPlugin,
            match args.level {
                Some(level) => LevelPlugin { default_level: PathBuf::from(level) },
                None => LevelPlugin::default()
            }
    ));
    app.add_systems(Startup, setup_sys);

    app.init_state::<GameState>();
    if let Some(state) = args.initial_state {
        app.insert_state(state);
    }

    app.run();
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, ValueEnum)]
pub(crate) enum GameState {
    Game,
    Pause,
    Menu,
    GameOver,
    #[default]
    Splash,
}


fn setup_sys(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>
) {
    cmd.spawn(Camera2d);

    // cmd.spawn((
    //     Player { health: 100 },
    //     ProjectileLauncher {
    //         launch_key: KeyCode::Space
    //     },
    //     Mesh2d(meshes.add(Annulus::new(25.0, 50.0))),
    //     MeshMaterial2d(materials.add(Color::WHITE)),
    //     Transform::from_xyz(0., - window_height / 2. + 75., 0.),
    // ));
}
