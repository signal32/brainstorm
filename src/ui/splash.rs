use bevy::prelude::*;
use super::{despawn_entities, GameState};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Splash), splash_setup_sys)
        .add_systems(Update, countdown_sys.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_entities::<OnSplashScreen>);
    }
}

/// Tag entities that are on the splash screen with this
/// i.e. if they occur on [`GameState::Splash`]
#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);


fn splash_setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>
){
    let logo = asset_server.load("menu/splash.png");
    cmd.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnSplashScreen
    ))
    .with_children(|parent| {
        parent.spawn((
            ImageNode::new(logo),
            Node {
                width: Val::Percent(50.0),
                ..default()
            },
        ));
    });
    // Insert the timer as a resource
    cmd.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));

}

fn countdown_sys(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut game_state: ResMut<NextState<GameState>>,
){
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}