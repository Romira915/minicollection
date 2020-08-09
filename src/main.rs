use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

extern crate minicollection as lib;
use lib::{bundle::Bundle, states::ping::PingState};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = config_dir.join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear([
                        10. / 255.,
                        10. / 255.,
                        10. / 255.,
                        1.0,
                    ]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(Bundle)?;

    let mut game = Application::build(assets_dir, PingState::default())?
        .with_frame_limit(FrameRateLimitStrategy::Yield, lib::FRAME_RATE as u32)
        .build(game_data)?;
    // let mut game = Application::new(assets_dir, PingState::default(), game_data)?;
    game.run();

    Ok(())
}
