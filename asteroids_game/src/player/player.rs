use crate::components::{Acceleration, Position, Velocity};
use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query},
    },
    log::info,
};

pub fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((
        Position::new(50.0, 50.0),
        Velocity::new(1.0, 1.0),
        Acceleration::default(),
    ));
}

pub fn update_position(mut query: Query<(&Velocity, &mut Position)>) {
    for (vel, mut pos) in query.iter_mut() {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}

pub fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>) {
    for (acc, mut vel) in query.iter_mut() {
        vel.x += acc.x;
        vel.y += acc.y;
    }
}

pub fn print_position(query: Query<(Entity, &Position)>) {
    for (entity, pos) in query.iter() {
        info!("Entity {:?} at position {:?}", entity, pos);
    }
}

pub fn print_velocity(query: Query<(Entity, &Velocity)>) {
    for (entity, vel) in query.iter() {
        info!("Entity {:?} travelling with velocity {:?}", entity, vel);
    }
}
