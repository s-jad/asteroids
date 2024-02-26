use bevy::{
    app::{Plugin, Update},
    ecs::{entity::Entity, system::Query},
    log::info,
    prelude::App,
    transform::components::Transform,
};

use crate::components::Velocity;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (print_position, print_velocity));
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity[{:?}] moved to {:?}", entity, transform);
    }
}

fn print_velocity(query: Query<(Entity, &Velocity)>) {
    for (entity, velocity) in query.iter() {
        info!("Entity[{:?}] velocity: {:?}", entity, velocity);
    }
}
