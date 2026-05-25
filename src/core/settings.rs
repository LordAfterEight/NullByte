use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub sound: SoundSettings,
    pub window: WindowSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub ambience_volume: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub resizable: bool,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 1.0,
            sfx_volume: 1.0,
            ambience_volume: 1.0,
        }
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fullscreen: true,
            resizable: false,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sound: SoundSettings::default(),
            window: WindowSettings::default(),
        }
    }
}

impl Settings {
    pub fn save_toml(&self, path: &str) {
        let toml_string = toml::to_string_pretty(self).unwrap();
        std::fs::write(path, toml_string).unwrap();
    }

    pub fn load_toml(path: &str) -> Self {
        let toml_string = std::fs::read_to_string(path).unwrap();
        toml::from_str(&toml_string).unwrap()
    }

    pub fn load_or_default(path: &str) -> Self {
        if std::path::Path::new(path).exists() {
            Self::load_toml(path)
        } else {
            let settings = Self::default();
            settings.save_toml(path);
            settings
        }
    }
}
