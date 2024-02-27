use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::Vec3,
    scene::SceneBundle,
};

#[derive(Component, Debug)]
pub struct Velocity {
    pub val: Vec3,
}

impl Velocity {
    pub fn new(val: Vec3) -> Self {
        Self { val }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub val: Vec3,
}

impl Acceleration {
    pub fn new(val: Vec3) -> Self {
        Self { val }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
}
