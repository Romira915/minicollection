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
use serde::{Deserialize, Serialize};

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

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
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

#[derive(Default)]
pub struct PingPlayer {
    pub player_num: PlayerNumber,
    pub state: PlayerState,
    pub anime_count: usize,
    pub is_on_stage: bool,
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

// impl Default for PingPlayer {
//     fn default() -> Self {
//         Self {
//             player_num: Default::default(),
//             state: Default::default(),
//             anime_count: Default::default(),
//         }
//     }
// }
