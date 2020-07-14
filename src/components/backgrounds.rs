use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::{frame_limiter::FrameRateLimitStrategy, math::*, transform::TransformBundle},
    ecs::{Component, DenseVecStorage},
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

#[derive(Default)]
pub struct Background {}

impl Component for Background {
    type Storage = DenseVecStorage<Self>;
}

pub struct Cloud {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
}

impl Component for Cloud {
    type Storage = DenseVecStorage<Self>;
}

impl Cloud {
    pub fn new((width, height): (f32, f32)) -> Self {
        Self {
            width,
            height,
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
        }
    }
}
