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
pub struct Exclamationmark {}

impl Component for Exclamationmark {
    type Storage = DenseVecStorage<Self>;
}
