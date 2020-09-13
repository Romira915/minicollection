use crate::{components::exclamationmark::*, states::PingEvent};
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
    );

    // TODO: 早押し判定システム実装
    fn run(
        &mut self,
        (entities, exclamationmarks, mut hiddens, input, time, mut channel): Self::SystemData,
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
        
        // When it appears
        if self.count_frame == self.spanw_frame && !self.pressed {
            for (entity, _) in (&entities, &exclamationmarks).join() {
                hiddens
                    .remove(entity)
                    .expect("Failed to exclamationmark remove hidden");
            }
        }

        // After appearing
        if self.count_frame >= self.spanw_frame && !self.pressed {
            self.past_frame += 1;

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
                channel.single_write(PingEvent::P1Win);
            } else if p2 {
                channel.single_write(PingEvent::P2Win);
            }
            if p1 || p2 {
                self.pressed = true;
            }
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
