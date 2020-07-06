use crate::components::{
    backgrounds::*,
    exclamationmark::Exclamationmark,
    player::{PingPlayer, PlayerNumber},
};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::*, transform::Transform},
    input::{self, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

const PING_PLAYER_SCALE: f32 = 5.0;
const CHARA_WIDTH: f32 = 0.3;
const CHARA_HEIGHT: f32 = 0.3;

#[derive(Default)]
pub struct PingState {
    player_initmotion_timer: Option<f32>,
    player1_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    cpu_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for PingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.player1_sprite_sheet_handle
            .replace(load_sprite_sheet(world, "texture/HeavyBandit"));
        self.cpu_sprite_sheet_handle
            .replace(load_sprite_sheet(world, "texture/LightBandit"));

        init_chara(world, PlayerNumber::P2);
        init_camera(world);
        init_exclamationmark(world);
        init_backgrounds(world);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(e) = event {
            if input::is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

fn init_camera(world: &mut World) {
    let (screen_width, screen_height) = get_screensize(world);

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
    let (screen_width, screen_height) = get_screensize(world);
    let x = screen_width * CHARA_WIDTH;
    let y = screen_height * CHARA_HEIGHT;

    let mut p1_transform = Transform::default();
    p1_transform.set_translation_xyz(x, y, 0.0);
    p1_transform.set_scale(Vector3::new(-PING_PLAYER_SCALE, PING_PLAYER_SCALE, 1.0));
    let p1_sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, "texture/HeavyBandit"),
        sprite_number: 0,
    };

    let mut p2_transform = Transform::default();
    p2_transform.set_translation_xyz(screen_width - x, y, 0.0);
    p2_transform.set_scale(Vector3::new(PING_PLAYER_SCALE, PING_PLAYER_SCALE, 1.0));
    let p2_sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, "texture/LightBandit"),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(PingPlayer::new(PlayerNumber::P1))
        .with(p1_sprite_render)
        .with(p1_transform)
        .build();

    world
        .create_entity()
        .with(PingPlayer::new(PlayerNumber::CPU))
        .with(p2_sprite_render)
        .with(p2_transform)
        .build();
}

fn init_exclamationmark(world: &mut World) {
    let (screen_width, screen_height) = get_screensize(world);

    let mut transform = Transform::default();
    transform.set_translation_xyz(screen_width * 0.5, screen_height * 0.5, 0.1);
    transform.set_scale(Vector3::new(0.25, 0.25, 1.0));
    let sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, "texture/exclamationmark"),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Exclamationmark::default())
        .with(transform)
        .with(sprite_render)
        .build();
}

fn init_backgrounds(world: &mut World) {
    let (screen_width, screen_height) = get_screensize(world);

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

    let mut cloud_transform = Transform::default();
    cloud_transform.set_scale(Vector3::new(
        screen_width / 1920.0 * 0.3,
        screen_width / 1080.0 * 0.3,
        1.0,
    ));
    let mut cloud2_transform = cloud_transform.clone();
    cloud_transform.set_translation_xyz(screen_width * 0.233, screen_height * 0.8, -0.9);
    cloud2_transform.set_translation_xyz(screen_width * 0.788, screen_height * 0.75, -0.9);
    let backgrounds_sprite_sheet = load_sprite_sheet(world, "backgrounds/day-backgrounds");
    let cloud_sprite_render = SpriteRender {
        sprite_sheet: backgrounds_sprite_sheet.clone(),
        sprite_number: 0,
    };
    let cloud2_sprite_render = SpriteRender {
        sprite_sheet: backgrounds_sprite_sheet,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(Background::default())
        .with(transform)
        .with(sprite_render)
        .build();

    world
        .create_entity()
        .with(Cloud::default())
        .with(cloud_transform)
        .with(cloud_sprite_render)
        .build();

    world
        .create_entity()
        .with(Cloud::default())
        .with(cloud2_transform)
        .with(cloud2_sprite_render)
        .build();
}

fn get_screensize(world: &mut World) -> (f32, f32) {
    let screen_dimensions = world.read_resource::<ScreenDimensions>();
    (screen_dimensions.width(), screen_dimensions.height())
}
