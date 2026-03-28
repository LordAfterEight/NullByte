pub struct Player {
    pub save_dir: String,
    pub username: String,
    pub devices: Vec<Box<dyn crate::core::device::DeviceTrait>>,
}

impl Player {
    pub fn from_file(src: &str) -> Self {
        Self {
            save_dir: src.to_string(),
            username: "Default".to_string(),
            devices: Vec::new(),
        }
    }

    pub fn new(name: &str) -> Self {
        Self {
            save_dir: "".to_string(),
            username: name.to_string(),
            devices: Vec::new()
        }
    }
}
