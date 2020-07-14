use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::*, transform::Transform},
    input::{self, VirtualKeyCode},
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
        if let StateEvent::Window(e) = event {
            if input::is_key_down(&e, VirtualKeyCode::Escape) {
                // return Trans::Quit;
                return Trans::Pop;
            }
        }
        Trans::None
    }
}
