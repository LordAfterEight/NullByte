#[allow(non_snake_case)]
use NullByte;
use NullByte::core::{Game, player};
use NullByte::devices;
use discord_rich_presence::DiscordIpc;
use winit_ui::font;

const WHITE: winit_ui::color::Color = winit_ui::color::Color::new(0xFF, 0xFF, 0xFF, 0xFF);
const BLACK: winit_ui::color::Color = winit_ui::color::Color::new(0x00, 0x00, 0x00, 0xFF);
const LIGHT_GRAY: winit_ui::color::Color = winit_ui::color::Color::new(0xD0, 0xD0, 0xD0, 0xFF);
const GRAY: winit_ui::color::Color = winit_ui::color::Color::new(0x80, 0x80, 0x80, 0xFF);
const DARK_GRAY: winit_ui::color::Color = winit_ui::color::Color::new(0x20, 0x20, 0x20, 0xFF);
const RED: winit_ui::color::Color = winit_ui::color::Color::new(0xFF, 0x00, 0x00, 0xFF);
const TEAL: winit_ui::color::Color = winit_ui::color::Color::new(0x24, 0xEB, 0xC4, 0xFF);

fn main() {
    let settings = NullByte::core::settings::Settings::load_or_default("settings.toml");
    let mut game = Game::init(settings);

    game.player = Some(player::Player::new("Default Player Name"));
    game.audio_manager
        .new_channel("assets/sound/NullByte [Main Menu Theme].wav", "Music");
    game.audio_manager
        .set_next("Music", "assets/sound/NullByte [Main Menu Theme].wav");
    game.audio_manager.new_channel(
        "assets/sound/NullByte Computer Ambience Start.wav",
        "Ambience",
    );
    game.audio_manager
        .set_next("Ambience", "assets/sound/NullByte Computer Ambience.wav");
    game.instant_start = std::time::Instant::now();
    game.update();

    let fonts = vec![
        winit_ui::font::Font::new("assets/fonts/IBMPlexMono-Light.ttf"),
        winit_ui::font::Font::new("assets/fonts/IBMPlexMono-Regular.ttf"),
        winit_ui::font::Font::new("assets/fonts/IBMPlexMono-Bold.ttf"),
    ];

    let app = winit_ui::App::new("NullByte", game.settings.window.width, game.settings.window.height)
        .keep_aspect_ratio(true)
        .set_resizable(true)
        .on_draw(move |canvas| {
            titlescreen(canvas, &mut game, &fonts);
            game.update();
        });

    let mut discord_client = discord_rich_presence::DiscordIpcClient::new("1424335981036834887");
    discord_client.connect().ok();
    _ = discord_client.set_activity(
        discord_rich_presence::activity::Activity::new()
            .name("NullByte")
            .details("In Main Menu")
            .activity_type(discord_rich_presence::activity::ActivityType::Playing),
    );

    app.run();
}

fn titlescreen(
    canvas: &mut winit_ui::canvas::Canvas,
    game: &mut Game,
    fonts: &Vec<winit_ui::font::Font>,
) {
    let font_light = &fonts[0];
    let font_regular = &fonts[1];
    let font_bold = &fonts[2];

    if !game.audio_manager.has_next("Music") {
        game.audio_manager
            .set_next("Music", "assets/sound/NullByte [Main Menu Theme].wav");
    }
    if !game.audio_manager.has_next("Ambience") {
        game.audio_manager
            .set_next("Ambience", "assets/sound/NullByte Computer Ambience.wav");
    }

    let elapsed = game.instant_start.elapsed().as_millis();
    let visible = (elapsed / 500) % 2 == 0;

    // ========== Draw title ===========
    canvas.draw_text(
        canvas.width / 2 - winit_ui::get_text_width("NullByte", 96.0, font_regular) / 2,
        canvas.height / 4,
        96.0,
        "NullByte",
        font_light,
        &RED,
    );
    canvas.draw_line(
        canvas.width / 2 - winit_ui::get_text_width("NullByte", 96.0, font_regular) / 2 - 100,
        canvas.height / 4 + 30,
        canvas.width / 2 + winit_ui::get_text_width("NullByte", 96.0, font_regular) / 2 + 100,
        canvas.height / 4 + 30,
        &DARK_GRAY,
    );

    // ========== Draw blinking cursor ===========
    if visible {
        canvas.draw_text(
            canvas.width / 2 + winit_ui::get_text_width("NullByte", 96.0, font_bold) / 2,
            canvas.height / 4,
            96.0,
            "_",
            font_regular,
            &GRAY,
        );
    } else {
        canvas.draw_text(
            canvas.width / 2 + winit_ui::get_text_width("NullByte", 96.0, font_bold) / 2,
            canvas.height / 4,
            96.0,
            "_",
            font_regular,
            &BLACK,
        );
    }

    // ========== Draw version and credits ===========
    canvas.draw_text(
        5,
        17,
        12.0,
        &format!("Version {}", env!("CARGO_PKG_VERSION")),
        font_bold,
        &TEAL,
    );
    canvas.draw_text(
        canvas.width - winit_ui::get_text_width("By Elias Stettmayer", 12.0, font_bold) - 5,
        17,
        12.0,
        "By Elias Stettmayer",
        font_bold,
        &TEAL,
    );
}
