pub trait Initializable {
    fn init() -> Self;
}

pub trait DeviceTrait {
    fn tick(&self) -> Result<(), DeviceError>;
}

#[derive(Debug)]
pub struct Device<T> {
    pub id: u32,
    pub inner: T,
}

impl<T> Device<T> {
    pub fn update() -> Result<(), DeviceError> {
        Ok(())
    }
}

impl<T: Initializable> Device<T> {
    /// Creates a brand new Device.
    /// Should only ever be run once per device, even across saves.
    /// Device data such as its ID should be loaded from save files.
    pub fn create() -> Self {
        Self {
            id: rand::random(),
            inner: T::init(),
        }
    }
}

impl<T> DeviceTrait for Device<T> {
    fn tick(&self) -> Result<(), DeviceError> {
        Device::<T>::update()
    }
}

impl std::fmt::Display for dyn DeviceTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub enum DeviceError {
    LoadingFailed(String),
    UpdateFailed(String),
}

#[derive(Copy, Clone, Debug)]
pub enum DeviceKind {
    Miner,
    Compressor,
    Analyzer,
    Extractor,
    Assembler,
}

impl std::fmt::Display for DeviceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            DeviceKind::Miner => f.write_str("Miner"),
            DeviceKind::Compressor => f.write_str("Compressor"),
            DeviceKind::Analyzer => f.write_str("Analyzer"),
            DeviceKind::Extractor => f.write_str("Extractor"),
            DeviceKind::Assembler => f.write_str("Assembler"),
            #[allow(unused)]
            _ => f.write_str("Invalid/Unknown Device"),
        };
    }
}
