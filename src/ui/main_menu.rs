use bevy::prelude::*;

use super::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup_sys)
            .add_systems(OnExit(GameState::Menu), despawn_entities::<OnMenuScreen>)
            .add_systems(OnEnter(MenuState::MainMenu), main_menu_setup_sys)
            .add_systems(OnExit(MenuState::MainMenu), despawn_entities::<OnMainMenuScreen>)
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup_sys)
            .add_systems(OnExit(MenuState::Settings), despawn_entities::<OnSettingsMenuScreen>)
            .add_systems(
                Update,
                (
                    button_color_sys,
                    menu_button_action_sys,
                    pause_menu_listener_sys,
                )
                    .run_if(in_state(GameState::Menu)),
            );
    }
}

// Tag Entities with this if they are visible on [MenuState::MainMenu]
#[derive(Component)]
struct OnMainMenuScreen;

// / Defines the MenuStates for the Main Menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    MainMenu,
    Settings,
    #[default]
    Disabled,
}

fn menu_setup_sys(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::MainMenu);
    debug!("menu state: main menu")
}

fn main_menu_setup_sys(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let title_text = (
        Text::new("Bird Invaders"),
        MenuFont::title_font(&asset_server),
        TextColor(MENU_TEXT_COLOR),
        Node {
            margin: UiRect::all(Val::Px(50.0)),
            ..default()
        },
    );
    let top_left_border = (
        Sprite {
            image: asset_server.load(PathBuf::from("menu").join("top_left_border.png")),
            custom_size: Some(Vec2::splat(300.0)),
            ..default()
        },
        Transform::from_xyz(-450.0, 250.0, -10.0),
        OnMenuScreen,
        OnMainMenuScreen,
    );
    let top_right_border = (
        Sprite {
            image: asset_server.load(PathBuf::from("menu").join("top_right_border.png")),
            custom_size: Some(Vec2::splat(300.0)),
            ..default()
        },
        Transform::from_xyz(450.0, 250.0, -10.0),
        OnMenuScreen,
        OnMainMenuScreen,
    );
    let bottom_left_border = (
        Sprite {
            image: asset_server.load(PathBuf::from("menu").join("bottom_left_border.png")),
            custom_size: Some(Vec2::splat(300.0)),
            ..default()
        },
        Transform::from_xyz(-450.0, -250.0, -10.0),
        OnMenuScreen,
        OnMainMenuScreen,
    );
    let bottom_right_border = (
        Sprite {
            image: asset_server.load(PathBuf::from("menu").join("bottom_right_border.png")),
            custom_size: Some(Vec2::splat(300.0)),
            ..default()
        },
        Transform::from_xyz(450.0, -250.0, -10.0),
        OnMenuScreen,
        OnMainMenuScreen,
    );

    let container = MenuContainerNode::spawn(&mut cmd);

    cmd.entity(container)
        .insert((OnMenuScreen, OnMainMenuScreen))
        .with_children(|parent| {
            parent.spawn(title_text);
        })
        .with_children(|mut parent| {
            ButtonNode::spawn(
                &mut parent,
                &asset_server,
                ButtonAction::Menu(MenuButtonAction::NewGame),
                "New Game".to_string(),
            );
        })
        .with_children(|mut parent| {
            ButtonNode::spawn(
                &mut parent,
                &asset_server,
                ButtonAction::Settings,
                "Settings".to_string(),
            );
        })
        .with_children(|mut parent| {
            ButtonNode::spawn(
                &mut parent,
                &asset_server,
                ButtonAction::Quit,
                "Quit".to_string(),
            );
        });
    
    cmd.spawn(top_left_border);
    cmd.spawn(top_right_border);
    cmd.spawn(bottom_left_border);
    cmd.spawn(bottom_right_border);
}

/// Defines the actions that should occur on [Button] presses
/// Allows quit, settings, back to main menu, and resume options
/// Add this system to allow menu button actions to occur
fn menu_button_action_sys(
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    interactions: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button_action) in &interactions {
        if *interaction == Interaction::Pressed {
            match button_action {
                ButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                ButtonAction::Settings => {
                    menu_state.set(MenuState::Settings);
                    debug!("menu state: settings")
                }
                ButtonAction::Menu(MenuButtonAction::BackToMenu) => {
                    menu_state.set(MenuState::MainMenu);
                    debug!("menu state: main menu")
                }
                ButtonAction::Menu(MenuButtonAction::NewGame) => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                    debug!("menu state: disabled and game state: game!")
                }
                _ => {
                    panic!("You've somehow done something that isn't a menu thing, in the menu.")
                }
            }
        }
    }
}
