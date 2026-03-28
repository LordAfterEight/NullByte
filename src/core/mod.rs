pub mod device;
pub mod player;
mod audio_manager;

pub static mut TICKS: u64 = 0;
fn advance() {
    unsafe { TICKS += 1; }
}

pub struct Game {
    pub window: minifb_ui::window::Window,
    pub player: Option<player::Player>,
    pub audio_manager: audio_manager::AudioManager,
}

impl Game {
    pub fn init() -> Self {
        Self {
            window: minifb_ui::window::Window::custom("NullByte", 1920, 1080, false, false),
            player: None,
            audio_manager: audio_manager::AudioManager::init()
        }
    }
}
