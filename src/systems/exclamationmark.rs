use crate::components::exclamationmark::*;
use amethyst::{
    core::{math::*, timing::Time, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};
use rand::rngs::ThreadRng;

pub struct ExclamationmarkSystem {
    spanw_frame: u64,
    // count_frame :usize,
}

impl<'s> System<'s> for ExclamationmarkSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, ExclamationmarkResources>,
        WriteStorage<'s, Exclamationmark>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            entities,
            exclamationmark_resources,
            mut exclamationmark,
            mut spriterenders,
            mut transforms,
            time,
        ): Self::SystemData,
    ) {
        if time.frame_number() == crate::FRAME_RATE as u64 * self.spanw_frame {
            entities
                .build_entity()
                .with(Exclamationmark, &mut exclamationmark)
                .with(
                    exclamationmark_resources.sprite_render.clone(),
                    &mut spriterenders,
                )
                .with(exclamationmark_resources.transform.clone(), &mut transforms)
                .build();
        }
    }
}

impl Default for ExclamationmarkSystem {
    fn default() -> Self {
        Self { spanw_frame: 5 }
    }
}
