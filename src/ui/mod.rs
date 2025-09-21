pub mod game_over;
pub mod main_menu;
pub mod pause;
pub mod splash;

use super::GameState;
use bevy::prelude::*;
use std::{path::PathBuf, sync::LazyLock};

use game_over::*;
use main_menu::*;
use pause::*;
use splash::*;

// set some color constants -- eventually this can maybe be configurable?
static BUTTON_DEFAULT_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(49, 104, 65));
static BUTTON_HOVER_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(77, 130, 92));
static BUTTON_PRESSED_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(101, 153, 116));
static MENU_BACKGROUND_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgba_u8(64, 64, 64, 196));

const MENU_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const DEFAULT_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugin, PausePlugin, SplashPlugin, GameOverPlugin));
        app.add_systems(
            Update,
            (pause_menu_listener_sys).run_if(in_state(GameState::Game)),
        );
    }
}

/// Tag Entities with this if they appear on any menu screen
///
/// Can be useful to despawn (or otherwise affect)
/// the entire Menu regardless of where you are in it
/// e.g. if you hit Esc while in [`GameState::Menu`] it should despawn all
/// [`OnMenuScreen`] entities and switch to [`GameState::Game`], which would be difficult to do
/// if we used only [`OnMainMenuScreen`] and [`OnSettingsMenuScreen`]

#[derive(Component)]
pub struct OnMenuScreen;

/// Tag Entities with this if they are visible on a settings substate e.g. [MenuState::Settings]
#[derive(Component)]
pub(crate) struct OnSettingsMenuScreen;

/// Enum of all the actions a [Button] should be able to perform,
/// with [MenuButtonAction] and [PauseButtonAction] variants
/// To use the variants:
/// [ButtonAction]::Menu([MenuButtonAction]::NewGame) for example
#[derive(Component)]
pub(crate) enum ButtonAction {
    Menu(MenuButtonAction),
    Pause(PauseButtonAction),
    GameOver(GameOverButtonAction),
    Quit,
    Settings,
}

/// Enum of all actions a menu [Button] should be able to perform
#[derive(Debug)]
pub(crate) enum MenuButtonAction {
    BackToMenu,
    NewGame,
}

/// Enum of all actions a [Button] on the pause menu should be able to perform
#[derive(Debug)]
pub(crate) enum PauseButtonAction {
    BackToMenu,
    QuitToTitle,
    Resume,
}

// Enum of all actions a [Button] on the game over screen should be able to perform
#[derive(Debug)]
pub(crate) enum GameOverButtonAction {
    ReturnToTitle,
    TryAgain,
}

/// ButtonNode! Standardise your buttons with this one cool trick!
///
/// # Usage
/// ```
/// let new_button = ButtonNode::spawn(parent, asset_server, ButtonAction::Action, "Button Text");
/// ```
/// assuming parent is &mut [`ChildBuilder`], and has already been defined
/// So [`ButtonNode`]s are always children of some other [`Entity`]
///
/// # Returns
/// [`Entity`] ID of the newly spawned button.
pub struct ButtonNode;

impl ButtonNode {
    pub fn spawn(
        parent: &mut ChildSpawnerCommands,
        asset_server: &Res<AssetServer>,
        button_action: ButtonAction,
        button_text: String,
    ) -> Entity {
        parent
            .spawn((
                Node {
                    width: Val::Px(500.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Button,
                BackgroundColor(*BUTTON_DEFAULT_COLOR),
                button_action,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(button_text),
                    MenuFont::button_font(asset_server),
                    TextColor(DEFAULT_TEXT_COLOR),
                ));
            })
            .id()
    }
}

/// [`MenuContainerNode`] is a standardised menu screen container,
/// with default settings like displaying items in a vertical column.
/// # Usage
/// ```
/// let new_menu_container = MenuContainerNode::spawn(&mut Commands);
/// ```
pub struct MenuContainerNode;

impl MenuContainerNode {
    pub fn spawn(cmd: &mut Commands) -> Entity {
        cmd.spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },))
            .id()
    }
}

/// Standard font styles for Menu UI
/// # Usage:
/// ```
/// cmd.spawn((
///     Text::new("Text here"),
///     MenuFont::button_font(asset_server)
/// ))
/// ```
pub struct MenuFont;

impl MenuFont {
    pub fn button_font(asset_server: &Res<AssetServer>) -> TextFont {
        TextFont {
            font: asset_server.load(PathBuf::from("fonts").join("NewHiScore.ttf")),
            font_size: 50.,
            ..default()
        }
    }

    pub fn title_font(asset_server: &Res<AssetServer>) -> TextFont {
        TextFont {
            font: asset_server.load(PathBuf::from("fonts").join("NewHiScore.ttf")),
            font_size: 120.,
            ..default()
        }
    }

    pub fn sub_title_font(asset_server: &Res<AssetServer>) -> TextFont {
        TextFont {
            font: asset_server.load(PathBuf::from("fonts").join("NewHiScore.ttf")),
            font_size: 80.,
            ..default()
        }
    }
}

/// On [Interaction] with any [Button], update the colour of it.
/// It has different colours to distinguish between no interaction,
/// hover, and pressed. Uses pre-defined constant colours.
fn button_color_sys(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = (*BUTTON_PRESSED_COLOR).into();
            }
            Interaction::Hovered => {
                *color = (*BUTTON_HOVER_COLOR).into();
            }
            Interaction::None => {
                *color = (*BUTTON_DEFAULT_COLOR).into();
            }
        }
    }
}

pub fn pause_menu_listener_sys(
    keys: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    menu_state: Res<State<MenuState>>,
    pause_state: Res<State<PauseState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match **game_state {
            GameState::Game => {
                next_game_state.set(GameState::Pause);
                debug!("game state changed to paused!");
            }
            GameState::Menu => match **menu_state {
                MenuState::MainMenu => {
                    debug!("Nothing should happen by pressing Esc here");
                }
                MenuState::Settings => {
                    next_menu_state.set(MenuState::MainMenu);
                    debug!("menu state is now main menu");
                }
                _ => {
                    panic!("HOW DID WE GET HERE???");
                }
            },
            GameState::Pause => match **pause_state {
                PauseState::PauseMenu => {
                    next_pause_state.set(PauseState::Disabled);
                    next_game_state.set(GameState::Game);
                    debug!("pause menu: disabled, and game state: game");
                }
                PauseState::Settings => {
                    next_pause_state.set(PauseState::PauseMenu);
                    debug!("pause state: pause menu");
                }
                _ => {
                    panic!("HOW DID WE GET HERE???");
                }
            },
            GameState::Splash => {
                // do nothing lol
                debug!("HAH silly, u can't Esc the splash screen, just wait.");
            }
            _ => {
                panic!("bro how did you even get here");
            }
        }
    }
}

pub(crate) fn settings_menu_setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    current_state: Res<State<GameState>>,
) {
    let sub_title_text = (
        Text::new("Settings"),
        MenuFont::sub_title_font(&asset_server),
        TextColor(MENU_TEXT_COLOR),
        Node {
            margin: UiRect::all(Val::Px(50.0)),
            ..default()
        },
    );
    let button_action;
    match **current_state {
        GameState::Menu => {
            // return to main menu
            button_action = ButtonAction::Menu(MenuButtonAction::BackToMenu);
        }
        GameState::Pause => {
            // return to pause menu
            button_action = ButtonAction::Pause(PauseButtonAction::BackToMenu);
        }
        _ => {
            panic!("we should probably be panicking about this")
        }
    }
    let container = MenuContainerNode::spawn(&mut cmd);
    cmd.entity(container)
        .insert((
            OnMenuScreen,
            OnSettingsMenuScreen,
            BackgroundColor(*MENU_BACKGROUND_COLOR),
        ))
        .with_children(|parent| {
            parent.spawn(sub_title_text);
        })
        .with_children(|mut parent| {
            ButtonNode::spawn(
                &mut parent,
                &asset_server,
                button_action,
                "Back to Menu".to_string(),
            );
        });
}

// stole this directly from an example but it seems a sensible way of removing
// unneeded Entities with a given Component indiscriminantly
fn despawn_entities<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
