pub mod asset;
pub mod spawner;

use std::path::PathBuf;

use asset::*;
use spawner::*;
use bevy::prelude::*;
use rand::Rng;

use crate::{level::Level, physics::{ColliderContactEvent, Velocity}, util::ron_asset_loader::RonAssetLoader, GameState};

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BirdAsset>();
        app.init_asset_loader::<RonAssetLoader<BirdAsset>>();
        app.add_systems(OnEnter(GameState::Game), setup_sys);
        app.add_systems(FixedUpdate, setup_spawner_sys.run_if(in_state(GameState::Game)));
        app.add_systems(FixedUpdate, (
            bird_spawn_sys,
            bird_hit_sys,
            update_bird_tweet_sys,
        ).run_if(in_state(GameState::Game)));
        app.add_systems(FixedPostUpdate, load_bird_assets_sys);
    }
}

#[derive(Component)]
struct Bird {
    name: String,
    hunger: i8,
}

impl Bird {
    fn tweet(&self) {
        println!("tweet i am a {}", self.name)
    }
}

/// Makes birds fly away from non bird entities that collide with them.
fn bird_hit_sys(
    mut contact_ev: EventReader<ColliderContactEvent>,
    mut birds: Query<(&mut Velocity, &mut Transform), With<Bird>>,
    mut level: ResMut<Level>,
) {
    let mut rng = rand::rng();
    for ev in contact_ev.read() {
        // avoid total chaos by disavowing bird to bird collisions
        if birds.get(ev.a).is_ok() && birds.get(ev.b).is_ok() {
            continue;
        }

        // otherwise extract the bird from either of the two collisions, if any
        let bird = if let Ok(b) = birds.get_mut(ev.a) { Some(b) }
        else if let Ok(b) =  birds.get_mut(ev.b) { Some(b) }
        else { None };

        if let Some((mut velocity, mut tf)) = bird {
            velocity.0 *= 2.;
            let rotate_rads = if rng.random_bool(0.5) { -2. } else { 2. };
            tf.rotate_local_z(rotate_rads);

            level.score += 10;
        }
    }
}

fn update_bird_tweet_sys(
    mut bird_text: Single<&mut Text, With<BirdTweetText>>,
    birds: Query<&Bird, Added<Bird>>,
    level: Res<Level>,
) {
    for bird in birds.iter() {
        bird_text.0 = format!("Score {}\ntweet i am a {}", level.score, bird.name);
    }
}


#[derive(Component)]
struct BirdTweetText;

fn setup_sys(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    // This is a bit of a hack to make sure only 1 BirdTweetText ever exists.
    // Otherwise another gets added each time we enter game state which causes a panic elsewhere.
    once!(cmd.spawn((
        BirdTweetText,
        Text::new("Feed the birds!".to_string()), // initial greeting before any birds show up
        TextFont {
            font: asset_server.load(PathBuf::from("fonts").join("NewHiScore.ttf")),
            font_size: 50.,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Right),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            right: Val::Px(5.),
            ..default()
        },
    )));
}
