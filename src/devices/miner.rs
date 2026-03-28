pub struct Miner {
    id: u32,
    name: String
}

impl Miner {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: "Miner".to_string(),
        }
    }
}

impl crate::core::device::Device for Miner {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn tick(&mut self) {
    }
}
