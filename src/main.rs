#[allow(non_snake_case)]
use NullByte;
use NullByte::core::{Game, player};
use NullByte::devices;

use minifb_ui::ui::text::Text;

fn main() {
    let mut game = Game::init();
    game.window.window.set_target_fps(30);

    let black = minifb_ui::color::Color::from(0x0);

    game.player = Some(player::Player::new("Default Player Name"));
    game.player
        .as_mut()
        .unwrap()
        .add_device(crate::devices::miner::Miner::new(0));


    game.audio_manager.new_channel(
        "assets/sound/NullByte Computer Ambience Start.wav",
        "Ambience",
    );

    game.audio_manager.set_next("Ambience", "assets/sound/NullByte Computer Ambience.wav");

    let mut amb_vol = Text::new(
        &format!("Ambience volume: {}%", game.settings.ambience_volume * 100.0),
        minifb_ui::ttf::Font::new("assets/fonts/good timing bd.otf"),
        );

    game.title_screen();
}
