pub trait Device: 'static {
    fn id(&self) -> u32;
    fn name(&self) -> &str;
    fn tick(&mut self);
}
