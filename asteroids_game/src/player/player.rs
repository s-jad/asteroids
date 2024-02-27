use bevy::{
    app::{App, Plugin, PostStartup},
    ecs::system::{Commands, Res},
    math::Vec3,
    prelude::default,
    scene::SceneBundle,
    transform::components::Transform,
};

use crate::{
    components::{Acceleration, MovingObjectBundle, Velocity},
    plugins::asset_loader::SceneAssets,
};

const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.1);
const STARTING_ACCELERATION: Vec3 = Vec3::ZERO;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<SceneAssets>) {
    let scale = Vec3::new(0.5, 0.5, 0.5);

    commands.spawn(MovingObjectBundle {
        velocity: Velocity::new(STARTING_VELOCITY),
        acceleration: Acceleration::new(STARTING_ACCELERATION),
        model: SceneBundle {
            scene: asset_server.player_spaceship.clone(),
            transform: Transform {
                translation: STARTING_TRANSLATION,
                scale,
                ..default()
            },
            ..default()
        },
    });
}
