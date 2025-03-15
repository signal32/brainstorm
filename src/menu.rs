use bevy::{prelude::*};
use std::{path::PathBuf, sync::LazyLock};
use super::{despawn_screen, pause_menu_listener_sys, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        // when we enter Menu GameState, spawn in the menu items
        .init_state::<MenuState>()
        // handle stuff about whether we are in the menu GameState
        .add_systems(OnEnter(GameState::Menu), menu_setup_sys)
        .add_systems(Update, (
            button_color_sys,
//            pause_menu_listener_sys, // uncomment once u certralise the logic for this
            menu_button_action_sys
        ).run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>)
        // handle the Main Menu gubbins
        .add_systems(OnEnter(MenuState::MainMenu), main_menu_setup_sys)
        .add_systems(OnExit(MenuState::MainMenu), despawn_screen::<OnMainMenuScreen>)
        // handle the settings screen gubbins
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup_sys)
        .add_systems(OnExit(MenuState::Settings), despawn_screen::<OnSettingsMenuScreen>);
    }
}

/// Tag Entities with this if they appear on any menu screen
#[derive(Component)]
struct OnMenuScreen;
/// Tag Entities with this if they are visible on [MenuState::MainMenu]
#[derive(Component)]
struct OnMainMenuScreen;
/// Tag Entities with this if they are visible on [MenuState::Settings]
#[derive(Component)]
struct OnSettingsMenuScreen;

// define the menu states
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    MainMenu,
    Settings,
    #[default]
    Disabled
}

// all the actions that a menu button can possibly do
#[derive(Component)]
enum MenuButtonAction {
    Settings,
    BackToMainMenu,
    NewGame,
    Quit
}

// set some color constants -- eventually this can maybe be configurable?
static BUTTON_DEFAULT_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(49, 104, 65));
static BUTTON_HOVER_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(56, 104, 76));
static BUTTON_PRESSED_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(59, 104, 93));

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

/// On [Interaction] with any [Button], update the colour of it.
/// It has different colours to distinguish between no interaction,
/// hover, and pressed. Uses pre-defined constant colours.
fn button_color_sys(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
        ),
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

fn menu_setup_sys(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::MainMenu);
    info!("menu state: main menu")
}

/// Constructs the main menu, spawning in the necessary Entities
fn main_menu_setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    let button_node = Node {
        width: Val::Px(500.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_font = TextFont {
        font: asset_server.load(PathBuf::from("fonts").join("NewHiScore.ttf")),
        font_size: 50.,
        ..default()
    };
    let title_font = TextFont {
        font: asset_server.load(PathBuf::from("fonts").join("NewHiScore.ttf")),
        font_size: 120.,
        ..default()
    };

    cmd.spawn((
        // this is the main bit that encapsulates the whole main menu
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        OnMenuScreen,
        OnMainMenuScreen,
    ))
    .with_children( |parent| {
        // MAIN TITLE
        parent.spawn((
            Text::new("Bird Invaders"),
            title_font.clone(),
            TextColor(TEXT_COLOR),
            Node {
                margin: UiRect::all(Val::Px(50.0)),
                ..default()
            },
        ));
         // NEW GAME BUTTON
        parent.spawn((
            Button,
            button_node.clone(),
            BackgroundColor(*BUTTON_DEFAULT_COLOR),
            MenuButtonAction::NewGame
        ))
        .with_children( |parent| {
            parent.spawn((
                Text::new("Play"),
                button_font.clone(),
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
            ));
        });
        // SETTINGS BUTTON
        parent.spawn((
            Button,
            button_node.clone(),
            BackgroundColor(*BUTTON_DEFAULT_COLOR),
            MenuButtonAction::Settings
        ))
        .with_children( |parent| {
            parent.spawn((
                Text::new("Settings"),
                button_font.clone(),
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
            ));
        });
        // QUIT BUTTON
        parent.spawn((
            Button,
            button_node.clone(),
            BackgroundColor(*BUTTON_DEFAULT_COLOR),
            MenuButtonAction::Quit
        ))
        .with_children( |parent| {
            parent.spawn((
                Text::new("Quit"),
                button_font.clone(),
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
            ));
        });
    });
}

/// Defines the actions that should occur on [Button] presses
/// Allows quit, settings, back to main menu, and resume options
/// Add this system to allow menu button actions to occur
fn menu_button_action_sys(
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    interactions: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, menu_button_action) in &interactions {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Settings => {
                    menu_state.set(MenuState::Settings);
                    info!("menu state: settings")
                }
                MenuButtonAction::BackToMainMenu => {
                    menu_state.set(MenuState::MainMenu);
                    info!("menu state: main menu")
                }
                MenuButtonAction::NewGame => {
                    game_state.set(GameState::Loading);
                    menu_state.set(MenuState::Disabled);
                    info!("menu state: disabled and game state: game!")
                }
            }
        }
    }
}

/// Constructs the settings menu, spawns in necessary entities
fn settings_menu_setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    let button_node = Node {
        width: Val::Px(500.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_font = TextFont {
        font: asset_server.load(PathBuf::from("fonts").join("NewHiScore.ttf")),
        font_size: 50.,
        ..default()
    };
    let title_font = TextFont {
        font: asset_server.load(PathBuf::from("fonts").join("NewHiScore.ttf")),
        font_size: 90.,
        ..default()
    };

    cmd.spawn((
        // this is the main bit that encapsulates the whole settings menu
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        OnMenuScreen,
        OnSettingsMenuScreen,
    ))
    .with_children( |parent| {
        // SETTINGS TITLE
        parent.spawn((
            Text::new("Settings"),
            title_font.clone(),
            TextColor(TEXT_COLOR),
            Node {
                margin: UiRect::all(Val::Px(50.0)),
                ..default()
            },
        ));
        // BACK TO MAIN MENU BUTTON
        parent.spawn((
            Button,
            button_node.clone(),
            BackgroundColor(*BUTTON_DEFAULT_COLOR),
            MenuButtonAction::BackToMainMenu
        ))
        .with_children( |parent| {
            parent.spawn((
                Text::new("Main Menu"),
                button_font.clone(),
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
            ));
        });
    });
}
