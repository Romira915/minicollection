use crate::components::exclamationmark::*;
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
    );

    fn run(&mut self, (entities, exclamationmarks, mut hiddens, input, time): Self::SystemData) {
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

        if self.count_frame == self.spanw_frame && !self.pressed {
            for (entity, _) in (&entities, &exclamationmarks).join() {
                hiddens
                    .remove(entity)
                    .expect("Failed to exclamationmark remove hidden");
            }
        }
        if self.count_frame >= self.spanw_frame && !self.pressed {
            self.past_frame += 1;

            if let Some(enter) = input.action_is_down("enter") {
                self.pressed = enter;
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
