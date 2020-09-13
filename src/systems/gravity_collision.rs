use crate::components::{player::*, stages::*, GeneralData, Gravity};
use amethyst::{
    core::{math::*, timing::Time, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

#[derive(SystemDesc)]
pub struct GravityCollisionSystem;

impl<'s> System<'s> for GravityCollisionSystem {
    type SystemData = (
        WriteStorage<'s, PingPlayer>,
        ReadStorage<'s, Stage>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Gravity>,
        WriteStorage<'s, GeneralData>,
    );
    
    // WARN: 他のシステムに流用できない可能性がある
    fn run(
        &mut self,
        (mut players, stages, mut transforms, gravity, mut generaldatas): Self::SystemData,
    ) {
        for (player, _, generaldata) in (&players, &gravity, &mut generaldatas).join() {
            if !player.is_on_stage {
                generaldata.velocity.y = (generaldata.velocity.y + -20.0).max(-700.0);
            } else {
                generaldata.velocity.y = generaldata.velocity.y.max(0.0);
                // generaldata.velocity.y = 0.0;
            }
        }

        for (player, transform, generaldata) in (&mut players, &transforms, &generaldatas).join() {
            for (_, stage_transform, stage_generaldata) in
                (&stages, &transforms, &generaldatas).join()
            {
                player.is_on_stage = collison(
                    &transform,
                    &generaldata,
                    &stage_transform,
                    &stage_generaldata,
                );
                if player.is_on_stage {
                    break;
                }
            }
        }
    }
}

fn collison(A: &Transform, dataA: &GeneralData, B: &Transform, dataB: &GeneralData) -> bool {
    ((A.translation().x - B.translation().x).abs() < dataA.width / 2.0 + dataB.width / 2.0)
        && ((A.translation().y - B.translation().y).abs() < dataA.height / 2.0 + dataB.height / 2.0)
}
