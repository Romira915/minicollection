use crate::components::backgrounds::*;
use amethyst::{
    core::{math::*, timing::Time, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

#[derive(SystemDesc)]
pub struct BackgroundsSystem;

impl<'s> System<'s> for BackgroundsSystem {
    type SystemData = (ReadStorage<'s, Background>, ReadStorage<'s, Cloud>);

    fn run(&mut self, (background, cloud): Self::SystemData) {}
}
