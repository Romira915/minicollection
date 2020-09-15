use crate::{
    components::{exclamationmark::*, player::*},
    states::PingEvent,
};
use amethyst::{
    core::{math::*, timing::Time, Hidden, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{
        Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, SystemData, World, Write,
        WriteExpect, WriteStorage,
    },
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
    shrev::*,
    ui::{UiFinder, UiText, UiTransform},
};
use rand::rngs::ThreadRng;

pub struct ExclamationmarkSystem {
    spanw_frame: usize,
    count_frame: usize,
    past_frame: usize,
    pressed: bool,
}

impl<'s> System<'s> for ExclamationmarkSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Exclamationmark>,
        WriteStorage<'s, Hidden>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Write<'s, EventChannel<PingEvent>>,
        WriteStorage<'s, PingPlayer>,
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
    );

    // doneTODO: 早押し判定システム実装
    // TODO: 経過フレーム・勝敗UI実装
    fn run(
        &mut self,
        (
            entities,
            exclamationmarks,
            mut hiddens,
            input,
            time,
            mut channel,
            mut players,
            mut ui_text,
            ui_finder,
        ): Self::SystemData,
    ) {
        self.count_frame += 1;
        // if self.count_frame == crate::FRAME_RATE * self.spanw_frame {
        //     entities
        //         .build_entity()
        //         .with(Exclamationmark, &mut exclamationmark)
        //         .with(
        //             exclamationmark_resources.sprite_render.clone(),
        //             &mut spriterenders,
        //         )
        //         .with(exclamationmark_resources.transform.clone(), &mut transforms)
        //         .build();
        // }

        let mut push_anime = |p: PlayerNumber, s: PlayerState| {
            for (player) in (&mut players).join() {
                if player.player_num == p {
                    player.push_state(s.clone());
                }
            }
        };

        // Processing when the button is pressed
        // arg: Before the exclamation mark spanned
        // Return if someone pressed: bool
        let mut process_when_pressed = |before: bool| {
            let mut p1 = false;
            let mut p2 = false;

            if let Some(enter) = input.action_is_down("enter") {
                p1 = enter;
            }
            if let Some(enter) = input.action_is_down("enter_p2") {
                p2 = enter;
            }
            if p1 && p2 {
                channel.single_write(PingEvent::Draw);
            } else if p1 {
                channel.single_write(if before {
                    PingEvent::P1Flying
                } else {
                    PingEvent::P1Win
                });
                push_anime(PlayerNumber::P1, PlayerState::Attack);
            } else if p2 {
                channel.single_write(if before {
                    PingEvent::P2Flying
                } else {
                    PingEvent::P2Win
                });
                push_anime(PlayerNumber::P2, PlayerState::Attack);
            }
            if p1 || p2 {
                return true;
            }
            false
        };

        // Before appearing exclamationmark
        if self.count_frame <= self.spanw_frame && !self.pressed {
            self.pressed = process_when_pressed(true);
        }

        // When it appears exclamationmark
        if self.count_frame == self.spanw_frame && !self.pressed {
            for (entity, _) in (&entities, &exclamationmarks).join() {
                hiddens
                    .remove(entity)
                    .expect("Failed to exclamationmark remove hidden");
            }
        }

        // After appearing exclamationmark
        if self.count_frame >= self.spanw_frame && !self.pressed {
            self.past_frame += 1;

            // ui update
            let past_ui = ui_finder
                .find("past_frame")
                .expect("Found to ui past_frame");
            let past_text = ui_text.get_mut(past_ui).expect("Failed to ui_text.get_mut");

            past_text.text = self.past_frame.to_string();

            self.pressed = process_when_pressed(false);
        }
    }
}

impl Default for ExclamationmarkSystem {
    fn default() -> Self {
        Self {
            spanw_frame: crate::FRAME_RATE * 5,
            count_frame: Default::default(),
            past_frame: Default::default(),
            pressed: false,
        }
    }
}

// impl ExactSizeIterator {
//     pub fn new(world: &mut World) -> Self {
//         <Self as System<'_>>::SystemData::setup(world);
//     }
// }
