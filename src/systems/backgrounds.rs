use crate::{components::backgrounds::*, WorldDef};
use amethyst::{
    core::{math::*, timing::Time, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{sprite::Sprite, SpriteRender},
};
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Float, Normal, StandardNormal};

#[derive(SystemDesc)]
pub struct BackgroundsSystem;

impl<'s> System<'s> for BackgroundsSystem {
    type SystemData = (
        ReadStorage<'s, Background>,
        WriteStorage<'s, Cloud>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadExpect<'s, WorldDef>,
    );

    fn run(&mut self, (backgrounds, mut clouds, mut transforms, time, worlddef): Self::SystemData) {
        let mut rng = rand::thread_rng();
        let gaussian = Normal::new(0.0, 0.001).unwrap();

        for (cloud, transform) in (&mut clouds, &mut transforms).join() {
            cloud.velocity += gaussian.sample(&mut rng) as f32;

            transform.prepend_translation_x(cloud.velocity * time.delta_seconds());
            if transform.translation().x > worlddef.screen_width + cloud.width * 0.5 {
                transform.set_translation_x(-cloud.width * 0.5);
            }
        }
    }
}
