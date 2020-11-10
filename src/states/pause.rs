use crate::{
    components::player::{PingPlayer, PlayerState},
    states::{title::TitleState, ExtendedStateEvent},
};
use amethyst::{
    animation::{self, AnimationControlSet, AnimationSet},
    assets::{AssetStorage, Handle, Loader},
    core::{math::*, transform::Transform},
    ecs::Entity,
    ecs::{Entities, Join, ReadStorage, WriteStorage},
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

#[derive(Default)]
pub struct PauseState {
    ui_root: Option<Entity>,
    button_play: Option<Entity>,
    button_title: Option<Entity>,
}

impl<'a, 'b> State<GameData<'a, 'b>, ExtendedStateEvent> for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        log::info!("pause");

        player_animation_control(world, "pause");

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/pause.ron", ())));
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, ExtendedStateEvent> {
        let mut world = data.world;
        data.data.update(world);

        if self.button_play.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                self.button_play = finder.find("play");
            });
        }
        if self.button_title.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                self.button_title = finder.find("title");
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
            ExtendedStateEvent::Input(e) => match e {
                InputEvent::ControllerButtonPressed {
                    which: _,
                    button: ControllerButton::Start,
                }
                | InputEvent::ButtonPressed(Button::Key(VirtualKeyCode::Escape)) => {
                    log::info!("pop");
                    Trans::Pop
                }
                _ => Trans::None,
            },
            ExtendedStateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_play {
                    Trans::Pop
                } else if Some(target) == self.button_title {
                    Trans::Replace(Box::new(TitleState::default()))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        if let Some(entity) = self.ui_root {
            world
                .delete_entity(entity)
                .expect("Failed to remove pause ui_root");
        }
        self.button_play = None;
        self.button_title = None;

        player_animation_control(world, "start");
    }
}

fn player_animation_control(world: &mut World, command: &str) {
    world.exec(
        |(entities, players, animation_sets, mut control_sets): (
            Entities,
            ReadStorage<PingPlayer>,
            ReadStorage<AnimationSet<PlayerState, SpriteRender>>,
            WriteStorage<AnimationControlSet<PlayerState, SpriteRender>>,
        )| {
            for (entity, _, _) in (&entities, &players, &animation_sets).join() {
                let control_set = animation::get_animation_set(&mut control_sets, entity).unwrap();
                for &state in PlayerState::iter() {
                    if control_set.has_animation(state) {
                        match command {
                            "start" => {
                                control_set.start(state);
                            }
                            "pause" => {
                                control_set.pause(state);
                            }
                            _ => {}
                        };
                    }
                }
            }
        },
    );
}
