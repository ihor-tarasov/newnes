use super::UnitWrapper;

pub struct OffsetUnit {
    offset: u16,
    unit: UnitWrapper,
}

impl OffsetUnit {
    pub fn new(offset: u16, unit: UnitWrapper) -> Self {
        Self { offset, unit }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.unit.read(self.offset.wrapping_add(address))
    }

    pub fn write(&self, address: u16, byte: u8) {
        self.unit.write(self.offset.wrapping_add(address), byte)
    }
}
