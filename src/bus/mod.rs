use std::{collections::BTreeMap, ops::RangeInclusive};

mod memory_range;
mod offset_unit;
mod unit;
mod unit_wrapper;

pub use unit::*;
pub use unit_wrapper::*;

use memory_range::MemoryRange;
use offset_unit::OffsetUnit;

pub struct Bus(BTreeMap<MemoryRange, OffsetUnit>);

impl Bus {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, range: RangeInclusive<u16>, unit: UnitWrapper) {
        self.0.insert(
            MemoryRange::new(*range.start(), *range.end()),
            OffsetUnit::new(*range.start(), unit),
        );
    }

    pub fn read(&self, address: u16) -> u8 {
        self.0
            .get(&MemoryRange::from_address(address))
            .map(|entry| entry.read(address))
            .unwrap_or(0)
    }

    pub fn write(&self, address: u16, byte: u8) {
        self.0
            .get(&MemoryRange::from_address(address))
            .map(|entry| entry.write(address, byte));
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        u16::from_be_bytes([self.read(address), self.read(address.wrapping_add(1))])
    }

    pub fn write_u16(&self, address: u16, word: u16) {
        let bytes = word.to_be_bytes();
        self.write(address, bytes[0]);
        self.write(address.wrapping_add(1), bytes[1])
    }
}
