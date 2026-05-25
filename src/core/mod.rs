mod audio_manager;
pub mod device;
pub mod player;
pub mod settings;

pub struct Game {
    pub player: Option<player::Player>,
    pub audio_manager: audio_manager::AudioManager,
    pub settings: settings::Settings,
    pub instant_start: std::time::Instant,
}

impl Game {
    pub fn init(settings: settings::Settings) -> Self {
        Self {
            player: None,
            audio_manager: audio_manager::AudioManager::init(),
            settings,
            instant_start: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let master = self.settings.sound.master_volume;
        self.audio_manager
            .set_volume("Ambience", self.settings.sound.ambience_volume * master);
        self.audio_manager
            .set_volume("SFX", self.settings.sound.sfx_volume * master);
        self.audio_manager
            .set_volume("Music", self.settings.sound.music_volume * master);
        self.audio_manager.update();
    }
}
