mod camera;
mod components;
mod enemies;
mod player;
mod plugins;

use bevy::{app::App, DefaultPlugins};

use camera::CameraPlugin;
use enemies::EnemyPlugin;
use player::SpaceshipPlugin;
use plugins::{AssetLoaderPlugin, DebugPlugin, LightingPlugin, MovementPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // User configured plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(LightingPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
