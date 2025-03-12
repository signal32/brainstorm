use bevy::prelude::*;
use super::{despawn_screen, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        // when we enter Menu GameState, spawn in the menu items
        .add_systems(OnEnter(GameState::Menu), menu_setup_sys)
        // while we are in this state, run button listener
        .add_systems(Update, button_sys)
        // when we leave this state, despawn all the entities that were needed for this screen
        .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>)
        // only bother to do things if there is user input -- slow otherwise ?
        .insert_resource(WinitSettings::desktop_app());
    }
}

// this is a tag component, so that we know what is on the menu screen
#[derive](Component)
struct OnMenuScreen;

// set some color constants -- eventually this can maybe be configurable?
const BUTTON_DEFAULT_COLOR: Color = Color::srgb_u8(49, 104, 65);
const BUTTON_HOVER_COLOR: Color = Color::srgb_u8(49, 104, 93);
const BUTTON_PRESSED_COLOR: Color = Color::srgb_u8(49, 104, 120);


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

fn menu_setup_sys {
    
}