use crate::Component;

#[derive(Component, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
}

impl Acceleration {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}