use crate::{
    components::player::{PingPlayer, PlayerState},
    states::ExtendedStateEvent,
};
use amethyst::{
    animation::{
        self, AnimationCommand, AnimationControlSet, AnimationSet, EndControl, StepDirection,
    },
    assets::{
        AssetStorage, Handle, Loader, PrefabData, PrefabLoader, PrefabLoaderSystemDesc,
        ProgressCounter, RonFormat,
    },
    core::{
        math::*,
        shrev::{EventChannel, ReaderId},
        transform::Transform,
        ArcThreadPool, EventReader, Hidden, HiddenPropagate,
    },
    derive::EventReader,
    ecs::{
        prelude::Entity, Component, Dispatcher, DispatcherBuilder, Entities, Join, Read,
        ReadStorage, SystemData, World, WriteStorage,
    },
    input::{
        self, is_close_requested, BindingTypes, Button, ControllerButton, InputEvent, InputHandler,
        StringBindings, VirtualKeyCode,
    },
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{UiCreator, UiEvent, UiFinder, UiPrefab, UiText},
    window::ScreenDimensions,
    winit::Event,
};

#[derive(Default)]
pub struct TitleState {
    progress_counter: Option<ProgressCounter>,
    ui_root: Option<Entity>,
}

impl<'a, 'b> State<GameData<'a, 'b>, ExtendedStateEvent> for TitleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.progress_counter = Some(Default::default());

        self.ui_root = Some(world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/title.ron", self.progress_counter.as_mut().unwrap())
        }));
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, ExtendedStateEvent> {
        let world = data.world;
        data.data.update(world);

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        if let Some(ui_root) = self.ui_root {
            world
                .delete_entity(ui_root)
                .expect("Failed to delete entity");
            self.ui_root = None;
        }
    }
}
