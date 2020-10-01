use crate::{
    components::player::{PingPlayer, PlayerState},
    states::{loading::LoadingState, ping::PingState, ExtendedStateEvent},
    systems::button_control::ButtonControlSystem,
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
    ui::{UiCreator, UiEvent, UiEventType, UiFinder, UiPrefab, UiText},
    window::ScreenDimensions,
    winit::Event,
};

#[derive(Default)]
pub struct TitleState<'c, 'd> {
    dispatcher: Option<Dispatcher<'c, 'd>>,
    progress_counter: Option<ProgressCounter>,
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_exit: Option<Entity>,
}

impl<'a, 'b, 'c, 'd> State<GameData<'a, 'b>, ExtendedStateEvent> for TitleState<'c, 'd> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.progress_counter = Some(Default::default());

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(ButtonControlSystem, "button_control_system", &[]);
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

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

        if let Some(progress_counter) = self.progress_counter.as_ref() {
            if !progress_counter.is_complete() {
                let mut progress_counter = None;
                std::mem::swap(&mut self.progress_counter, &mut progress_counter);
                return Trans::Push(Box::new(LoadingState::new(progress_counter)));
            }
        }

        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(world);
        }

        if self.button_start.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                self.button_start = finder.find("start");
            });
        }
        if self.button_exit.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                self.button_exit = finder.find("exit");
            })
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: ExtendedStateEvent,
    ) -> Trans<GameData<'a, 'b>, ExtendedStateEvent> {
        match event {
            ExtendedStateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_start {
                    log::debug!("push ping");
                    return Trans::Switch(Box::new(PingState::default()));
                }
                if Some(target) == self.button_exit {
                    log::debug!("exit");
                    return Trans::Quit;
                }
            }
            _ => {}
        }

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
