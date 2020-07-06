use crate::components::player::{PingPlayer, PlayerState};
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
    );

    fn run(
        &mut self,
        (mut ping_players, mut transforms, input, mut sprites, time): Self::SystemData,
    ) {
        for (p_player, transform, sprite) in
            (&mut ping_players, &mut transforms, &mut sprites).join()
        {
            if let Some(mv_amount_x) = input.axis_value("x_axis") {
                p_player.velocity.x = mv_amount_x * 250.;
            }
            if let Some(mv_amount_y) = input.axis_value("y_axis") {
                p_player.velocity.y = mv_amount_y * 250.;
            }
            if let Some(rotation) = input.action_is_down("rotation") {
                let rotation = 0.05 * rotation as i32 as f32 * PI;
                transform.prepend_rotation_z_axis(rotation);
            }

            transform.prepend_translation_x(p_player.velocity.x * time.delta_seconds());
            transform.prepend_translation_y(p_player.velocity.y * time.delta_seconds());

            if p_player.velocity.x * -transform.scale().x < 0.0 {
                transform.scale_mut().x *= -1.0;
            }

            if p_player.velocity[p_player.velocity.iamax()] == 0.0 {
                p_player.state = PlayerState::wait;
            } else {
                p_player.state = PlayerState::run;
            }
        }
    }
}
