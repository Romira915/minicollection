use amethyst::{
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
        AnimationSetPrefab, EndControl,
    },
    assets::{PrefabData, ProgressCounter},
    audio::{AudioBundle, DjSystemDesc},
    core::{frame_limiter::FrameRateLimitStrategy, math::*, transform::TransformBundle},
    derive::PrefabData,
    ecs::{prelude::Entity, Component, DenseVecStorage},
    error::Error,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

const QUEUE_CAPACITY: usize = 2;

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct PlayerPrefabData {
    animation_set: AnimationSetPrefab<PlayerState, SpriteRender>,
}

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
    Wait,
    BattleMode,
    Run,
    Attack,
    Rise,
    Falling,
    Falled,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Wait
    }
}

#[derive(Default)]
pub struct PingPlayer {
    pub player_num: PlayerNumber,
    pub previous_state: PlayerState,
    pub anime_count: usize,
    pub is_on_stage: bool,
    pub state_queue: VecDeque<PlayerState>,
}

impl PingPlayer {
    pub fn new(p_num: PlayerNumber) -> Self {
        Self {
            player_num: p_num,
            state_queue: VecDeque::with_capacity(QUEUE_CAPACITY),
            ..Default::default()
        }
    }

    pub fn push_state(&mut self, state: PlayerState) {
        match self.state_queue.back() {
            Some(s) if *s == state || self.state_queue.len() == QUEUE_CAPACITY => {}
            _ => {
                self.state_queue.push_back(state);
            }
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
