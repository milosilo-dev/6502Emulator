use std::fs;

use crate::bus::{Device, TickReturn};

pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn default(data: Vec<u8>) -> Self {
        Self {
            data,
        }
    }

    pub fn load(path: &str) -> Option<Self> {
        let contents = fs::read(path).unwrap_or(vec![]);
        if contents.len() == 0 {
            return None
        }

        let rom = Self{
            data: contents
        };

        Some(rom)
    }
}

impl Device for Rom {
    fn read(&mut self, addr: u16) -> u8 {
        if self.data.len() > addr as usize{
            self.data[addr as usize]
        } else {
            0
        }
    }

    #[allow(unused_variables)]
    fn write(&mut self, addr: u16, value: u8) {}

    #[allow(unused_variables)]
    fn tick(&mut self) -> TickReturn {TickReturn::NONE}
}