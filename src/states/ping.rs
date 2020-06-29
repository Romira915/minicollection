use crate::components::player::Ping_Player;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

#[derive(Default)]
pub struct PingState {
    player_initmotion_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for PingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        init_chara(world, self.sprite_sheet_handle.clone().unwrap());
        init_camera(world);
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    let (screen_width, screen_height) = get_screensize(world);

    transform.set_translation_xyz(screen_width * 0.5, screen_height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(screen_width, screen_height))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/Characters.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/Characters.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn init_chara(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    let (screen_width, screen_height) = get_screensize(world);
    transform.set_translation_xyz(screen_width * 0.5, screen_height * 0.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Ping_Player::default())
        .with(sprite_render)
        .with(transform)
        .build();
}

fn get_screensize(world: &mut World) -> (f32, f32) {
    let screen_dimensions = world.read_resource::<ScreenDimensions>();
    (screen_dimensions.width(), screen_dimensions.height())
}
