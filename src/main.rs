#[allow(non_snake_case)]
use NullByte;
use NullByte::core::{Game, player};
use NullByte::devices;

const TITLE_COLOR: winit_ui::color::Color = winit_ui::color::Color::new(0xFF, 0x00, 0x00, 0xFF);

fn main() {
    let settings = NullByte::core::settings::Settings::load_or_default("settings.toml");

    let mut app = winit_ui::App::new(
        "NullByte",
        settings.window.width,
        settings.window.height,
        settings.window.resizable,
        settings.window.fullscreen,
    );

    let mut game = Game::init(settings);

    game.player = Some(player::Player::new("Default Player Name"));
    game.player
        .as_mut()
        .unwrap()
        .add_device(devices::miner::Miner::new(0));

    game.audio_manager.new_channel(
        "assets/sound/NullByte Computer Ambience Start.wav",
        "Ambience",
    );
    game.audio_manager.set_next("Ambience", "assets/sound/NullByte Computer Ambience.wav");

    let font_regular = winit_ui::font::Font::new("assets/fonts/IBMPlexMono-Regular.ttf");

    app.on_draw(move |canvas| {
        if !game.audio_manager.has_next("Ambience") {
            game.audio_manager.set_next("Ambience", "assets/sound/NullByte Computer Ambience.wav");
        }


        canvas.draw_text(
            canvas.width / 2 - winit_ui::get_text_width("NullByte", 96.0, &font_regular) / 2,
            canvas.height / 4,
            96.0, "NullByte",
            &font_regular,
            &TITLE_COLOR
        );
        game.update();
    });
    app.run();
}
