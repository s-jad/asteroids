use bevy::{
    app::{App, Plugin, Startup},
    ecs::system::Commands,
    pbr::{AmbientLight, PointLight, PointLightBundle},
    prelude::default,
    render::{camera::ClearColor, color::Color},
    transform::components::Transform,
};

const LIGHT_DISTANCE: f32 = 30.0;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.25, 0.0, 0.15)));
        app.insert_resource(AmbientLight::default());
        app.add_systems(Startup, spawn_spotlight);
    }
}

fn spawn_spotlight(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, LIGHT_DISTANCE, 0.0);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 5_000_000.0,
            shadows_enabled: true,
            range: 150.0,
            radius: 0.0,
            ..default()
        },
        transform,
        ..default()
    });
}
