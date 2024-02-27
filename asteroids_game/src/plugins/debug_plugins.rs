use crate::components::Velocity;
use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        system::{Query, Res, ResMut, Resource},
    },
    log::info,
    prelude::App,
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
};

#[derive(Debug, Resource)]
struct DebugTimer {
    time: Timer,
}

impl Default for DebugTimer {
    fn default() -> Self {
        Self {
            time: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugTimer::default())
            .add_systems(Update, (print_position, print_velocity));
    }
}

fn print_position(query: Query<(Entity, &Transform)>, mut dt: ResMut<DebugTimer>, time: Res<Time>) {
    if dt.time.tick(time.delta()).just_finished() {
        for (entity, transform) in query.iter() {
            info!("Entity[{:?}] moved to {:?}", entity, transform);
        }
    }
}

fn print_velocity(query: Query<(Entity, &Velocity)>, mut dt: ResMut<DebugTimer>, time: Res<Time>) {
    if dt.time.tick(time.delta()).just_finished() {
        for (entity, vel) in query.iter() {
            info!("Entity[{:?}] moved to {:?}", entity, vel);
        }
    }
}
