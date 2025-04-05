use bevy::prelude::*;

use super::{
    despawn_entities,
    ButtonAction,
    ButtonNode,
    MenuContainerNode,
    MenuFont,
    MENU_TEXT_COLOR,
    SettingsButtonAction,
};
use super::main_menu::MenuState;
use super::pause::PauseMenuState;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: instead of hard-coding the states it checks for,
        // i need to have a way of externally adding states to the allowed states for a settings menu
        // perhaps its a resource? (idk if thats an acceptable use of resources)
        // but i want this to be a thing that you include and its fairly self-contained,
        // and you just call the things you need from it, give a button
        // a sensible ButtonAction and it calls all this setup stuff for the settings
        //
        // but for now, hard-coded is fine
        app
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup_sys)
            .add_systems(OnExit(MenuState::Settings), despawn_entities::<OnSettingsScreen>)
            .add_systems(OnEnter(PauseMenuState::Settings), settings_menu_setup_sys)
            .add_systems(OnExit(PauseMenuState::Settings), despawn_entities::<OnSettingsScreen>);
    }
}

#[derive(Component)]
pub(crate) struct OnSettingsScreen;

pub(crate) fn settings_menu_setup_sys(
    // TODO: pass in the current state when this is called and then can use a match
    // case to check where we came from to decide where to go back to maybe?
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
        },
    );
    let container = MenuContainerNode::spawn(&mut cmd);
    cmd.entity(container).
    insert(OnSettingsScreen)
    .with_children( |parent| {
            parent.spawn(sub_title_text);
        })
        .with_children(|mut parent| {
            ButtonNode::spawn(
                &mut parent,
                &asset_server,
                // FIXME: this wont work properly bc itll always try to take u to the Main Menu
                // we need to have a thing that stores/caches the menu u just came from somehow,
                // so that we can return to it3
                ButtonAction::Settings(SettingsButtonAction::BackToMenu),
                "Back to Menu".to_string(),
            );
        });
}
