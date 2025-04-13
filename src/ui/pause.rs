use bevy::prelude::*;

use super::{
    button_color_sys,
    despawn_entities,
    pause_menu_listener_sys,
    settings_menu_setup_sys,
    ButtonAction,
    PauseButtonAction,
    GameState,
    MenuContainerNode,
    ButtonNode,
    MenuFont,
    OnMenuScreen,
    OnSettingsMenuScreen,
    MENU_TEXT_COLOR
};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<PauseState>()
        .add_systems(OnEnter(GameState::Pause), pause_setup_sys)
        .add_systems(OnExit(GameState::Pause), despawn_entities::<OnMenuScreen>)
        .add_systems(OnEnter(PauseState::PauseMenu), pause_menu_setup_sys)
        .add_systems(OnExit(PauseState::PauseMenu), despawn_entities::<OnPauseMenuScreen>)
        .add_systems(OnEnter(PauseState::Settings), settings_menu_setup_sys)
        .add_systems(OnExit(PauseState::Settings), despawn_entities::<OnSettingsMenuScreen>)
        .add_systems(Update, (
                button_color_sys,
                pause_button_action_sys,
                pause_menu_listener_sys
            ).run_if(in_state(GameState::Pause))
        );
    }
}

/// Tag Entities with this if they are visible on [PauseState::PauseMenu]
#[derive(Component)]
struct OnPauseMenuScreen;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub(crate) enum PauseState {
    PauseMenu,
    Settings,
    #[default]
    Disabled
}

fn pause_setup_sys(
    mut pause_state: ResMut<NextState<PauseState>>
) {
    pause_state.set(PauseState::PauseMenu);
}

fn pause_menu_setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    let container = MenuContainerNode::spawn(&mut cmd);
    let title_text = (
        Text::new("Game Paused"),
        MenuFont::title_font(&asset_server),
        TextColor(MENU_TEXT_COLOR)
    );
    cmd.entity(container)
    .insert((
        OnMenuScreen,
        OnPauseMenuScreen
    ))
    .with_children(|parent|{
        parent.spawn(title_text);
    })
    .with_children( |mut parent| {
        ButtonNode::spawn(&mut parent, &asset_server, ButtonAction::Pause(PauseButtonAction::Resume), "Resume".to_string());
    })
    .with_children( |mut parent| {
        ButtonNode::spawn(&mut parent, &asset_server, ButtonAction::Settings, "Settings".to_string());
    })
    .with_children( |mut parent| {
        ButtonNode::spawn(&mut parent, &asset_server, ButtonAction::Pause(PauseButtonAction::QuitToTitle), "Quit to Title".to_string());
    })
    .with_children( |mut parent| {
        ButtonNode::spawn(&mut parent, &asset_server, ButtonAction::Quit, "Quit".to_string());
    });
}

/// Defines the actions that should occur on [Button] presses
/// Allows quit, settings, back to pause menu, restart game, and resume options
/// Add this system to allow pause menu button actions to occur
fn pause_button_action_sys (
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut pause_state: ResMut<NextState<PauseState>>,
    interactions: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>)
    >,
) {
    for (interaction, button_action) in &interactions {
        if *interaction == Interaction::Pressed {
            match button_action {
                ButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                ButtonAction::Settings => {
                    pause_state.set(PauseState::Settings);
                    debug!("pause state: settings")
                }
                ButtonAction::Pause(PauseButtonAction::QuitToTitle) => {
                    game_state.set(GameState::Menu);
                    pause_state.set(PauseState::Disabled);
                    debug!("pause state: disabled and game state: menu!")
                }
                ButtonAction::Pause(PauseButtonAction::Resume) => {
                    game_state.set(GameState::Game);
                    pause_state.set(PauseState::Disabled);
                    debug!("pause state: disabled and game state: game!")
                }
                ButtonAction::Pause(PauseButtonAction::BackToMenu) => {
                    pause_state.set(PauseState::PauseMenu);
                    debug!("returning to pause menu. pause state: pausemenu")
                }
                _ => {
                    panic!("You've somehow done something that isn't a pause thing, in the pause menu.")
                }
            }
        }
    }
}