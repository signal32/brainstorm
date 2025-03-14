use bevy::{prelude::*};
use std::{path::PathBuf};
use super::{despawn_screen, GameState, UIText};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        // when we enter Menu GameState, spawn in the menu items
        .init_state::<MenuState>()
        // handle stuff about whether we are in the menu GameState
        .add_systems(OnEnter(GameState::Menu), menu_setup_sys)
        .add_systems(Update, (
            button_sys,
            unmenu_yourself
        ).run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>)
        // handle the Main Menu gubbins
        .add_systems(OnEnter(MenuState::MainMenu), main_menu_setup_sys)
        .add_systems(Update, (
            menu_button_action_sys
        ).run_if(in_state(MenuState::MainMenu)))
        .add_systems(OnExit(MenuState::MainMenu), despawn_screen::<OnMainMenuScreen>);
        // handle the settings screen gubbins
        // .add_systems(OnEnter(MenuState::Settings), settings_menu_setup_sys)
        // .add_systems(OnExit(MenuState::Settings), despawn_screen::<OnSettingsScreen>);
    }
}

// this is a tag component, so that we know what is on the menu screen
#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnSettingsScreen;

// define the menu states
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
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
    Resume,
    Quit
}

// set some color constants -- eventually this can maybe be configurable?
// this doestnt work and idk why yet
// const BUTTON_DEFAULT_COLOR: Color = Color::srgb_u8(49, 104, 65);
// const BUTTON_HOVER_COLOR: Color = Color::srgb_u8(49, 104, 93);
// const BUTTON_PRESSED_COLOR: Color = Color::srgb_u8(49, 104, 120);

const BUTTON_DEFAULT_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
const BUTTON_HOVER_COLOR: Color = Color::srgb(0.30, 0.30, 0.30);
const BUTTON_PRESSED_COLOR: Color = Color::srgb(0.45, 0.45, 0.45);

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);


fn button_sys (
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap(); // getting the text in case want to update it e.g. **text = "Resume".to_string();
        match *interaction {
            Interaction::Pressed => {
                *color = BUTTON_PRESSED_COLOR.into();
                //border_color.0 = Color::WHITE; // can change border color if u want
            }
            Interaction::Hovered => {
                *color = BUTTON_HOVER_COLOR.into();
            }
            Interaction::None => {
                *color = BUTTON_DEFAULT_COLOR.into();
            }
        }
    }
}

fn menu_setup_sys(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::MainMenu);
    info!("in theory the MenuState should now be MainMenu")
}

fn main_menu_setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_font = TextFont {
        font: asset_server.load(PathBuf::from("fonts").join("NewHiScore.ttf")),
        font_size: 60.,
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
            ..default()
        },
        OnMenuScreen,
        OnMainMenuScreen,
    ))
    .with_children( |parent| {
        parent.spawn((
            Text::new("Bird Invaders"),
            title_font,
            TextColor(TEXT_COLOR),
            Node {
                margin: UiRect::all(Val::Px(50.0)),
                ..default()
            },
            UIText
        ));
        // ITS BUTTON TIME
        parent.spawn((
            Button,
            button_node.clone(),
            BackgroundColor(BUTTON_DEFAULT_COLOR),
            MenuButtonAction::Settings
        ))
        .with_children( |parent| {
            parent.spawn((
                Text::new("Settings"),
                button_font,
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
                UIText
            ));
        });
    });
}

fn unmenu_yourself(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>
) {
    if keys.just_pressed(KeyCode::KeyM) {
        game_state.set(GameState::Game);
        menu_state.set(MenuState::Disabled);
        info!("WE GO BACK TO GAMING NOW, GAMERS!");
    }
}

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
                    info!("IM SO GLAD! (we are in settings menu state now thats nice)")
                }
                MenuButtonAction::BackToMainMenu => {
                    menu_state.set(MenuState::MainMenu);
                    info!("back to main menu")
                }
                MenuButtonAction::Resume => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                    info!("resume the game yippeeee birds be flyin")
                }
            }
        }
    }
}

// fn settings_menu_setup_sys (
//     mut cmd: Commands
// ) {
//     cmd.spawn( // spawn some stuff

//     );
// }