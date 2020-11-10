use crate::{
    components::player::{PingPlayer, PlayerState},
    states::{title::TitleState, ExtendedStateEvent},
};
use amethyst::{
    animation::{self, AnimationControlSet, AnimationSet},
    assets::{AssetStorage, Handle, Loader},
    core::HiddenPropagate,
    core::{math::*, timing::Time, transform::Transform},
    ecs::Entity,
    ecs::{Entities, Join, Read, ReadStorage, WriteStorage},
    input::{
        self, is_close_requested, Button, ControllerButton, InputEvent, InputHandler,
        StringBindings, VirtualKeyCode,
    },
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::UiCreator,
    ui::UiEvent,
    ui::UiEventType,
    ui::UiFinder,
    window::ScreenDimensions,
};

const HIDE_REMOVE_LIMIT: f32 = 3f32;

#[derive(Default)]
pub struct WinState {
    ui_root: Option<Entity>,
    label_win: Option<Entity>,
    hide_remove_count: f32,
    is_hide_remove: bool,
}

impl<'a, 'b> State<GameData<'a, 'b>, ExtendedStateEvent> for WinState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, ExtendedStateEvent> {
        let mut world = data.world;
        data.data.update(world);

        if self.label_win.is_none() {
            self.label_win = world.exec(|finder: UiFinder<'_>| finder.find("win"));
        }

        world.exec(|time: Read<Time>| {
            self.hide_remove_count += time.delta_seconds();
        });

        if self.hide_remove_count > HIDE_REMOVE_LIMIT && !self.is_hide_remove {
            if let Some(entity) = self.label_win {
                world.exec(|mut hiddenPs: WriteStorage<HiddenPropagate>| {
                    hiddenPs
                        .remove(entity)
                        .expect("Failed to remove HiddenPropagate");
                });
                self.is_hide_remove = true;
            }
        }

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        if let Some(root_entity) = self.ui_root {
            world
                .delete_entity(root_entity)
                .expect("Failed to remove ping win_ui_root");
            self.ui_root = None;
        }
    }
}

impl WinState {
    pub fn new(ui_root: Entity) -> Self {
        WinState {
            ui_root: Some(ui_root),
            ..Default::default()
        }
    }
}
