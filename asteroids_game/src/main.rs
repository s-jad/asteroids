mod camera;
mod components;
mod player;
mod plugins;

use bevy::{
    app::App,
    pbr::AmbientLight,
    render::{camera::ClearColor, color::Color},
    DefaultPlugins,
};

use camera::CameraPlugin;
use player::SpaceshipPlugin;
use plugins::{DebugPlugin, MovementPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.25, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 125.0,
        })
        .add_plugins(DefaultPlugins)
        // User configured plugins
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
