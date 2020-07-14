use crate::components::{
    player::{PingPlayer, PlayerState},
    GeneralData,
};
use amethyst::{
    core::{math::*, timing::Time, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

const PI: f32 = 3.14159265359;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, PingPlayer>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
        WriteStorage<'s, GeneralData>,
    );

    fn run(
        &mut self,
        (mut ping_players, mut transforms, input, mut sprites, time, mut generaldatas): Self::SystemData,
    ) {
        for (p_player, transform, sprite, generaldata) in (
            &mut ping_players,
            &mut transforms,
            &mut sprites,
            &mut generaldatas,
        )
            .join()
        {
            if let Some(mv_amount_x) = input.axis_value("x_axis") {
                if mv_amount_x != 0.0 {
                    generaldata.velocity.x =
                        mv_amount_x * (generaldata.velocity.x.abs() + 50.0).min(300.0);
                } else {
                    generaldata.velocity.x = (generaldata.velocity.x * 0.7);
                    if generaldata.velocity.x.abs() < 1.0 {
                        generaldata.velocity.x = 0.0;
                    }
                }
            }
            // if let Some(mv_amount_y) = input.axis_value("y_axis") {
            //     generaldata.velocity.y +=
            //         mv_amount_y * (generaldata.velocity.y.abs() + 30.0).min(250.0);
            // }
            // if let Some(rotation) = input.action_is_down("rotation") {
            //     let rotation = 0.05 * rotation as i32 as f32 * PI;
            //     transform.prepend_rotation_z_axis(rotation);
            // }

            // transform.append_translation_xyz(
            //     generaldata.velocity.x * time.delta_seconds(),
            //     generaldata.velocity.y * time.delta_seconds(),
            //     0.0,
            // );
            transform.prepend_translation_x(generaldata.velocity.x * time.delta_seconds());
            transform.prepend_translation_y(generaldata.velocity.y * time.delta_seconds());

            if generaldata.velocity.x * -transform.scale().x < 0.0 {
                transform.scale_mut().x *= -1.0;
            }

            if generaldata.velocity.x == 0.0 {
                p_player.state = PlayerState::wait;
            } else {
                p_player.state = PlayerState::run;
            }
        }
    }
}
