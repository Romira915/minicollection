use crate::{
    components::player::{PingPlayer, PlayerState},
    states::ExtendedStateEvent,
};
use amethyst::{
    animation::{self, AnimationControlSet, AnimationSet},
    assets::{AssetStorage, Handle, Loader},
    core::{math::*, transform::Transform},
    ecs::{Entities, Join, ReadStorage, WriteStorage},
    input::{
        self, is_close_requested, Button, ControllerButton, InputEvent, InputHandler,
        StringBindings, VirtualKeyCode,
    },
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

pub struct PauseState;

impl<'a, 'b> State<GameData<'a, 'b>, ExtendedStateEvent> for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        log::info!("pause");

        player_animation_control(world, "pause");
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
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

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
