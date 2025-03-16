use bevy::prelude::*;

use super::{
    button_color_sys,
    despawn_entities,
    pause_menu_listener_sys,
    ButtonAction,
    ButtonNode,
    GameState,
    MenuContainerNode,
    MenuButtonAction,
    MenuFont,
    MENU_TEXT_COLOR
};

use super::pause::PauseMenuState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // TODO: do app.things here
        app
        .init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup_sys)
        .add_systems(OnExit(GameState::Menu), despawn_entities::<OnMenuScreen>)
        .add_systems(OnEnter(MenuState::MainMenu), main_menu_setup_sys)
        .add_systems(OnExit(MenuState::MainMenu), despawn_entities::<OnMainMenuScreen>)
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup_sys)
        .add_systems(OnExit(MenuState::Settings), despawn_entities::<OnSettingsMenuScreen>)
        .add_systems(Update, (
                button_color_sys,
                menu_button_action_sys,
                pause_menu_listener_sys
            ).run_if(in_state(GameState::Menu))
        );
    }
}

/// Tag Entities with this if they appear on any menu screen
/// 
/// Can be useful to despawn (or otherwise affect)
/// the entire Menu regardless of where you are in it
/// e.g. if you hit Esc while in [`GameState::Menu`] it should despawn all 
/// [`OnMenuScreen`] entities and switch to [`GameState::Gam`e], which would be difficult to do
/// if we used only [`OnMainMenuScreen`] and [`OnSettingsMenuScreen`]
#[derive(Component)]
struct OnMenuScreen;
/// Tag Entities with this if they are visible on [MenuState::MainMenu]
#[derive(Component)]
struct OnMainMenuScreen;
/// Tag Entities with this if they are visible on [MenuState::Settings]
#[derive(Component)]
pub(crate) struct OnSettingsMenuScreen;

// / Defines the MenuStates for the Main Menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    MainMenu,
    Settings,
    #[default]
    Disabled
}

fn menu_setup_sys(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::MainMenu);
    info!("menu state: main menu")
}

fn main_menu_setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    let container = MenuContainerNode::spawn(&mut cmd);
    let title_text = (
        Text::new("Bird Invaders"),
        MenuFont::title_font(&asset_server),
        TextColor(MENU_TEXT_COLOR),
        Node {
            margin: UiRect::all(Val::Px(50.0)),
            ..default()
        }
    );
    cmd.entity(container).
    insert((
        OnMenuScreen,
        OnMainMenuScreen
    ))
    .with_children(|parent| {
        parent.spawn(title_text);
    })
    .with_children( |mut parent| {
        ButtonNode::spawn(&mut parent, &asset_server, ButtonAction::Menu(MenuButtonAction::NewGame), "New Game".to_string());
    })
    .with_children( |mut parent| {
        ButtonNode::spawn(&mut parent, &asset_server, ButtonAction::Settings, "Settings".to_string());
    })
    .with_children( |mut parent| {
        ButtonNode::spawn(&mut parent, &asset_server, ButtonAction::Quit, "Quit".to_string());
    });
}

pub(crate) fn settings_menu_setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    let sub_title_text = (
        Text::new("Settings"),
        MenuFont::sub_title_font(&asset_server),
        TextColor(MENU_TEXT_COLOR),
        Node {
            margin: UiRect::all(Val::Px(50.0)),
            ..default()
        }
    );
    let container = MenuContainerNode::spawn(&mut cmd);
    cmd.entity(container).
    insert((
        OnMenuScreen,
        OnSettingsMenuScreen
    ))
    .with_children( |parent| {
        parent.spawn(sub_title_text);
    })
    .with_children( |mut parent| {
        ButtonNode::spawn(&mut parent, &asset_server, ButtonAction::Menu(MenuButtonAction::BackToMenu), "Back to Menu".to_string());
    });
}

/// Defines the actions that should occur on [Button] presses
/// Allows quit, settings, back to main menu, and resume options
/// Add this system to allow menu button actions to occur
fn menu_button_action_sys(
    current_game_state: Res<State<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
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
                    menu_state.set(MenuState::Settings);
                    info!("menu state: settings")
                }
                ButtonAction::Menu(MenuButtonAction::BackToMenu) => {
                    match **current_game_state {
                        GameState::Pause => {
                            // return to pause menu
                            pause_state.set(PauseMenuState::PauseMenu);
                            info!("pause state: pause menu");
                        }
                        GameState::Menu => {
                            // return to main menu
                            menu_state.set(MenuState::MainMenu);
                            info!("menu state: main menu");
                        }
                        _ => {
                            panic!("you have reached something unreachable, trying to go BackToMenu in a GameState that is not Menu or Pause");
                        }
                    }
                }
                ButtonAction::Menu(MenuButtonAction::NewGame) => {
                    game_state.set(GameState::Loading);
                    menu_state.set(MenuState::Disabled);
                    info!("menu state: disabled and game state: game!")
                }
                _ => {
                    panic!("You've somehow done something that isn't a menu thing, in the menu.")
                }
            }
        }
    }
}