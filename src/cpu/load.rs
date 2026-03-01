use std::fs;
use std::{thread, time::Duration};

use super::cpu::CPU;
use crate::bus::Bus;

impl CPU {
    pub fn load_rom(&mut self, bus: &mut Bus, path: &str, offset: u16) -> bool {
        let contents = fs::read(path).unwrap_or(vec![]);
        if contents.len() == 0 {
            return false
        }

        let mut addr = offset;
        for byte in contents{
            bus.write(addr, byte);
            if addr != 0xFFFF{
                addr+=1;
            }
        }

        return true;
    }

    pub fn run(&mut self, bus: &mut Bus) {
        loop {
            let ticks = self.step(bus, 1);
            self.config.logger.log(format!("pc: {:X}", self.pc));
            thread::sleep(Duration::from_millis(((1.0 / self.config.speed) as u32 * ticks) as u64));
        }
    }
}
