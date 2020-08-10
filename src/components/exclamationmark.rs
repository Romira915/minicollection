use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::{frame_limiter::FrameRateLimitStrategy, math::*, transform::Transform},
    ecs::{Component, DenseVecStorage, Entity},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle, SpriteRender,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

pub struct Exclamationmark;

impl Component for Exclamationmark {
    type Storage = DenseVecStorage<Self>;
}

pub struct ExclamationmarkResources {
    pub sprite_render: SpriteRender,
    pub transform: Transform,
}

impl Component for ExclamationmarkResources {
    type Storage = DenseVecStorage<Self>;
}

impl ExclamationmarkResources {
    pub fn new(sprite_render: SpriteRender, transform: Transform) -> Self {
        Self {
            sprite_render,
            transform,
        }
    }
}
