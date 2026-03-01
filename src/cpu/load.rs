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

    // Run a test rom until a break address is hit
    // 1 -> fail, 0 -> sucsess
    pub fn run(&mut self, bus: &mut Bus, break_address: u16, max_cycles: Option<u64>) -> u8 {
        if max_cycles.is_some(){
            for _ in 0..max_cycles.unwrap() {
                let ticks = self.step(bus, 1);
                self.config.logger.log(format!("pc: {:X}", self.pc));
                thread::sleep(Duration::from_millis(((1.0 / self.config.speed) as u32 * ticks) as u64));

                if self.pc == break_address {
                    return 0;
                }
            }
        } else {
            loop {
                let ticks = self.step(bus, 1);
                self.config.logger.log(format!("pc: {:X}", self.pc));
                thread::sleep(Duration::from_millis(((1.0 / self.config.speed) as u32 * ticks) as u64));

                if self.pc == break_address {
                    return 0;
                }
            }
        }
        1
    }
}
