pub trait Unit {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, byte: u8);
}

pub trait RefUnit {
    fn read(&self, address: u16) -> u8;
    fn write(&self, address: u16, byte: u8);
}
