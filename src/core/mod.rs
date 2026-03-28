mod audio_manager;
pub mod device;
pub mod player;
mod settings;

use minifb_ui::Key;
use minifb_ui::color::Color;
use minifb_ui::ttf::Font;
use minifb_ui::ui::button::*;
use minifb_ui::ui::text::Text;

pub struct Game {
    pub window: minifb_ui::window::Window,
    pub player: Option<player::Player>,
    pub audio_manager: audio_manager::AudioManager,
    pub settings: settings::Settings,
    pub instant_start: std::time::Instant,
}

impl Game {
    pub fn init() -> Self {
        let window = minifb_ui::window::Window::custom("NullByte", 1920, 1080, true, false);
        Self {
            window: window,
            player: None,
            audio_manager: audio_manager::AudioManager::init(),
            settings: settings::Settings::default(),
            instant_start: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.audio_manager
            .set_volume("Ambience", self.settings.ambience_volume);
        self.audio_manager
            .set_volume("SFX", self.settings.sfx_volume);
        self.audio_manager
            .set_volume("Music", self.settings.music_volume);
        self.window.update();
        self.audio_manager.update();
    }

    pub fn title_screen(&mut self) {
        let title = Text::new("NullByte", Font::new("assets/fonts/good timing bd.otf"));
        self.window.draw_text(
            self.settings.resolution_width / 2 - title.get_width(40.0) / 2,
            self.settings.resolution_height / 3,
            &title,
            40.0,
            &Color::from(0xAA2222),
        );

        let mut play_button = Button::default()
            .label("Play", Font::new("assets/fonts/good timing bd.otf"), 20.0)
            .position(
                self.settings.resolution_width / 2 - 25,
                self.settings.resolution_height / 2,
            )
            .size(70, 40)
            .idle_bg(Color::from(0x333333))
            .hover_bg(Color::from(0x444444))
            .click_bg(Color::from(0x222222))
            .idle_label_col(Color::from(0xCCCCCC))
            .hover_label_col(Color::from(0xDDDDDD))
            .click_label_col(Color::from(0xAAFFAA))
            .label_alignment(Alignment::Center)
            .border(2);

        self.window.draw_line(
            (self.settings.resolution_width / 2 - 100) as isize,
            (self.settings.resolution_height / 3 + 50) as isize,
            (self.settings.resolution_width / 2 + 100) as isize,
            (self.settings.resolution_height / 3 + 50) as isize,
            2,
            Color::from(0xAAAAAA),
        );

        while self.window.window.is_open() {
            if !self.audio_manager.has_next("Ambience") {
                self.audio_manager
                    .set_next("Ambience", "assets/sound/NullByte Computer Ambience.wav");
            }
            if self.window.window.is_key_down(minifb_ui::Key::Escape) {
                self.audio_manager.interrupt(
                    "Ambience",
                    "assets/sound/NullByte Computer Ambience End.wav",
                );
                while self.window.window.is_open() && !self.audio_manager.is_finished("Ambience") {
                    self.window.update();
                    self.audio_manager.update();
                }
                std::process::exit(0);
            }
            play_button.draw(&mut self.window);
            self.update();
        }
    }
}
