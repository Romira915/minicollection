use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::{frame_limiter::FrameRateLimitStrategy, math::*, transform::TransformBundle},
    ecs::{Component, DenseVecStorage, NullStorage},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};
#[derive(Copy, Clone, PartialEq)]
pub enum SkyTime {
    Day,
    Sunset,
    Night,
}

pub struct Background {
    pub skytime: SkyTime,
}

impl Component for Background {
    type Storage = DenseVecStorage<Self>;
}

impl Background {
    pub fn new(skytime: SkyTime) -> Self {
        Background { skytime }
    }
}

pub struct Cloud {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
    pub skytime: SkyTime,
}

impl Component for Cloud {
    type Storage = DenseVecStorage<Self>;
}

impl Cloud {
    pub fn new((width, height): (f32, f32), skytime: SkyTime) -> Self {
        Self {
            width,
            height,
            skytime,
            ..Default::default()
        }
    }
}

impl Default for Cloud {
    fn default() -> Self {
        Self {
            velocity: 10.0,
            width: Default::default(),
            height: Default::default(),
            skytime: SkyTime::Day,
        }
    }
}

// #[derive(Default)]
// pub struct Day;
// impl Component for Day {
//     type Storage = NullStorage<Self>;
// }
// #[derive(Default)]
// pub struct Sunset;
// impl Component for Sunset {
//     type Storage = NullStorage<Self>;
// }
// #[derive(Default)]
// pub struct Night;
// impl Component for Night {
//     type Storage = NullStorage<Self>;
// }
