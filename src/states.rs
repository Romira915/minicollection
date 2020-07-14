pub mod ping;
pub mod pause;

use amethyst::{prelude::*, window::ScreenDimensions};
pub fn get_screensize(world: &mut World) -> (f32, f32) {
    let screen_dimensions = world.read_resource::<ScreenDimensions>();
    (screen_dimensions.width(), screen_dimensions.height())
}

pub fn get_sprite_size(ron_path: &str, sprite_number: usize) -> Option<(f32, f32)> {
    use amethyst::renderer::sprite::{SpriteList, Sprites};
    use std::{fs::File, io::Read};
    let mut buf = String::new();
    File::open(&format!("assets/{}.ron", ron_path))
        .expect("Error: ron file don't open in get_sprite_size")
        .read_to_string(&mut buf)
        .unwrap();

    // let sprite: Sprites = ron_path::from_str(&buf).unwrap();
    if let Sprites::List(sprite_list) = ron::from_str(&buf).unwrap() {
        Some((
            sprite_list.sprites[sprite_number].width as f32,
            sprite_list.sprites[sprite_number].height as f32,
        ))
    } else {
        None
    }
}

use std::ops::Mul;
pub trait Multiplication<T>
where
    T: Mul<Output = T> + Clone,
{
    fn mul(self, n: T) -> Self;
}
