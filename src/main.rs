use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Event, Pipeline,
    RenderBundle, Stage, VirtualKeyCode};

use amethyst::core::transform::TransformBundle;

use amethyst::input::InputBundle;


mod pong;
mod systems;
use crate::pong::Pong;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let path = "./resources/display_config.ron";
    let config = DisplayConfig::load(&path);

    let binding_path = format!(
        "./resources/bindings_config.ron"
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;


    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.143, 0.168, 0.162, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);
    let mut game = Application::new("./", Pong, game_data)?;
    game.run();

    Ok(())
}