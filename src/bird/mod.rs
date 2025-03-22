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
            setup_bird_hunger_bar_sys,
            update_bird_hunger_bar_sys,
        ).run_if(in_state(GameState::Game)));
        app.add_systems(FixedPostUpdate, load_bird_assets_sys);
    }
}

#[derive(Component)]
struct Bird {
    name: String,
    /// Units of food required to satisfy hunger
    hunger: i8,
    initial_hunger: i8,
    /// Base points to grant player on being fed unit of food
    on_feed_points: u32,
}

impl Bird {
    fn tweet(&self) {
        println!("tweet i am a {}", self.name)
    }
}

#[derive(Component)]
struct BirdHungerBar;

/// Makes birds fly away from non bird entities that collide with them.
fn bird_hit_sys(
    mut contact_ev: EventReader<ColliderContactEvent>,
    mut birds: Query<(&mut Velocity, &mut Transform, &mut Bird)>,
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

        if let Some((mut velocity, mut tf, mut bird)) = bird {
            bird.hunger = bird.hunger.saturating_sub(1);
            level.score += bird.on_feed_points;

            // fly away once no longer hungry
            if bird.hunger == 0 {
                velocity.0 *= 2.;
                let rotate_rads = if rng.random_bool(0.5) { -2. } else { 2. };
                tf.rotate_local_z(rotate_rads);
            }
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

fn update_bird_hunger_bar_sys(
    changed_birds: Query<(&Bird, &Children), Changed<Bird>>,
    mut cmd: Commands,
    mut bird_hunger_bars: Query<(Entity, &mut Mesh2d, &mut MeshMaterial2d<ColorMaterial>), With<BirdHungerBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (bird, children) in changed_birds.iter() {
        let percent_full = (bird.initial_hunger - bird.hunger) as f32 / (bird.initial_hunger as f32);

        for &child in children.iter() {
            if let Ok((entity, mut mesh, mut material)) = bird_hunger_bars.get_mut(child) {
                if bird.hunger == 0 {
                    cmd.entity(entity).despawn()
                }

                // fade colour bar between orange and green as bird gets fed
                let orange = bevy::color::palettes::css::ORANGE;
                let green = bevy::color::palettes::css::GREEN;
                let colour = Srgba::from_vec3(orange.to_vec3().lerp(green.to_vec3(), percent_full));
                material.0 = materials.add(Color::Srgba(colour));

                // make colour bar bigger
                // TODO: scale existing meshes rather than make new ones
                // although i don't believe this to be a performance problem
                // it would be nice to interpolate between positions to create a smoothened animation
                mesh.0 = meshes.add(Capsule2d::new(5., 100. * percent_full));
            }
        }
    }
}

fn setup_bird_hunger_bar_sys(
    mut cmd: Commands,
    added_hunger_bars: Query<Entity, Added<BirdHungerBar>>,
) {
    for bar in added_hunger_bars.iter() {
        cmd.entity(bar).insert((
            Mesh2d(Handle::default()),
            MeshMaterial2d::<ColorMaterial>(Handle::default()),
        ));
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
