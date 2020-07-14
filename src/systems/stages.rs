use crate::components::stages::*;
use amethyst::{
    core::{math::*, timing::Time, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

#[derive(SystemDesc)]
pub struct StageSystem;

impl<'s> System<'s> for StageSystem {
    type SystemData = (ReadStorage<'s, Stage>,);

    fn run(&mut self, (stages,): Self::SystemData) {}
}
