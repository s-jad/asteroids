use bevy::{
    app::{App, Plugin, Update},
    ecs::system::Query,
    transform::components::Transform,
};

use crate::components::{Acceleration, Velocity};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity));
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut transform) in query.iter_mut() {
        transform.translation.x += vel.val.x;
        transform.translation.y += vel.val.y;
        transform.translation.z += vel.val.z;
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>) {
    for (acc, mut vel) in query.iter_mut() {
        vel.val.x += acc.val.x;
        vel.val.y += acc.val.y;
        vel.val.z += acc.val.z;
    }
}
