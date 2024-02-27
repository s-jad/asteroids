use std::ops::Range;

use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Handle},
    ecs::{
        component::Component,
        system::{Commands, Res, ResMut, Resource},
    },
    math::{Quat, Vec3},
    prelude::default,
    scene::{Scene, SceneBundle},
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
};
use rand::{thread_rng, Rng};

use crate::components::{Acceleration, MovingObjectBundle, Velocity};

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_DELAY: f32 = 2.0;

#[derive(Component, Debug)]
pub struct AsteroidMarker;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    time: Timer,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            time: Timer::from_seconds(SPAWN_DELAY, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_asteroids);
    }
}

fn spawn_asteroids(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    spawn_timer.time.tick(time.delta());
    if !spawn_timer.time.just_finished() {
        return;
    }

    let mut rng = thread_rng();
    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut rand_unit_vec =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = rand_unit_vec() * VELOCITY_SCALAR;
    let acceleration = Vec3::ZERO;

    let asteroid_type: Handle<Scene> = match rng.gen_range(0..=4) {
        0 => asset_server.load("asteroid_1.glb#Scene0"),
        1 => asset_server.load("asteroid_2.glb#Scene0"),
        2 => asset_server.load("asteroid_3.glb#Scene0"),
        3 => asset_server.load("asteroid_4.glb#Scene0"),
        _ => asset_server.load("large_asteroid_1.glb#Scene0"),
    };

    let rand_scale = rng.gen_range(0.75..1.25);
    let scale = Vec3::new(rand_scale, rand_scale, rand_scale);

    let rotation = Quat::from_axis_angle(
        Vec3::new(
            rng.gen_range(0.1..1.5),
            rng.gen_range(0.1..1.5),
            rng.gen_range(0.1..1.5),
        ),
        10.0,
    );

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            model: SceneBundle {
                scene: asteroid_type,
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                ..default()
            },
        },
        AsteroidMarker,
    ));
}
