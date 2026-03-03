use std::{cell::RefCell, rc::Rc};

use crate::{bus::Device, platform::keyboard::Keyboard};

pub struct SystemVIA{
    keyboard: Rc<RefCell<Keyboard>>,
    selected_row: u8
}

impl SystemVIA{
    pub fn default(keyboard: Rc<RefCell<Keyboard>>) -> Self{
        Self {
            keyboard,
            selected_row: 0
        }
    }
}

// MMIO Device to be mapped from 0xFE40 - 0xFE4F
impl Device for SystemVIA{
    fn read(&self, addr: u16) -> u8 {
        match addr{
            2 => {
                // Port B
                let keyboard = self.keyboard.borrow();
                keyboard.get_row(self.selected_row).unwrap_or(0)
            }
            _ => {0}
        }
    }

    #[allow(unused_variables)]
    fn write(&mut self, addr: u16, value: u8) {
        match addr{
            1 => {
                // Port A
                self.selected_row = value;
            }
            _ => {}
        }
    }

    #[allow(unused_variables)]
    fn tick(&mut self) {}
}