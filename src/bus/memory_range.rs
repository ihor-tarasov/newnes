use std::cmp::Ordering;

pub struct MemoryRange {
    start: u16,
    end: u16,
}

impl MemoryRange {
    pub fn new(start: u16, end: u16) -> Self {
        Self { start, end }
    }

    pub fn from_address(address: u16) -> Self {
        Self {
            start: address,
            end: address,
        }
    }
}

impl Ord for MemoryRange {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.end < other.start {
            Ordering::Less
        } else if self.start > other.end {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for MemoryRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for MemoryRange {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for MemoryRange {}
