use bevy::{
    app::{App, Plugin, PostStartup, Update},
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec3,
    prelude::default,
    scene::SceneBundle,
    time::Time,
    transform::components::Transform,
};

use crate::{
    components::{Acceleration, MovingObjectBundle, Velocity},
    plugins::asset_loader::SceneAssets,
};

const STARTING_VELOCITY: Vec3 = Vec3::ZERO;
const STARTING_ACCELERATION: Vec3 = Vec3::ZERO;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_ACCELERATION: f32 = 2.5;
const SPACESHIP_MAX_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const BASIC_MISSILE_SPEED: f32 = 30.0;
const BASIC_MISSILE_SPAWN_SCALAR: f32 = 1.0;

#[derive(Component, Debug)]
pub struct SpaceShip;

#[derive(Component, Debug)]
pub struct SpaceShipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_controls, spaceship_missile_controls),
        );
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<SceneAssets>) {
    let scale = Vec3::new(0.5, 0.5, 0.5);

    commands.spawn((
        MovingObjectBundle {
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
        },
        SpaceShip,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<SpaceShip>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if kb_input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_ACCELERATION;
    } else if kb_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_ACCELERATION;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if kb_input.pressed(KeyCode::KeyD) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    if kb_input.pressed(KeyCode::KeyQ) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if kb_input.pressed(KeyCode::KeyE) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);

    velocity.val = -transform.forward() * movement;
}

fn spaceship_missile_controls(
    mut commands: Commands,
    query: Query<&Transform, With<SpaceShip>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<SceneAssets>,
) {
    let transform = query.single();

    if kb_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * BASIC_MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: SceneBundle {
                    scene: asset_server.missile.clone(),
                    transform: Transform {
                        translation: transform.translation
                            + -transform.forward() * BASIC_MISSILE_SPAWN_SCALAR,
                        scale: Vec3::new(0.1, 0.1, 0.1),
                        ..default()
                    },
                    ..default()
                },
            },
            SpaceShipMissile,
        ));
    }
}
