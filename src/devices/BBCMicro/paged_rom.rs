use crate::bus::Device;

struct Rom {
    data: [u8; 16383],
}

impl Rom {
    pub fn default(data: [u8; 16383]) -> Self {
        Self {
            data,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }
}

pub struct PagedRom {
    roms: Vec<Rom>,
    rom: u8,
}

impl PagedRom {
    pub fn default() -> Self {
        Self {
            roms: vec![],
            rom: u8::max_value()
        }
    }

    pub fn select_rom(&mut self, rom: u8) -> bool{
        if rom < (self.roms.len() - 1) as u8{
            self.rom = rom;
            return true;
        }
        false
    }

    pub fn add_rom(&mut self, data: [u8; 16383]) {
        self.roms.push(Rom::default(data))
    }
}

impl Device for PagedRom {
    fn read(&self, addr: u16) -> u8 {
        self.roms[self.rom as usize].read(addr - 0x8000)
    }

    #[allow(unused_variables)]
    fn write(&mut self, addr: u16, value: u8) {}
}
