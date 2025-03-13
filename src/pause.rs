use bevy::{prelude::*, winit::WinitSettings};
use super::{despawn_screen, GameState};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        // when we enter Pause GameState, spawn in the pause items
        .add_systems(OnEnter(GameState::Pause), pause_setup_sys)
        // while we are in this state, run button listener
        //.add_systems(Update, button_sys)
        // when we leave this state, despawn all the entities that were needed for this screen
        .add_systems(OnExit(GameState::Pause), despawn_screen::<OnPaused>)
        // only bother to do things if there is user input -- slow otherwise ?
        .insert_resource(WinitSettings::desktop_app());
    }
}

// this is a tag component, so that we know what is displayed while game is paused
#[derive(Component)]
struct OnPaused;

fn pause_setup_sys(

) {
    info!("we are paused");
}