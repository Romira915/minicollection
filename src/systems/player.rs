use crate::components::player::Ping_Player;
use amethyst::core::{math::*, SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

const PI: f32 = 3.14159265359;

#[derive(SystemDesc)]
pub struct Player_System;

impl<'s> System<'s> for Player_System {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ping_Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, Ping_Player, input): Self::SystemData) {
        for (p_player, transform) in (&Ping_Player, &mut transforms).join() {
            let movement_y = input.axis_value("updown");
            let movement_x = input.axis_value("leftright");

            if let Some(mv_amount_y) = movement_y {
                let scaled_amount = 1.4 * mv_amount_y as f32;
                transform.prepend_translation_y(scaled_amount);
                // transform.set_scale(Vector3::new(0.0, 0.0, 0.0));
                transform.set_scale(
                    *transform.scale()
                        + Vector3::new(mv_amount_y * 0.5, mv_amount_y * 0.5, mv_amount_y * 0.5),
                );
            }
            if let Some(mv_amount_x) = movement_x {
                let scaled_amount = 1.4 * mv_amount_x as f32;
                transform.prepend_translation_x(scaled_amount);
                // transform.set_scale(Vector3::new(2.0, 2.0, 1.0));
                transform.set_scale(
                    *transform.scale()
                        + Vector3::new(mv_amount_x * -0.5, mv_amount_x * -0.5, mv_amount_x * -0.5),
                );
            }
            if let Some(rotation) = input.action_is_down("rotation") {
                let rotation = 0.05 * rotation as i32 as f32 * PI;
                transform.prepend_rotation_z_axis(rotation);
            }
        }
    }
}
