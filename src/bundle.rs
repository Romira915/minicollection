use crate::systems::{
    backgrounds::BackgroundsSystem, chara_animation::PingCharaAnimationSystem,
    exclamationmark::ExclamationmarkSystem, gravity_collision::*, player::PlayerSystem, stages::*,
};
use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

pub struct Bundle;

impl<'a, 'b> SystemBundle<'a, 'b> for Bundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // builder.add(PlayerSystem, "player_system", &["input_system"]);
        // builder.add(PingCharaAnimationSystem, "chara_amimation_system", &[]);
        // builder.add(ExclamationmarkSystem, "exclamationmark_system", &[]);
        // builder.add(BackgroundsSystem, "backgrounds_system", &[]);
        // builder.add(StageSystem, "stage_system", &[]);
        // builder.add(
        //     GravityCollisionSystem,
        //     "gravity_collision_system",
        //     &["player_system", "stage_system"],
        // );
        Ok(())
    }
}
