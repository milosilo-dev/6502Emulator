use crate::bus::Device;

pub struct Mem {
    data: [u8; 1024 * 64],
}

impl Mem {
    pub fn default() -> Self{
        Self{
            data: [0 as u8; 1024 * 64]
        }
    }

    pub fn len(&self) -> usize{
        self.data.len()
    }
}

impl Device for Mem{
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}