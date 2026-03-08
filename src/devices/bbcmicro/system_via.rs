use std::{cell::RefCell, rc::Rc};

use crate::{bus::{Device, TickReturn}, cpu::cpu::CPU, platform::keyboard::Keyboard};

const ROW_COUNT: usize = 10;

// interupt_flag / interupt_enable bit masks
const CA2_BIT: u8 = 0b0000_0010; // bit 1 – keyboard CA2
const IRQ_BIT: u8 = 0b1000_0000; // bit 7 – any active IRQ

pub struct SystemVIA {
    keyboard: Rc<RefCell<Keyboard>>,

    port_b_direction: u8,
    port_a_direction: u8,
    port_b: u8,
    port_a: u8,

    interupt_enable: u8,
    interupt_flag: u8,

    last_matrix: [u8; ROW_COUNT],
}

impl SystemVIA {
    pub fn default(keyboard: Rc<RefCell<Keyboard>>) -> Self {
        Self {
            keyboard,
            port_b_direction: 0,
            port_a_direction: 0,
            port_b: 0,
            port_a: 0,
            interupt_enable: 0,
            interupt_flag: 0,
            last_matrix: [0u8; ROW_COUNT],
        }
    }

    /// Called by the CPU/bus to check whether this device is asserting IRQ.
    pub fn irq(&self) -> bool {
        self.interupt_flag & IRQ_BIT != 0
    }

    fn raise_ca2(&mut self) {
        // Only raise if CA2 is enabled in interupt_enable
        if self.interupt_enable & CA2_BIT != 0 {
            self.interupt_flag |= CA2_BIT | IRQ_BIT;
        }
    }

    fn clear_ca2(&mut self) {
        self.interupt_flag &= !(CA2_BIT);
        // Clear master IRQ bit if no other flags remain
        if self.interupt_flag & !IRQ_BIT == 0 {
            self.interupt_flag &= !IRQ_BIT;
        }
    }
}

impl Device for SystemVIA {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0 => {
                let inputs = 0xF0;
                (self.port_b & self.port_b_direction) | (inputs & !self.port_b_direction)
            }
            1 => {
                let row = self.port_b & 0x0F;
                let input = self.keyboard.borrow().get_row(row).unwrap_or(0xF0);
                self.clear_ca2();
                (self.port_a & self.port_a_direction) | (input & !self.port_a_direction)
            }
            2 => self.port_b_direction,
            3 => self.port_a_direction,
            0xD => self.interupt_flag,
            0xE => self.interupt_enable,
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0 => self.port_b = value,
            1 => self.port_a = value,
            2 => self.port_b_direction = value,
            3 => self.port_a_direction = value,
            0xD => {
                // Writing to interupt_flag clears the specified flag bits
                self.interupt_flag &= !value;
                if self.interupt_flag & !IRQ_BIT == 0 {
                    self.interupt_flag &= !IRQ_BIT;
                }
            }
            0xE => {
                // Bit 7 high = set the specified bits; bit 7 low = clear them
                if value & IRQ_BIT != 0 {
                    self.interupt_enable |= value & !IRQ_BIT;
                } else {
                    self.interupt_enable &= !value;
                }
            }
            _ => {}
        }
    }

    fn tick(&mut self) -> TickReturn {
        // Scan every row and raise CA2 if any key state has changed
        for row in 0..ROW_COUNT {
            let current = self.keyboard.borrow().get_row(row as u8).unwrap_or(0);
            if current != self.last_matrix[row] {
                self.last_matrix[row] = current;
                self.raise_ca2();
                break; // one IRQ per tick is enough; MOS will re-scan
            }
        }
        if self.irq(){
            TickReturn::IRQ
        } else {
            TickReturn::NONE
        }
    }
}