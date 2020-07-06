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

#[derive(PartialEq, Eq)]
pub enum PlayerNumber {
    P1,
    P2,
    CPU,
}

impl Default for PlayerNumber {
    fn default() -> Self {
        Self::P1
    }
}

#[derive(PartialEq, Eq)]
pub enum PlayerState {
    wait,
    combat_mode,
    run,
    attack,
    rise,
    down,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::wait
    }
}

pub struct PingPlayer {
    pub player_num: PlayerNumber,
    pub velocity: Vector2<f32>,
    pub state: PlayerState,
    pub anime_count: usize,
}

impl PingPlayer {
    pub fn new(p_num: PlayerNumber) -> Self {
        Self {
            player_num: p_num,
            ..Default::default()
        }
    }
}

impl Component for PingPlayer {
    type Storage = DenseVecStorage<Self>;
}

impl Default for PingPlayer {
    fn default() -> Self {
        Self {
            velocity: Vector2::new(0.0, 0.0),
            player_num: Default::default(),
            state: Default::default(),
            anime_count: Default::default(),
        }
    }
}
