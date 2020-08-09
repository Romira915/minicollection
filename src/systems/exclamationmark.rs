use crate::components::exclamationmark::*;
use amethyst::{
    core::{math::*, timing::Time, Hidden, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{
        Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, SystemData, World,
        WriteStorage,
    },
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};
use rand::rngs::ThreadRng;

pub struct ExclamationmarkSystem {
    spanw_frame: usize,
    count_frame: usize,
}

impl<'s> System<'s> for ExclamationmarkSystem {
    type SystemData = (
        ReadStorage<'s, Exclamationmark>,
        ReadStorage<'s, SpriteRender>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Hidden>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (exclamationmarks, spriterenders, transforms, mut hiddens, time): Self::SystemData,
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

        for (exclamationmark,) in (&exclamationmarks,).join() {
            if self.count_frame == self.spanw_frame {
                hiddens
                    .remove(exclamationmark.self_entity.clone())
                    .expect("Failed to exclamationmark remove hidden");
            }
        }
    }
}

impl Default for ExclamationmarkSystem {
    fn default() -> Self {
        Self {
            spanw_frame: crate::FRAME_RATE * 5,
            count_frame: Default::default(),
        }
    }
}
