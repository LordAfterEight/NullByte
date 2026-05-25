pub struct Player {
    pub save_dir: String,
    pub username: String,
    pub age: Option<u8>,
    pub gender: Option<String>,
    pub location: Option<String>,
    pub registration_date: Option<String>,
    pub last_login: Option<String>,
    pub domain: Option<String>,
    pub devices: Vec<Box<dyn crate::core::device::Device>>,
}

impl Player {
    pub fn from_file(src: &str) -> Self {
        Self {
            save_dir: src.to_string(),
            username: "Default".to_string(),
            age: None,
            gender: None,
            location: None,
            registration_date: None,
            last_login: None,
            domain: None,
            devices: Vec::new(),
        }
    }

    pub fn new(name: &str) -> Self {
        Self {
            save_dir: "".to_string(),
            username: name.to_string(),
            devices: Vec::new(),
            age: None,
            gender: None,
            location: None,
            registration_date: None,
            last_login: None,
            domain: None,
        }
    }

    pub fn add_device<T: crate::core::device::Device>(&mut self, device: T) -> &mut Self{
        let device = Box::new(device);
        self.devices.push(device);
        self
    }
}
