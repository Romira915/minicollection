use crate::{
    components::{
        backgrounds::*,
        exclamationmark::{Exclamationmark, ExclamationmarkResources},
        player::*,
        stages::*,
        GeneralData, Gravity,
    },
    states::{
        loading::LoadingState, pause::PauseState, win::WinState, ExtendedStateEvent, PingEvent,
    },
    WorldDef,
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

const PING_PLAYER_SCALE: f32 = 5.0;
const CHARA_WIDTH: f32 = 0.3;
const CHARA_HEIGHT: f32 = 0.7;

const WIN_SCORE: u32 = 5;

#[derive(Default)]
pub struct PingState<'a, 'b> {
    player_initmotion_timer: Option<f32>,
    player1_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    cpu_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    progress_counter: Option<ProgressCounter>,
    entities: Vec<Entity>,
    score: [u32; 2], // index 0 is p1, index 1 is p2 or cpu.
    paused: bool,
    is_pressed: bool,  // ゲームの流れが終了したかどうか
    is_game_end: bool, // 全てのゲームが終了したかどうか
    count_after_pressed: usize,
    ui_root: Option<Entity>,
    score_ui: Option<Entity>,
    past_ui: Option<Entity>,
    win_ui_root: Option<Entity>,
    win_ui: Option<Entity>,
    sky_num: usize,
}

impl<'a, 'b, 'c, 'd> State<GameData<'c, 'd>, ExtendedStateEvent> for PingState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.progress_counter = Some(Default::default());

        let world_def = {
            let screen_size = super::get_screensize(world);
            WorldDef {
                screen_width: screen_size.0,
                screen_height: screen_size.1,
            }
        };

        // world.add_resource(world_def);
        world.insert(world_def); // NOTE: changed by rust-analyzer

        use crate::systems::{
            backgrounds::BackgroundsSystem, chara_animation::PingCharaAnimationSystem,
            exclamationmark::ExclamationmarkSystem, gravity_collision::*, player::PlayerSystem,
            stages::*,
        };
        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(PlayerSystem, "player_system", &[]);
        dispatcher_builder.add(
            PingCharaAnimationSystem::default(),
            "chara_amimation_system",
            &[],
        );
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

        if self.entities.is_empty() {
            self.init_chara(world, PlayerNumber::P2);
            init_camera(world);
            self.init_exclamationmark(world);
            self.init_backgrounds(world);
            self.init_stage(world);
            self.ui_root = Some(world.exec(|mut creator: UiCreator<'_>| {
                creator.create("ui/score.ron", self.progress_counter.as_mut().unwrap())
            }));
            self.win_ui_root = Some(world.exec(|mut creator: UiCreator<'_>| {
                creator.create("ui/win.ron", self.progress_counter.as_mut().unwrap())
            }));
        }

        self.remove_hidden_onSky(world);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        if self.is_game_end {
            world
                .delete_entities(&self.entities)
                .expect("Failed to remove PingState");
            self.entities.clear();

            if let Some(root_entity) = self.ui_root {
                world
                    .delete_entity(root_entity)
                    .expect("Failed to remove ping ui_root");
                self.ui_root = None;
            }

            self.score_ui = None;
            self.past_ui = None;
        } else {
            world.exec(
                |(entitys, mut hiddens, exclamationmarks): (
                    Entities,
                    WriteStorage<Hidden>,
                    ReadStorage<Exclamationmark>,
                )| {
                    for (entity, exclamationmark) in (&entitys, &exclamationmarks).join() {
                        hiddens
                            .insert(entity, Hidden::default())
                            .expect("Failed to insert hiddens");
                    }
                },
            )
        }
    }

    fn update(
        &mut self,
        mut data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'c, 'd>, ExtendedStateEvent> {
        let mut world = &mut data.world;
        // It is absolutely necessary
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

        // this cannot happen in 'on_start', as the entity might not be fully
        // initialized/registered/created yet.
        if self.score_ui.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                self.score_ui = finder.find("score");
            })
        }
        if self.past_ui.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                self.past_ui = finder.find("past_frame");
            })
        }
        if self.win_ui.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                self.win_ui = finder.find("win");
            })
        }

        world.exec(|mut ui_text: WriteStorage<UiText>| {
            if let Some(score) = self.score_ui.and_then(|entity| ui_text.get_mut(entity)) {
                score.text = format!("{}     {}", self.score[0], self.score[1]);
            }
        });

        if self.is_pressed {
            self.count_after_pressed += 1;

            if self.count_after_pressed > crate::FRAME_RATE * 2 {
                // TODO Switch後の演出実装
                return Trans::Switch(Box::new(PingState::new_from(&self))); // NOTE cloneの代用．self.cloneではコンパイルが通らない．要調査．
            }
        }

        if self.score[0] >= WIN_SCORE || self.score[1] >= WIN_SCORE {
            world.exec(
                |(entitys, mut hiddens, exclamationmarks, mut ui_text): (
                    Entities,
                    WriteStorage<Hidden>,
                    ReadStorage<Exclamationmark>,
                    WriteStorage<UiText>,
                )| {
                    if let Some(win) = self.win_ui.and_then(|entity| ui_text.get_mut(entity)) {
                        if self.score[0] > self.score[1] {
                            win.text = format!("Player1 WIN");
                        } else {
                            win.text = format!("Player2 WIN");
                        }
                    }
                    for (entity, exclamationmark) in (&entitys, &exclamationmarks).join() {
                        hiddens
                            .insert(entity, Hidden::default())
                            .expect("Failed to insert hiddens");
                    }
                },
            );
            return Trans::Push(Box::new(WinState::new(self.win_ui_root.unwrap().clone())));
        }

        Trans::None
    }

    fn shadow_update(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // doneTODO: ポーズ時にuiをHideする
        let ui_entites = [self.score_ui, self.past_ui];
        let mut entites_iter = ui_entites.iter();
        if self.paused {
            while let Some(Some(ui)) = entites_iter.next() {
                world.exec(|mut hidden: WriteStorage<HiddenPropagate>| {
                    if hidden.get(*ui).is_none() {
                        hidden
                            .insert(*ui, HiddenPropagate::new())
                            .expect("Failed to insert HiddenPropagate");
                        log::debug!("{:?} insert HiddenPropagate", ui);
                    }
                })
            }
        }

        let mut entites_iter = ui_entites.iter();
        if !self.paused {
            while let Some(Some(ui)) = entites_iter.next() {
                world.exec(|mut hidden: WriteStorage<HiddenPropagate>| {
                    if hidden.get(*ui).is_some() {
                        hidden
                            .remove(*ui)
                            .expect("Failed to remove HiddenPropagate");
                        log::debug!("{:?} remove HiddenPropagate", ui);
                    }
                });
            }
        }
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: ExtendedStateEvent,
    ) -> Trans<GameData<'c, 'd>, ExtendedStateEvent> {
        match event {
            ExtendedStateEvent::Window(e) => {
                if is_close_requested(&e) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            ExtendedStateEvent::Input(e) => match e {
                InputEvent::ControllerButtonPressed {
                    which: _,
                    button: ControllerButton::Start,
                }
                | InputEvent::ButtonPressed(Button::Key(VirtualKeyCode::Escape)) => {
                    log::debug!("push pause");
                    // Trans::Push(Box::new(WinState::default()))
                    Trans::Push(Box::new(PauseState::default()))
                }
                _ => Trans::None,
            },
            ExtendedStateEvent::Ping(e) => match e {
                PingEvent::P1Win => {
                    log::info!("P1 Win");
                    self.score[0] = self.score[0].saturating_add(1);
                    self.is_pressed = true;
                    Trans::None
                }
                PingEvent::P2Win => {
                    log::info!("P2 Win");
                    self.score[1] = self.score[1].saturating_add(1);
                    self.is_pressed = true;
                    Trans::None
                }
                PingEvent::Draw => {
                    log::info!("Draw");
                    self.is_pressed = true;
                    Trans::None
                }
                PingEvent::P1Flying => {
                    log::info!("P1 Flying");
                    self.score[0] = self.score[0].saturating_sub(1);
                    self.is_pressed = true;
                    Trans::None
                }
                PingEvent::P2Flying => {
                    log::info!("P2 Flying");
                    self.score[1] = self.score[1].saturating_sub(1);
                    self.is_pressed = true;
                    Trans::None
                }
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.paused = true;
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.paused = false;
    }
}

impl<'a, 'b> Clone for PingState<'a, 'b> {
    fn clone(&self) -> Self {
        PingState {
            player_initmotion_timer: None,
            player1_sprite_sheet_handle: self.player1_sprite_sheet_handle.clone(),
            cpu_sprite_sheet_handle: self.cpu_sprite_sheet_handle.clone(),
            entities: self.entities.clone(),
            score: self.score.clone(), // index 0 is p1, index 1 is p2 or cpu.
            paused: self.paused.clone(),
            is_pressed: self.is_pressed.clone(), // ゲームの流れが終了したかどうか
            count_after_pressed: self.count_after_pressed.clone(),
            ui_root: self.ui_root.clone(),
            score_ui: self.score_ui.clone(),
            past_ui: self.past_ui.clone(),
            ..Default::default()
        }
    }
}

use std::marker::{Send, Sync};
impl<'a, 'b> PingState<'a, 'b> {
    pub fn new_from(s: &PingState) -> Self {
        PingState {
            // Takeover data
            score: s.score,
            entities: s.entities.clone(),
            ui_root: s.ui_root.clone(),
            score_ui: s.score_ui.clone(),
            past_ui: s.past_ui.clone(),
            is_game_end: s.is_game_end.clone(),
            win_ui_root: s.win_ui_root.clone(),
            sky_num: (s.sky_num + 1) % 3,
            ..Default::default()
        }
    }

    fn init_chara(&mut self, world: &mut World, p2_mode: PlayerNumber) {
        let (screen_width, screen_height) = super::get_screensize(world);
        let p1_path = "texture/HeavyBandit";
        let p2_path = "texture/LightBandit";
        let x = screen_width * CHARA_WIDTH;
        let y = screen_height * CHARA_HEIGHT;
        let mut p1_transform = Transform::default();
        p1_transform.set_translation_xyz(x, y, 0.0);
        p1_transform.set_scale(Vector3::new(-PING_PLAYER_SCALE, PING_PLAYER_SCALE, 1.0));
        let p1_sprite_render = SpriteRender {
            sprite_sheet: super::load_sprite_sheet(
                world,
                p1_path,
                self.progress_counter.as_mut().unwrap(),
            ),
            sprite_number: 0,
        };
        let mut p2_transform = Transform::default();
        p2_transform.set_translation_xyz(screen_width - x, y, 0.0);
        p2_transform.set_scale(Vector3::new(PING_PLAYER_SCALE, PING_PLAYER_SCALE, 1.0));
        let p2_sprite_render = SpriteRender {
            sprite_sheet: super::load_sprite_sheet(
                world,
                p2_path,
                self.progress_counter.as_mut().unwrap(),
            ),
            sprite_number: 0,
        };
        let prefab = world.exec(|loader: PrefabLoader<'_, PlayerPrefabData>| {
            loader.load(
                "prefab/player_animation.ron",
                RonFormat,
                self.progress_counter.as_mut().unwrap(),
            )
        });

        self.entities.push(
            world
                .create_entity()
                .with(prefab.clone())
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
                .build(),
        );
        self.entities.push(
            world
                .create_entity()
                .with(prefab)
                .with(PingPlayer::new(p2_mode))
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
                .build(),
        );
    }
    fn init_exclamationmark(&mut self, world: &mut World) {
        let (screen_width, screen_height) = super::get_screensize(world);
        let mut transform = Transform::default();
        transform.set_translation_xyz(screen_width * 0.5, screen_height * 0.5, 0.1);
        transform.set_scale(Vector3::new(0.25, 0.25, 1.0));
        let sprite_render = SpriteRender {
            sprite_sheet: super::load_sprite_sheet(
                world,
                "texture/exclamationmark",
                self.progress_counter.as_mut().unwrap(),
            ),
            sprite_number: 0,
        };

        self.entities.push(
            world
                .create_entity()
                .with(Exclamationmark)
                .with(transform)
                .with(sprite_render)
                .with(Hidden)
                .build(),
        );

        // world.exec(|mut exclamationmark: WriteStorage<Exclamationmark>| {
        //     exclamationmark
        //         .insert(entity, Exclamationmark)
        //         .expect("Failed to Exclamationmark insert");
        // });
        // self.entities.push(entity);
        // world.add_resource(ExclamationmarkResources::new(sprite_render, transform));
    }
    fn init_backgrounds(&mut self, world: &mut World) {
        let image_paths = [
            ("day-beach-sky", "day-backgrounds", SkyTime::Day),
            ("sunset-beach-sky", "sunset-backgrounds", SkyTime::Sunset),
            ("night-beach-sky", "night-backgrounds", SkyTime::Night),
        ];

        for (sky, backgrounds_path, skytime) in image_paths.iter() {
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
                sprite_sheet: super::load_sprite_sheet(
                    world,
                    &format!("backgrounds/{}", sky),
                    self.progress_counter.as_mut().unwrap(),
                ),
                sprite_number: 0,
            };
            // cloud setting
            let backgrounds_path = &(String::from("backgrounds/") + *backgrounds_path);
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
            let backgrounds_sprite_sheet = super::load_sprite_sheet(
                world,
                backgrounds_path,
                self.progress_counter.as_mut().unwrap(),
            );
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
                    Cloud::new(size.mul(cloud_scale), *skytime),
                    Cloud::new(size2.mul(cloud_scale), *skytime),
                )
            };
            self.entities.push(
                world
                    .create_entity()
                    .with(Background::new(*skytime))
                    .with(transform)
                    .with(sprite_render)
                    .with(Hidden)
                    .build(),
            );
            self.entities.push(
                world
                    .create_entity()
                    .with(cloud)
                    .with(cloud_transform)
                    .with(cloud_sprite_render)
                    .with(Hidden)
                    .build(),
            );
            self.entities.push(
                world
                    .create_entity()
                    .with(cloud2)
                    .with(cloud2_transform)
                    .with(cloud2_sprite_render)
                    .with(Hidden)
                    .build(),
            );

        }
    }
    fn init_stage(&mut self, world: &mut World) {
        let (screen_width, screen_height) = super::get_screensize(world);
        let stage_path = "stages/TX Tileset Ground";
        let sprite_sheet =
            super::load_sprite_sheet(world, stage_path, self.progress_counter.as_mut().unwrap());
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
                self.entities.push(
                    world
                        .create_entity()
                        .with(Stage)
                        .with(transform)
                        .with(sprite_render)
                        .with(GeneralData::default().with_size((sprite_width, sprite_height)))
                        .build(),
                );
            }
        }
    }

    fn remove_hidden_onSky(&mut self, world: &mut World) {
        world.exec(
            |(mut hiddens, entites, backgrounds, clouds): (
                WriteStorage<Hidden>,
                Entities,
                ReadStorage<Background>,
                ReadStorage<Cloud>,
            )| {
                let sky = [SkyTime::Day, SkyTime::Sunset, SkyTime::Night];
                for (entity, background) in (&entites, &backgrounds).join() {
                    if background.skytime == sky[self.sky_num] {
                        if let Some(_) = hiddens.get(entity) {
                            hiddens.remove(entity).unwrap();
                        }
                    } else {
                        if let None = hiddens.get(entity) {
                            hiddens.insert(entity, Hidden).unwrap();
                        }
                    }
                }
                for (entity, cloud) in (&entites, &clouds).join() {
                    if cloud.skytime == sky[self.sky_num] {
                        if let Some(_) = hiddens.get(entity) {
                            hiddens.remove(entity).unwrap();
                        }
                    } else {
                        if let None = hiddens.get(entity) {
                            hiddens.insert(entity, Hidden).unwrap();
                        }
                    }
                }
            },
        )
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
