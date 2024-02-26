use bevy::prelude::*;
use player::spawn_spaceship;
mod components;
mod player;

fn main() {
    App::new()
        .add_systems(Startup, spawn_spaceship)
        .add_plugins(DefaultPlugins)
        .run();
}
