use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    core::{math::*, transform::Transform, ArcThreadPool},
    ecs::{prelude::Entity, Dispatcher, DispatcherBuilder},
    input::{
        self, is_close_requested, Button, ControllerButton, InputEvent, InputHandler,
        StringBindings, VirtualKeyCode,
    },
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

pub struct NowLoadingState {
    progress_counter: Option<ProgressCounter>,
    now_loading_entity: Option<Entity>,
}

impl SimpleState for NowLoadingState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.as_ref().unwrap().is_complete() {
            return Trans::Pop;
        }

        Trans::None
    }
}

impl NowLoadingState {
    pub fn new(progress_counter: Option<ProgressCounter>) -> Self {
        NowLoadingState {
            progress_counter,
            now_loading_entity: None,
        }
    }
}
