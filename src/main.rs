#[allow(non_snake_case)]
use NullByte;
use NullByte::core::{Game, device::*, player};
use NullByte::devices;

fn main() {
    let mut game = Game::init();
    game.window.window.set_target_fps(30);

    let black = minifb_ui::color::Color::from(0x0);

    game.player = Some(player::Player::new("Default Player Name"));
    game.player
        .as_mut()
        .unwrap()
        .devices
        .push(Box::new(Device::<devices::miner::Miner>::create()));

    println!("{}", &game.player.as_ref().unwrap().devices[0]);

    game.audio_manager.new_channel(
        "assets/sound/NullByte Computer Ambience Start.wav",
        "Ambience",
    );

    game.audio_manager.set_next("Ambience", "assets/sound/NullByte Computer Ambience.wav");

    let text = minifb_ui::ui::text::Text::new(
        "Hello World",
        minifb_ui::ttf::Font::new("assets/fonts/good timing bd.otf"),
    );

    while game.window.window.is_open() {
        game.window.clear(&black);
        game.window.draw_text(
            10,
            10,
            &text,
            48.0,
            &minifb_ui::color::Color::from(0xFFFFFF),
        );

        if game.window.window.is_key_down(minifb_ui::Key::Escape) {
            game.audio_manager.interrupt("Ambience", "assets/sound/NullByte Computer Ambience End.wav");
            loop {
                if game.audio_manager.is_finished("Ambience") {
                    std::process::exit(0);
                }
                game.window.update();
                game.audio_manager.update();
            }
        }

        if game.window.window.is_key_down(minifb_ui::Key::Down) {
            game.audio_manager.set_volume("Ambience", game.audio_manager.channels[0].volume - 0.01);
        }

        if game.window.window.is_key_down(minifb_ui::Key::Up) {
            game.audio_manager.set_volume("Ambience", game.audio_manager.channels[0].volume + 0.01);
        }

        game.window.update();
        game.audio_manager.update();
    }
}
