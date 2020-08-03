use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::*, transform::Transform},
    input::{
        self, is_close_requested, Button, ControllerButton, InputEvent, InputHandler,
        StringBindings, VirtualKeyCode,
    },
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

pub struct PauseState;

impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Pause");
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Input(e) => match e {
                InputEvent::ControllerButtonPressed {
                    which: _,
                    button: ControllerButton::Start,
                }
                | InputEvent::ButtonPressed(Button::Key(VirtualKeyCode::Escape)) => Trans::Pop,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }
}
