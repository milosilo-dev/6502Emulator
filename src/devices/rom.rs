use std::fs;

use crate::bus::Device;

pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn default(data: Vec<u8>) -> Self {
        Self {
            data,
        }
    }

    pub fn load(&mut self, path: &str) -> bool {
        let contents = fs::read(path).unwrap_or(vec![]);
        if contents.len() == 0 {
            return false
        }

        let mut addr = 0;
        for byte in contents{
            self.data[addr] =  byte;
            if addr != self.data.len() - 1{
                addr+=1;
            }
        }

        return true;
    }
}

impl Device for Rom {
    fn read(&self, addr: u16) -> u8 {
        if self.data.len() > addr as usize{
            self.data[addr as usize]
        } else {
            0
        }
    }

    #[allow(unused_variables)]
    fn write(&mut self, addr: u16, value: u8) {}

    #[allow(unused_variables)]
    fn tick(&mut self) -> bool {true}
}