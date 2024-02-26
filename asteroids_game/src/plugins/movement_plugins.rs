use bevy::{
    app::{App, Plugin, Update},
    ecs::system::{Query, Res},
    time::Time,
    transform::components::Transform,
};

use crate::components::{Acceleration, Velocity};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity));
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (vel, mut transform) in query.iter_mut() {
        transform.translation.x += vel.val.x * time.delta_seconds();
        transform.translation.y += vel.val.y * time.delta_seconds();
        transform.translation.z += vel.val.z * time.delta_seconds();
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acc, mut vel) in query.iter_mut() {
        vel.val.x += acc.val.x * time.delta_seconds();
        vel.val.y += acc.val.y * time.delta_seconds();
        vel.val.z += acc.val.z * time.delta_seconds();
    }
}
