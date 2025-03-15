use bevy::{prelude::*};
use std::{path::PathBuf, sync::LazyLock};
use super::{despawn_screen, pause_menu_listener_sys, GameState};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<PauseMenuState>()
        // handle the GameState Pause thing
        .add_systems(OnEnter(GameState::Pause), pause_setup_sys)
        .add_systems(Update, (
            button_color_sys,
            // pause_menu_listener_sys,
            unpause_listener_sys,
            menu_button_action_sys
        ).run_if(in_state(GameState::Pause)))
        .add_systems(OnExit(GameState::Pause), despawn_screen::<OnMenuScreen>)
        // handle the Main Menu gubbins
        .add_systems(OnEnter(PauseMenuState::PauseMenu), pause_menu_setup_sys)
        .add_systems(OnExit(PauseMenuState::PauseMenu), despawn_screen::<OnPauseMenuScreen>)
        // handle the settings screen gubbins
        .add_systems(OnEnter(PauseMenuState::Settings), settings_menu_setup_sys)
        .add_systems(OnExit(PauseMenuState::Settings), despawn_screen::<OnSettingsMenuScreen>);
    }
}

// set some color constants -- eventually this can maybe be configurable?
static BUTTON_DEFAULT_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(49, 104, 65));
static BUTTON_HOVER_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(56, 104, 76));
static BUTTON_PRESSED_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(49, 104, 93));

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

/// Tag Entities with this if they appear on any menu screen
#[derive(Component)]
struct OnMenuScreen;
/// Tag Entities with this if they occur on [PauseMenuState::PauseMenu]
#[derive(Component)]
struct OnPauseMenuScreen;
/// Tag Entities with this if they occur on [PauseMenuState::Settings]
#[derive(Component)]
struct OnSettingsMenuScreen;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PauseMenuState {
    PauseMenu,
    Settings,
    #[default]
    Disabled
}

// all the actions that a menu button can possibly do
#[derive(Component)]
enum MenuButtonAction {
    Settings,
    BackToPauseMenu,
    Resume,
    Quit
}

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

/// Listens for the escape key to be pressed, and if so,
/// changes the [GameState] and [PauseMenuState] accordingly
/// to exit the pause menu
fn unpause_listener_sys(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut pause_state: ResMut<NextState<PauseMenuState>>
) {
    if keys.just_pressed(KeyCode::Escape) {
        pause_state.set(PauseMenuState::Disabled);
        game_state.set(GameState::Game);
        info!("*pause state:* disabled & *game state:* game");
    }
}

fn pause_setup_sys(mut pause_state: ResMut<NextState<PauseMenuState>>) {
    pause_state.set(PauseMenuState::PauseMenu);
}

/// Constructs the pause menu, spawning in buttons and info text
fn pause_menu_setup_sys(
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
        OnPauseMenuScreen,
    ))
    .with_children( |parent| {
        // MAIN TITLE
        parent.spawn((
            Text::new("Game Paused"),
            title_font.clone(),
            TextColor(TEXT_COLOR),
            Node {
                margin: UiRect::all(Val::Px(50.0)),
                ..default()
            },
        ));
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
        // RESUME BUTTON
        parent.spawn((
            Button,
            button_node.clone(),
            BackgroundColor(*BUTTON_DEFAULT_COLOR),
            MenuButtonAction::Resume
        ))
        .with_children( |parent| {
            parent.spawn((
                Text::new("Resume"),
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

/// Constructs the settings sub-menu, spawning in buttons and info text
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
            MenuButtonAction::BackToPauseMenu
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


fn menu_button_action_sys(
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut pause_state: ResMut<NextState<PauseMenuState>>,
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
                    pause_state.set(PauseMenuState::Settings);
                    info!("*pause state:* settings")
                }
                MenuButtonAction::BackToPauseMenu => {
                    pause_state.set(PauseMenuState::PauseMenu);
                    info!("*pause state:* pause menu")
                }
                MenuButtonAction::Resume => {
                    game_state.set(GameState::Game);
                    pause_state.set(PauseMenuState::Disabled);
                    info!("*pause state:* disabled and *game state:* game")
                }
            }
        }
    }
}
