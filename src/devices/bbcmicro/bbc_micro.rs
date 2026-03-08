use std::{cell::RefCell, rc::Rc, thread, time::{Duration, SystemTime}};

use crate::{bus::{Bus, TickReturn}, cpu::cpu::CPU, devices::{bbcmicro::{paged_rom::{PagedRom, ROMSelectRegister}, system_via::SystemVIA, video_system::VideoSystem, video_ula::VideoULA}, mem::Mem, rom::Rom}, platform::{framebuffer::Fb, keyboard::Keyboard, logging::{NoLog, Stdout}}};

pub struct BBCMicro {
    cpu: CPU,
    bus: Bus,
}

impl BBCMicro {
    pub fn new() -> Self{
        let mut cpu = CPU::default();
        cpu.config.logger = Box::new(NoLog{});
        cpu.config.speed = 10.0;
        let mut bus = Bus::default();

        let ram = Rc::new(RefCell::new(Mem::default(32 * 1024)));
        bus.register(0..=0x7FFF, Box::new(ram.clone()));

        let paged_rom = Rc::new(RefCell::new(PagedRom::default()));
        let basic = Rom::load("roms/bbc_micro/BASIC2.rom").unwrap_or(Rom::default(vec![0; 0xBFFF - 0x8000 + 1]));
        paged_rom.borrow_mut().add_rom(basic);
        bus.register(0x8000..=0xBFFF, Box::new(paged_rom.clone()));

        let keyboard = Rc::new(RefCell::new(Keyboard::default()));
        let fb = Box::new(Fb::default(keyboard.clone()));
        let video_system= Rc::new(RefCell::new(VideoSystem::default(fb, Rc::clone(&ram))));
        bus.register(0xFE00..=0xFE07, Box::new(video_system.clone()));
        
        let video_ula = VideoULA{video_system: video_system};
        bus.register(0xFE20..=0xFE2F, Box::new(video_ula));

        let system_via = SystemVIA::default(Rc::clone(&keyboard));
        bus.register(0xFE40..=0xFE4F, Box::new(system_via));

        let page_rom_select = ROMSelectRegister::default(paged_rom);
        bus.register(0xFE30..=0xFE30, Box::new(page_rom_select));

        let os_rom = Rom::load("roms/bbc_micro/OS-1.2.rom").unwrap_or(Rom::default(vec![0; 0xFFFF - 0xC000 + 1]));
        bus.register(0xC000..=0xFFFF, Box::new(os_rom));

        cpu.reset(&mut bus);

        Self {
            cpu,
            bus
        }
    }

    pub fn tick(&mut self) -> bool {
        let mut ticks = self.cpu.step(&mut self.bus, 1);

        let now = SystemTime::now();
        for _ in 0..ticks{
            match self.bus.tick() {
                TickReturn::SHUTDOWN => {
                    return false;
                }
                TickReturn::IRQ => {
                    println!("IRQ Pin pulled");
                    self.cpu.brk(&mut self.bus, &mut ticks);
                }
                TickReturn::NONE =>{}
            }
        }

        let elapsed = now.elapsed().unwrap_or(Duration::from_micros(0));
        let time = Duration::from_micros(((1.0 / self.cpu.config.speed) as u32 * ticks) as u64);
        let sleep_time: Duration = if time > elapsed {
            time - elapsed
        } else {
            Duration::from_micros(0)
        };
        thread::sleep(sleep_time);
        true
    }
}