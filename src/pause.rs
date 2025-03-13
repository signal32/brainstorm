use bevy::{prelude::*};
use std::{path::PathBuf};
use super::{despawn_screen, GameState, UIText};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        // when we enter Pause GameState, spawn in the pause items
        .add_systems(OnEnter(GameState::Pause), pause_setup_sys)
        // while we are in this state, run unpause listener
        .add_systems(Update, unpause_listener_sys)
        // when we leave this state, despawn all the entities that were needed for this screen
        .add_systems(OnExit(GameState::Pause), despawn_screen::<OnPauseScreen>);
    }
}

// this is a tag component, so that we know what is displayed while game is paused
#[derive(Component)]
struct OnPauseScreen;

fn pause_setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("we are paused");
    cmd.spawn((
        UIText,
        Text::new("game paused!!!".to_string()),
        TextFont {
            font: asset_server.load(PathBuf::from("fonts").join("NewHiScore.ttf")),
            font_size: 100.,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            right: Val::Px(5.),
            ..default()
        },
        OnPauseScreen
    ));
}

fn unpause_listener_sys(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Game);
        info!("game state changed to game!");
    }
}