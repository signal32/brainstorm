use bevy::prelude::*;
use super::*;
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::GameOver), game_over_setup_sys)
        .add_systems(Update, (
                button_color_sys,
                game_over_button_action_sys
            ).run_if(in_state(GameState::GameOver))
        )
        .add_systems(OnExit(GameState::GameOver), despawn_entities::<OnGameOverScreen>);
    }
}

#[derive (Component)]
struct OnGameOverScreen;

fn game_over_setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>
) {
    let container = MenuContainerNode::spawn(&mut cmd);
    let game_over_text = (
        Text::new("Game Over"),
        MenuFont::sub_title_font(&asset_server),
        TextColor(MENU_TEXT_COLOR),
    );
    cmd.entity(container)
        .insert(OnGameOverScreen)
        .with_children(|parent| {
            parent.spawn(game_over_text);
        })
        .with_children(|mut parent| {
            ButtonNode::spawn(
                &mut parent,
                &asset_server,
                ButtonAction::GameOver(GameOverButtonAction::TryAgain),
                "Try Again".to_string(),
            );
        })
        .with_children(|mut parent| {
            ButtonNode::spawn(
                &mut parent,
                &asset_server,
                ButtonAction::GameOver(GameOverButtonAction::ReturnToTitle),
                "Return to Title".to_string(),
            );
        });
}

fn game_over_button_action_sys(
    mut game_state: ResMut<NextState<GameState>>,
    interactions: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button_action) in &interactions {
        if *interaction == Interaction::Pressed {
            match button_action {
                ButtonAction::GameOver(GameOverButtonAction::TryAgain) => {
                    game_state.set(GameState::Game);
                    debug!("Setting GameState to Game")
                }
                ButtonAction::GameOver(GameOverButtonAction::ReturnToTitle) => {
                    game_state.set(GameState::Menu);
                    debug!("Setting GameState to Menu")
                }
                _ => {
                    panic!("Something has gone wrong in the Game Over screen.")
                }
            }
        }
    }
}