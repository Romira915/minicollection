pub mod bundle;
pub mod components;
pub mod states;
pub mod systems;

pub const FRAME_RATE: usize = 60;

pub struct WorldDef {
    screen_width: f32,
    screen_height: f32,
}