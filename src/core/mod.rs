pub struct Game {
    pub window: minifb_ui::window::Window,
    pub sfx_channels: Vec<rotilities::Sink>,
    pub mus_channels: Vec<rotilities::Sink>,
}

impl Game {
    pub fn init() -> Self {
        Self {
            window: minifb_ui::window::Window::custom("NullByte", 1920, 1080, false, false),
            sfx_channels: Vec::new(),
            mus_channels: Vec::new()
        }
    }
}
