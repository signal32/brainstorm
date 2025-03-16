use bevy::prelude::*;

use super::{
    button_color_sys,
    despawn_entities,
    pause_menu_listener_sys,
    ButtonAction,
    PauseButtonAction,
    GameState,
    MenuContainerNode,
    ButtonNode,
    MenuFont,
    MENU_TEXT_COLOR
};
use super::main_menu::{
    settings_menu_setup_sys,
    OnSettingsMenuScreen,
};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<PauseMenuState>()
        .add_systems(OnEnter(GameState::Pause), pause_setup_sys)
        .add_systems(OnExit(GameState::Pause), despawn_entities::<OnMenuScreen>)
        .add_systems(OnEnter(PauseMenuState::PauseMenu), pause_menu_setup_sys)
        .add_systems(OnExit(PauseMenuState::PauseMenu), despawn_entities::<OnPauseMenuScreen>)
        .add_systems(OnEnter(PauseMenuState::Settings), settings_menu_setup_sys)
        .add_systems(OnExit(PauseMenuState::Settings), despawn_entities::<OnSettingsMenuScreen>)
        .add_systems(Update, (
                button_color_sys,
                pause_button_action_sys,
                pause_menu_listener_sys
            ).run_if(in_state(GameState::Pause))
        );
    }
}

/// Tag Entities with this if they appear on any menu screen
/// 
/// Can be useful to despawn (or otherwise affect)
/// the entire Menu regardless of where you are in it
/// e.g. if you hit Esc while in [`GameState::Menu`] it should despawn all 
/// [`OnMenuScreen`] entities and switch to [`GameState::Game`], which would be difficult to do
/// if we used only [`OnPauseMenuScreen`] and [`OnSettingsMenuScreen`]
#[derive(Component)]
struct OnMenuScreen;
/// Tag Entities with this if they are visible on [PauseMenuState::PauseMenu]
#[derive(Component)]
struct OnPauseMenuScreen;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub(crate) enum PauseMenuState {
    PauseMenu,
    Settings,
    #[default]
    Disabled
}

fn pause_setup_sys(
    mut pause_state: ResMut<NextState<PauseMenuState>>
) {
    pause_state.set(PauseMenuState::PauseMenu);
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
    mut pause_state: ResMut<NextState<PauseMenuState>>,
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
                    pause_state.set(PauseMenuState::Settings);
                    info!("pause state: settings")
                }
                ButtonAction::Pause(PauseButtonAction::QuitToTitle) => {
                    game_state.set(GameState::Menu);
                    pause_state.set(PauseMenuState::Disabled);
                    info!("pause state: disabled and game state: menu!")
                }
                ButtonAction::Pause(PauseButtonAction::Resume) => {
                    game_state.set(GameState::Game);
                    pause_state.set(PauseMenuState::Disabled);
                    info!("pause state: disabled and game state: game!")
                }
                _ => {
                    panic!("You've somehow done something that isn't a menu thing, in the menu.")
                }
            }
        }
    }
}