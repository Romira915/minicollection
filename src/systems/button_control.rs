use crate::{components::backgrounds::*, WorldDef};
use amethyst::{
    core::{math::*, timing::Time, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{sprite::Sprite, SpriteRender},
    ui::{Selected, UiButtonActionRetrigger, UiFinder},
};

#[derive(SystemDesc)]
pub struct ButtonControlSystem;

impl<'s> System<'s> for ButtonControlSystem {
    type SystemData = (
        ReadStorage<'s, UiButtonActionRetrigger>,
        Read<'s, InputHandler<StringBindings>>,
        UiFinder<'s>,
    );

    fn run(&mut self, (ui_button_action_retriggers, input, ui_finder): Self::SystemData) {
        for (retrigger,) in (&ui_button_action_retriggers,).join() {}

        if input.action_is_down("start").unwrap_or(false) {
            if let Some(start) = ui_finder.find("start") {}
        }
    }
}
