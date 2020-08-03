pub mod backgrounds;
pub mod exclamationmark;
pub mod player;
pub mod stages;

use amethyst::{
    core::math::*,
    ecs::{Component, DenseVecStorage, NullStorage},
};

#[derive(Default)]
pub struct Gravity;

impl Component for Gravity {
    type Storage = NullStorage<Self>;
}

#[derive(Clone)]
pub struct GeneralData {
    pub width: f32,
    pub height: f32,
    pub velocity: Vector2<f32>,
}

impl Component for GeneralData {
    type Storage = DenseVecStorage<Self>;
}

impl GeneralData {
    pub fn with_size(mut self, (width, height): (f32, f32)) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_v(mut self, v: Vector2<f32>) -> Self {
        self.velocity = v;
        self
    }
}

impl Default for GeneralData {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            velocity: Vector2::new(0.0, 0.0),
        }
    }
}