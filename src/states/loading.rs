use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    core::{math::*, timing::Time, transform::Transform, ArcThreadPool},
    ecs::{prelude::Entity, Dispatcher, DispatcherBuilder},
    input::{
        self, is_close_requested, Button, ControllerButton, InputEvent, InputHandler,
        StringBindings, VirtualKeyCode,
    },
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{UiCreator, UiFinder, UiImage, UiTransform},
    window::ScreenDimensions,
};

const LOADING_OUTSIDE_WIDTH: f32 = 609.0;

pub struct LoadingState {
    progress_counter: Option<ProgressCounter>,
    ui_root: Option<Entity>,
    loading_inside: Option<Entity>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        self.ui_root = Some(world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/loading.ron", self.progress_counter.as_mut().unwrap())
        }));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // let StateData { world, .. } = data;
        let world = &mut data.world;
        let ref_progress_counter = self.progress_counter.as_ref().unwrap();

        if ref_progress_counter.is_complete() {
            return Trans::Pop;
        } else {
            let mut ui_transform = world.write_storage::<UiTransform>();
            if let Some(loading_inside) = self
                .loading_inside
                .and_then(|entity| ui_transform.get_mut(entity))
            {
                let num_finished = ref_progress_counter.num_finished() as f32;
                let num_assets = ref_progress_counter.num_assets() as f32;
                let percent = num_finished / num_assets;

                let time = world.read_resource::<Time>();
                if loading_inside.width <= percent * LOADING_OUTSIDE_WIDTH {
                    loading_inside.width += 500. * time.delta_seconds();
                    loading_inside.width = loading_inside.width.min(LOADING_OUTSIDE_WIDTH);
                }

                // loading_inside.width = percent * LOADING_OUTSIDE_WIDTH;
                loading_inside.local_x = loading_inside.width * 0.5;
            }
        }

        if self.loading_inside.is_none() {
            world.exec(|finder: UiFinder| {
                if let Some(entity) = finder.find("loading_inside") {
                    self.loading_inside = Some(entity);
                }
            });
        }

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove loading state");
        }

        self.ui_root = None;
    }
}

impl LoadingState {
    pub fn new(progress_counter: Option<ProgressCounter>) -> Self {
        LoadingState {
            progress_counter,
            ui_root: None,
            loading_inside: None,
        }
    }
}
