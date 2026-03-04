use std::{cell::RefCell, rc::Rc};

use crate::{bus::Device, devices::rom::Rom};

pub struct ROMSelectRegister {
    paged_rom: Rc<RefCell<PagedRom>>,
}

impl ROMSelectRegister {
    pub fn default(paged_rom: Rc<RefCell<PagedRom>>) -> Self{
        Self { paged_rom: paged_rom }
    }
}

impl Device for ROMSelectRegister {
    #[allow(unused_variables)]
    fn write(&mut self, addr: u16, value: u8) {
        self.paged_rom.borrow_mut().select_rom(value);
    }

    #[allow(unused_variables)]
    fn tick(&mut self) -> bool {true}
    
    #[allow(unused_variables)]
    fn read(&self, addr: u16) -> u8 {0}
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
        if self.roms.len() != 0 && rom < (self.roms.len() - 1) as u8{
            self.rom = rom;
            return true;
        }
        false
    }

    pub fn add_rom(&mut self, data: Vec<u8>) {
        self.roms.push(Rom::default(data))
    }
}

impl Device for Rc<RefCell<PagedRom>> {
    fn read(&self, addr: u16) -> u8 {
        self.borrow_mut().roms[self.borrow().rom as usize].read(addr)
    }

    #[allow(unused_variables)]
    fn write(&mut self, addr: u16, value: u8) {}

    #[allow(unused_variables)]
    fn tick(&mut self) -> bool {true}
}
