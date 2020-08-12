use crate::components::player::*;
use amethyst::{
    animation::{
        self, AnimationCommand, AnimationControlSet, AnimationSet, EndControl, StepDirection,
    },
    core::{math::*, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{prelude::Entities, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

#[derive(SystemDesc)]
pub struct PingCharaAnimationSystem;

impl<'s> System<'s> for PingCharaAnimationSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, PingPlayer>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, AnimationSet<PlayerState, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<PlayerState, SpriteRender>>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (entities, mut ping_players, mut sprites, animation_sets, mut control_sets, input): Self::SystemData,
    ) {
        for (player, sprite) in (&mut ping_players, &mut sprites).join() {
            // self.anime(player, sprite);
        }

        for (entity, animation_set) in (&entities, &animation_sets).join() {
            let control_set = animation::get_animation_set(&mut control_sets, entity).unwrap();

            if input.action_is_down("enter").unwrap() {
                control_set.toggle(PlayerState::Run);
            }
            if input.action_is_down("back").unwrap() {
                control_set.start(PlayerState::Attack);
            }
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
            PlayerState::Wait => Self::WAIT,
            PlayerState::CombatMode => Self::COMBAT_MODE,
            PlayerState::Run => Self::RUN,
            PlayerState::Attack => Self::ATTACK,
            PlayerState::Rise => Self::RISE,
            PlayerState::Down => Self::DOWN,
        };

        sprite.sprite_number =
            Self::num_extend(player.anime_count, sp_num_size, Self::ANIMATION_INTERVAL);
        player.anime_count = player.anime_count.wrapping_add(1);
    }

    fn num_extend(n: usize, (start, size): (usize, usize), repeat: usize) -> usize {
        n % (size * repeat) / repeat + start
    }
}
