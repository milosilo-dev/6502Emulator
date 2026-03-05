use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, cpu::cpu::CPU, devices::{bbcmicro::{paged_rom::{PagedRom, ROMSelectRegister}, system_via::SystemVIA, video_system::VideoSystem}, mem::Mem, rom::Rom}, platform::{framebuffer::Fb, keyboard::Keyboard, logging::Stdout}};

pub struct BBCMicro {
    cpu: CPU,
    bus: Bus,
}

impl BBCMicro {
    pub fn new() -> Self{
        let mut cpu = CPU::default();
        cpu.config.logger = Box::new(Stdout{});
        let mut bus = Bus::default();

        let ram = Rc::new(RefCell::new(Mem::default(32 * 1024)));
        bus.register(0..=0x7FFF, Box::new(ram.clone()));

        let paged_rom = Rc::new(RefCell::new(PagedRom::default()));
        let basic = Rom::load("roms/bbc_micro/BASIC2.rom").unwrap_or(Rom::default(vec![0; 0xBFFF - 0x8000 + 1]));
        paged_rom.borrow_mut().add_rom(basic);
        bus.register(0x8000..=0xBFFF, Box::new(paged_rom.clone()));

        let keyboard = Rc::new(RefCell::new(Keyboard::default()));
        let fb = Box::new(Fb::default(keyboard.clone()));
        let video_system= VideoSystem::default(fb, Rc::clone(&ram));
        bus.register(0xFE00..=0xFE07, Box::new(video_system));

        let system_via = SystemVIA::default(Rc::clone(&keyboard));
        bus.register(0xFE40..=0xFE4F, Box::new(system_via));

        let page_rom_select = ROMSelectRegister::default(paged_rom);
        bus.register(0xFE30..=0xFE30, Box::new(page_rom_select));

        let os_rom = Rom::load("roms/bbc_micro/OS-1.2.rom").unwrap_or(Rom::default(vec![0; 0xFFFF - 0xC000 + 1]));
        bus.register(0xC000..=0xFFFF, Box::new(os_rom));

        cpu.reset(&bus);

        Self {
            cpu,
            bus
        }
    }

    pub fn tick(&mut self) -> bool {
        let ticks = self.cpu.step(&mut self.bus, 1);
        for _ in 0..ticks{
            if !self.bus.tick() {
                return false;
            }
        }
        true
    }
}