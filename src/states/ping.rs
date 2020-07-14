use crate::{
    components::{
        backgrounds::*,
        exclamationmark::ExclamationmarkResources,
        player::{PingPlayer, PlayerNumber},
        stages::*,
        GeneralData, Gravity,
    },
    states::pause::PauseState,
    WorldDef,
};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::*, transform::Transform, ArcThreadPool},
    ecs::{Dispatcher, DispatcherBuilder},
    input::{self, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

const PING_PLAYER_SCALE: f32 = 5.0;
const CHARA_WIDTH: f32 = 0.3;
const CHARA_HEIGHT: f32 = 0.7;

#[derive(Default)]
pub struct PingState<'a, 'b> {
    player_initmotion_timer: Option<f32>,
    player1_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    cpu_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for PingState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let world_def = {
            let screen_size = super::get_screensize(world);
            WorldDef {
                screen_width: screen_size.0,
                screen_height: screen_size.1,
            }
        };
        world.add_resource(world_def);

        use crate::systems::{
            backgrounds::BackgroundsSystem, chara_animation::PingCharaAnimationSystem,
            exclamationmark::ExclamationmarkSystem, gravity_collision::*, player::PlayerSystem,
            stages::*,
        };
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(PlayerSystem, "player_system", &[]);
        dispatcher_builder.add(PingCharaAnimationSystem, "chara_amimation_system", &[]);
        dispatcher_builder.add(
            ExclamationmarkSystem::default(),
            "exclamationmark_system",
            &[],
        );
        dispatcher_builder.add(BackgroundsSystem, "backgrounds_system", &[]);
        dispatcher_builder.add(StageSystem, "stage_system", &[]);
        dispatcher_builder.add(
            GravityCollisionSystem,
            "gravity_collision_system",
            &["player_system", "stage_system"],
        );
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        init_chara(world, PlayerNumber::P2);
        init_camera(world);
        init_exclamationmark(world);
        init_backgrounds(world);
        init_stage(world);
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(e) = &event {
            if input::is_key_down(&e, VirtualKeyCode::Escape) {
                // return Trans::Quit;
                // return Trans::Push(Box::new(PauseState));
                // if let Some(dispatcher) = self.dispatcher.replace(None) {
                //     dispatcher.dispose(&mut data.world);
                // }
                return Trans::Replace(Box::new(PingState::default()));
                // return Trans::Switch(Box::new(PingState::default()));
            }
        }
        Trans::None
    }
}

fn init_camera(world: &mut World) {
    let (screen_width, screen_height) = super::get_screensize(world);

    let mut transform = Transform::default();
    transform.set_translation_xyz(screen_width * 0.5, screen_height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(screen_width, screen_height))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World, filename_noextension: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}.png", filename_noextension),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}.ron", filename_noextension),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn init_chara(world: &mut World, p2_mode: PlayerNumber) {
    let (screen_width, screen_height) = super::get_screensize(world);
    let p1_path = "texture/HeavyBandit";
    let p2_path = "texture/LightBandit";

    let x = screen_width * CHARA_WIDTH;
    let y = screen_height * CHARA_HEIGHT;

    let mut p1_transform = Transform::default();
    p1_transform.set_translation_xyz(x, y, 0.0);
    p1_transform.set_scale(Vector3::new(-PING_PLAYER_SCALE, PING_PLAYER_SCALE, 1.0));
    let p1_sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, p1_path),
        sprite_number: 0,
    };

    let mut p2_transform = Transform::default();
    p2_transform.set_translation_xyz(screen_width - x, y, 0.0);
    p2_transform.set_scale(Vector3::new(PING_PLAYER_SCALE, PING_PLAYER_SCALE, 1.0));
    let p2_sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, p2_path),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(PingPlayer::new(PlayerNumber::P1))
        .with(p1_sprite_render)
        .with(p1_transform)
        .with(Gravity::default())
        .with(
            GeneralData::default().with_size(
                super::get_sprite_size(p1_path, 0)
                    .unwrap()
                    .mul(PING_PLAYER_SCALE),
            ),
        )
        .build();

    world
        .create_entity()
        .with(PingPlayer::new(PlayerNumber::CPU))
        .with(p2_sprite_render)
        .with(p2_transform)
        .with(Gravity::default())
        .with(
            GeneralData::default().with_size(
                super::get_sprite_size(p2_path, 0)
                    .unwrap()
                    .mul(PING_PLAYER_SCALE),
            ),
        )
        .build();
}

fn init_exclamationmark(world: &mut World) {
    let (screen_width, screen_height) = super::get_screensize(world);

    let mut transform = Transform::default();
    transform.set_translation_xyz(screen_width * 0.5, screen_height * 0.5, 0.1);
    transform.set_scale(Vector3::new(0.25, 0.25, 1.0));
    let sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, "texture/exclamationmark"),
        sprite_number: 0,
    };

    world.add_resource(ExclamationmarkResources::new(sprite_render, transform));
}

fn init_backgrounds(world: &mut World) {
    use amethyst::renderer::sprite::{
        SpriteList,
        Sprites::{self, *},
    };
    use std::{fs::File, io::Read};

    let (screen_width, screen_height) = super::get_screensize(world);

    //background setting
    let mut transform = Transform::default();
    transform.set_translation_xyz(screen_width * 0.5, screen_height * 0.5, -1.0);
    // transform.set_translation_xyz(screen_width * 0.0, screen_height * 0.0, -1.0);
    transform.set_scale(Vector3::new(
        screen_width / 1920.0,
        screen_height / 1080.0,
        1.0,
    ));
    let sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, "backgrounds/day-beach-sky"),
        sprite_number: 0,
    };

    // cloud setting
    let backgrounds_path = "backgrounds/day-backgrounds";
    let mut cloud_transform = Transform::default();
    let cloud_scale = 0.3;
    cloud_transform.set_scale(Vector3::new(
        screen_width / 1920.0 * cloud_scale,
        screen_width / 1080.0 * cloud_scale,
        1.0,
    ));
    let mut cloud2_transform = cloud_transform.clone();
    cloud_transform.set_translation_xyz(screen_width * 0.233, screen_height * 0.9, -0.9);
    cloud2_transform.set_translation_xyz(screen_width * 0.788, screen_height * 0.75, -0.9);
    let backgrounds_sprite_sheet = load_sprite_sheet(world, backgrounds_path);
    let cloud_sprite_render = SpriteRender {
        sprite_sheet: backgrounds_sprite_sheet.clone(),
        sprite_number: 0,
    };
    let cloud2_sprite_render = SpriteRender {
        sprite_sheet: backgrounds_sprite_sheet,
        sprite_number: 1,
    };

    let (cloud, cloud2) = {
        let size = super::get_sprite_size(backgrounds_path, 0).unwrap();
        let size2 = super::get_sprite_size(backgrounds_path, 1).unwrap();

        (
            Cloud::new(size.mul(cloud_scale)),
            Cloud::new(size2.mul(cloud_scale)),
        )
    };

    world
        .create_entity()
        .with(Background::default())
        .with(transform)
        .with(sprite_render)
        .build();

    world
        .create_entity()
        .with(cloud)
        .with(cloud_transform)
        .with(cloud_sprite_render)
        .build();

    world
        .create_entity()
        .with(cloud2)
        .with(cloud2_transform)
        .with(cloud2_sprite_render)
        .build();
}

fn init_stage(world: &mut World) {
    let (screen_width, screen_height) = super::get_screensize(world);
    let stage_path = "stages/TX Tileset Ground";
    let sprite_sheet = load_sprite_sheet(world, stage_path);
    let (sprite_width, sprite_height) = super::get_sprite_size(stage_path, 0).unwrap();

    let range_x = (screen_width / sprite_width) as usize;
    let range_y: usize = 4;
    for i in 0..range_y {
        for j in 0..range_x {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                j as f32 * sprite_width + sprite_width / 2.0,
                i as f32 * sprite_height + sprite_height / 2.0,
                0.0,
            );
            let sprite_number = {
                let n = match j {
                    0 => 0,
                    x if x == range_x - 1 => 2,
                    _ => 1,
                };
                match i {
                    x if x == range_y - 1 => n,
                    _ => n + 3,
                }
            };
            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number,
            };

            world
                .create_entity()
                .with(Stage)
                .with(transform)
                .with(sprite_render)
                .with(GeneralData::default().with_size((sprite_width, sprite_height)))
                .build();
        }
    }
}

use super::Multiplication;
use std::ops::Mul;
// (T, T) * T == (T * T, T * T)
impl<T> Multiplication<T> for (T, T)
where
    T: Mul<Output = T> + Clone,
{
    fn mul(self, n: T) -> Self {
        (self.0 * n.clone(), self.1 * n)
    }
}
