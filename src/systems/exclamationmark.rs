use crate::components::exclamationmark::Exclamationmark;
use amethyst::{
    core::{math::*, timing::Time, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

pub struct ExclamationmarkSystem;

impl<'s> System<'s> for ExclamationmarkSystem {
    type SystemData = (
        ReadStorage<'s, Exclamationmark>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (exclamationmark, mut transforms): Self::SystemData) {}
}
