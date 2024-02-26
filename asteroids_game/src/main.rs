use bevy::{app::App, DefaultPlugins};
use player::SpaceshipPlugin;
use plugins::{DebugPlugin, MovementPlugin};
mod components;
mod player;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // User configured plugins
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
