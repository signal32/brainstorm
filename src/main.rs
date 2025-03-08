use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bird_tweet_sys)
        .add_systems(Startup, bird_add_sys)
        .run();
}

#[derive(Component)]
struct Bird {
    name: String
}

impl Bird {
    fn tweet(&self) {
        println!("tweet i am a {}", self.name)
    }
}

fn bird_tweet_sys(birds: Query<&Bird>) {
    for bird in birds.iter() {
        bird.tweet();
    }
}

fn bird_add_sys(mut cmd: Commands) {
    cmd.spawn(Bird { name: "greenfinch".to_string() });
}
