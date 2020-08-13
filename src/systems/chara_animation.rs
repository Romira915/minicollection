use crate::components::player::*;
use amethyst::{
    animation::{
        self, AnimationCommand, AnimationControlSet, AnimationSampling, AnimationSet, EndControl,
        StepDirection,
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

        for (entity, player, animation_set) in
            (&entities, &mut ping_players, &animation_sets).join()
        {
            let control_set = animation::get_animation_set(&mut control_sets, entity).unwrap();

            if let Some(next_state) = player.state_queue.pop_front() {
                // Execute only when it is different from the previous state
                if next_state != player.previous_state {
                    let end_control = match next_state {
                        one_loop_state
                            if one_loop_state == PlayerState::Attack
                                || one_loop_state == PlayerState::Rise
                                || one_loop_state == PlayerState::Falling =>
                        {
                            if one_loop_state == PlayerState::Falling {
                                add_animation(
                                    control_set,
                                    &animation_set,
                                    PlayerState::Falled,
                                    EndControl::Loop(None),
                                );
                            }
                            if !control_set.has_animation(one_loop_state) {
                                add_animation(
                                    control_set,
                                    &animation_set,
                                    one_loop_state,
                                    EndControl::Stay,
                                );
                            }
                            EndControl::Stay
                        }
                        loop_infinitely
                            if loop_infinitely == PlayerState::BattleMode
                                || loop_infinitely == PlayerState::Falled =>
                        {
                            for &state in &[PlayerState::BattleMode, PlayerState::Falled] {
                                if state != loop_infinitely && control_set.has_animation(state) {
                                    control_set.abort(state);
                                    log::info!("Abort '{:?}'", state);
                                }
                            }
                            add_animation(
                                control_set,
                                &animation_set,
                                loop_infinitely,
                                EndControl::Loop(None),
                            );
                        }
                        _ => {}
                    };
                }

                player.previous_state = next_state;
            }
            if input.action_is_down("enter").unwrap() {
                player.push_state(PlayerState::Falling);
            // add_animation(
            //     control_set,
            //     &animation_set,
            //     PlayerState::Falling,
            //     EndControl::Stay,
            // );
            } else {
                // control_set.abort(PlayerState::Run);
            }
            if input.action_is_down("back").unwrap() {
                player.push_state(PlayerState::BattleMode);
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
        let sp_num_size = match player.previous_state {
            PlayerState::Wait => Self::WAIT,
            PlayerState::BattleMode => Self::COMBAT_MODE,
            PlayerState::Run => Self::RUN,
            PlayerState::Attack => Self::ATTACK,
            PlayerState::Rise => Self::RISE,
            PlayerState::Falled => Self::DOWN,
            _ => Self::DOWN,
        };

        sprite.sprite_number =
            Self::num_extend(player.anime_count, sp_num_size, Self::ANIMATION_INTERVAL);
        player.anime_count = player.anime_count.wrapping_add(1);
    }

    fn num_extend(n: usize, (start, size): (usize, usize), repeat: usize) -> usize {
        n % (size * repeat) / repeat + start
    }
}

use std::cmp::Eq;
use std::hash::Hash;
fn add_animation<I, T>(
    control_set: &mut AnimationControlSet<I, T>,
    animation_set: &AnimationSet<I, T>,
    id: I,
    end: EndControl,
) where
    I: PartialEq + Hash + Eq + Copy,
    T: AnimationSampling,
{
    control_set.add_animation(
        id,
        &animation_set.get(&id).unwrap(),
        end,
        1.0,
        AnimationCommand::Start,
    );
}
