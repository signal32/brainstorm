use super::GameState;

use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        // TODO app things here
        //app.add_systems(OnEnter(GameState::Game), hud_setup_sys);
            //.add_systems(OnExit(GameState::Game), despawn_entities::<OnHud>); // idk if this a good way of doing it for the HUD tbh
    }
}

fn hud_setup_sys(
    cmd: &mut Commands,
) {
    // run this when the hud is initialised, i.e. on new game
    // cmd.spawn(
    //     // TODO: do hud things here
    // );
}

#[derive(Component)]
struct OnHud;
