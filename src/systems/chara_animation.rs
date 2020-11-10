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
pub struct PingCharaAnimationSystem {
    one_loop_state_list: Vec<PlayerState>,
    loop_infinitely_state_list: Vec<PlayerState>,
    preframe_is_down: bool,
}

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
                for &s in &[
                    PlayerState::Attack,
                    PlayerState::BattleMode,
                    PlayerState::Falled,
                    PlayerState::Falling,
                    PlayerState::Rise,
                    PlayerState::Run,
                    PlayerState::Wait,
                ] {
                    // log::info!("has animation {:?} => {}", s, control_set.has_animation(s));
                }
                // println!();
                // if next_state != player.previous_state {

                // Execute only when it is different from the previous state
                match next_state {
                    one_loop_state if self.one_loop_state_list.include(one_loop_state) => {
                        // if one_loop_state == PlayerState::Falling {
                        //     add_animation(
                        //         control_set,
                        //         &animation_set,
                        //         PlayerState::Falled,
                        //         EndControl::Loop(None),
                        //     );
                        // }
                        if !control_set.has_animation(one_loop_state) {
                            add_animation(
                                control_set,
                                &animation_set,
                                one_loop_state,
                                EndControl::Stay,
                            );
                        }
                        if one_loop_state == PlayerState::Falling {
                            player.push_state(PlayerState::Falled);
                        }
                    }
                    loop_infinitely if self.loop_infinitely_state_list.include(loop_infinitely) => {
                        for &state in self.loop_infinitely_state_list.iter() {
                            // Abort ids other than the infinite loop animation to be executed
                            if state != loop_infinitely
                                    // && state != PlayerState::Wait
                                    && control_set.has_animation(state)
                            {
                                control_set.abort(state);
                                log::debug!("Abort '{:?}'", state);
                            }
                        }
                        for &state in self.one_loop_state_list.iter() {
                            if control_set.has_animation(state) {
                                player.push_state(loop_infinitely);
                                break;
                            }
                            if !control_set.has_animation(loop_infinitely) {
                                add_animation(
                                    control_set,
                                    &animation_set,
                                    loop_infinitely,
                                    EndControl::Loop(None),
                                );
                                log::debug!("add_animation");
                            }
                        }
                    }
                    _ => {}
                };

                player.previous_state = next_state;
            }

            // NOTE: テスト機構
            // if input.action_is_down("enter").unwrap() {
            //     player.push_state(PlayerState::Attack);
            // // add_animation(
            // //     control_set,
            // //     &animation_set,
            // //     PlayerState::Falling,
            // //     EndControl::Stay,
            // // );
            // } else {
            //     // control_set.abort(PlayerState::Run);
            // }
            // if input.action_is_down("back").unwrap() {
            //     player.push_state(PlayerState::Rise);
            // }

            if player.player_num == PlayerNumber::P1
                && input.action_is_down("enter_p2").unwrap_or(false)
                && !self.preframe_is_down
            {
                player.push_state(PlayerState::Falling);
                self.preframe_is_down = true;
            } else if (player.player_num == PlayerNumber::P2
                || player.player_num == PlayerNumber::CPU)
                && input.action_is_down("enter").unwrap_or(false)
                && !self.preframe_is_down
            {
                player.push_state(PlayerState::Falling);
                self.preframe_is_down = true;
            }

            if (!input.action_is_down("enter").unwrap_or(false)
                && !input.action_is_down("enter_p2").unwrap_or(false))
                && self.preframe_is_down
            {
                self.preframe_is_down = false;
            }
        }
    }
}

impl Default for PingCharaAnimationSystem {
    fn default() -> Self {
        PingCharaAnimationSystem {
            one_loop_state_list: vec![PlayerState::Attack, PlayerState::Falling, PlayerState::Rise],
            loop_infinitely_state_list: vec![
                PlayerState::BattleMode,
                PlayerState::Falled,
                PlayerState::Run,
                PlayerState::Wait,
            ],
            preframe_is_down: false,
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

pub trait Include<T>
where
    T: Eq + PartialEq,
{
    fn include(&self, t: T) -> bool;
}

impl<T> Include<T> for Vec<T>
where
    T: PartialEq + Eq + Copy,
{
    fn include(&self, t: T) -> bool {
        for &e in self {
            if e == t {
                return true;
            }
        }
        false
    }
}
