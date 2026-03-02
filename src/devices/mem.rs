use std::{cell::RefCell, rc::Rc};

use crate::bus::Device;

pub struct Mem {
    data: Vec<u8>,
}

impl Mem {
    pub fn default(len: usize) -> Self {
        Self {
            data: vec![0 as u8; len],
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Device for Mem {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    #[allow(unused_variables)]
    fn tick(&mut self) {}
}

impl Device for Rc<RefCell<Mem>> {
    fn read(&self, addr: u16) -> u8 {
        self.borrow().read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.borrow_mut().write(addr, value);
    }

    fn tick(&mut self) {}
}
