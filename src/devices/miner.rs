pub struct Miner {
    kind: crate::core::device::DeviceKind
}

impl crate::core::device::Initializable for Miner {
    fn init() -> Self {
        Self {
            kind: crate::core::device::DeviceKind::Miner
        }
    }
}
