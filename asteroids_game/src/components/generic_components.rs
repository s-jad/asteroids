use bevy::{ecs::component::Component, math::Vec3};

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
