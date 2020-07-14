use crate::components::player::{PingPlayer, PlayerState};
use amethyst::{
    core::{math::*, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

#[derive(SystemDesc)]
pub struct PingCharaAnimationSystem;

impl<'s> System<'s> for PingCharaAnimationSystem {
    type SystemData = (WriteStorage<'s, PingPlayer>, WriteStorage<'s, SpriteRender>);

    fn run(&mut self, (mut ping_players, mut sprites): Self::SystemData) {
        for (player, sprite) in (&mut ping_players, &mut sprites).join() {
            self.anime(player, sprite);
        }
    }
}

impl PingCharaAnimationSystem {
    const WAIT: (usize, usize) = (0, 4);
    const COMBAT_MODE: (usize, usize) = (4, 4);
    const RUN: (usize, usize) = (8, 8);
    const ATTACK: (usize, usize) = (16, 8);
    const RISE: (usize, usize) = (24, 8);
    const DOWN: (usize, usize) = (32, 4);

    const ANIMATION_INTERVAL: usize = (0.07 * (crate::FRAME_RATE as f64)) as usize;

    fn anime(&mut self, player: &mut PingPlayer, sprite: &mut SpriteRender) {
        let sp_num_size = match player.state {
            PlayerState::wait => Self::WAIT,
            PlayerState::combat_mode => Self::COMBAT_MODE,
            PlayerState::run => Self::RUN,
            PlayerState::attack => Self::ATTACK,
            PlayerState::rise => Self::RISE,
            PlayerState::down => Self::DOWN,
        };

        sprite.sprite_number =
            Self::num_extend(player.anime_count, sp_num_size, Self::ANIMATION_INTERVAL);
        player.anime_count = player.anime_count.wrapping_add(1);
    }

    fn num_extend(n: usize, (start, size): (usize, usize), repeat: usize) -> usize {
        n % (size * repeat) / repeat + start
    }
}
