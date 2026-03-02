use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, cpu::cpu::CPU, devices::{bbcmicro::{paged_rom::{PagedRom, ROMSelectRegister}, video_system::VideoSystem}, mem::Mem, rom::Rom}, platform::framebuffer::Fb};

pub struct BBCMicro {
    cpu: CPU,
    bus: Bus,
}

impl BBCMicro {
    pub fn new() -> Self{
        let cpu = CPU::default();
        let mut bus = Bus::default();

        let ram = Rc::new(RefCell::new(Mem::default(32 * 1024)));
        bus.register(0..=0x7FFF, Box::new(ram.clone()));

        let paged_rom = Rc::new(RefCell::new(PagedRom::default()));
        bus.register(0x8000..=0xBFFF, Box::new(paged_rom.clone()));

        let os_rom_1 = Rom::default(vec![0; 0xBFFF-0x8000]);
        bus.register(0xC000..=0xFBFF, Box::new(os_rom_1));

        let fb = Box::new(Fb::default());
        let video_system= VideoSystem::default(fb, Rc::clone(&ram));
        bus.register(0xFE00..=0xFE07, Box::new(video_system));

        let page_rom_select = ROMSelectRegister::default(paged_rom);
        bus.register(0xFE30..=0xFE30, Box::new(page_rom_select));

        let os_rom_2 = Rom::default(vec![0; 0xFF00-0xFFFF]);
        bus.register(0xFF00..=0xFFFF, Box::new(os_rom_2));

        Self {
            cpu,
            bus
        }
    }

    pub fn tick(&mut self) {
        let ticks = self.cpu.step(&mut self.bus, 1);
        for _ in 0..ticks{
            self.bus.tick();
        }
    }
}