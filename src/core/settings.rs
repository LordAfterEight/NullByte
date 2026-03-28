#[derive(serde::Serialize)]
pub struct Settings {
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub ambience_volume: f32,
    pub resolution_width: usize,
    pub resolution_height: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            music_volume: 0.5,
            sfx_volume: 0.5,
            ambience_volume: 0.5,
            resolution_width: 1920,
            resolution_height: 1080
        }
    }
}
