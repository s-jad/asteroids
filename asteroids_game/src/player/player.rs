use crate::components::{Acceleration, Velocity};
use bevy::prelude::*;

const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const STARTING_ACCELERATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    acceleration: Acceleration,
    model: SceneBundle,
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpaceshipBundle {
        velocity: Velocity::new(STARTING_VELOCITY),
        acceleration: Acceleration::new(STARTING_ACCELERATION),
        model: SceneBundle {
            scene: asset_server.load("player_spaceship.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}
