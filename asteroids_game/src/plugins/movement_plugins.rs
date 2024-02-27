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
        transform.translation += vel.val * time.delta_seconds();
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acc, mut vel) in query.iter_mut() {
        vel.val += acc.val * time.delta_seconds();
    }
}
