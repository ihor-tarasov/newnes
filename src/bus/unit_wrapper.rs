use std::{cell::RefCell, rc::Rc};

use super::{RefUnit, Unit};

#[derive(Clone)]
pub struct UnitWrapper(Rc<RefCell<dyn Unit>>);

impl UnitWrapper {
    pub fn new<U>(unit: U) -> Self
    where
        U: Unit + 'static,
    {
        Self(Rc::new(RefCell::new(unit)))
    }
}

impl RefUnit for UnitWrapper {
    fn read(&self, address: u16) -> u8 {
        self.0.borrow().read(address)
    }

    fn write(&self, address: u16, byte: u8) {
        self.0.borrow_mut().write(address, byte)
    }
}
