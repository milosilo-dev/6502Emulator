use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::{Device, TickReturn},
    platform::keyboard::Keyboard,
};

const ROW_COUNT: usize = 8;

// interrupt bits
const CA2_BIT: u8 = 0b0000_0010;
const IRQ_BIT: u8 = 0b1000_0000;

pub struct SystemVIA {
    keyboard: Rc<RefCell<Keyboard>>,

    port_b_direction: u8,
    port_a_direction: u8,
    port_b: u8,
    port_a: u8,

    interrupt_enable: u8,
    interrupt_flag: u8,

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
            interrupt_enable: 0,
            interrupt_flag: 0,
            last_matrix: [0; ROW_COUNT],
        }
    }

    fn irq_active(&self) -> bool {
        (self.interrupt_flag & self.interrupt_enable) != 0
    }

    fn raise_ca2(&mut self) {
        self.interrupt_flag |= CA2_BIT;
    }
}

impl Device for SystemVIA {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // Port B
            0 => {
                let inputs = 0xF0;
                (self.port_b & self.port_b_direction)
                    | (inputs & !self.port_b_direction)
            }

            // Port A
            1 => {
                let row = self.port_b & 0x0F;
                let input = self.keyboard.borrow().get_row(row).unwrap_or(0xF0);
                println!("Key board input requested: {}", input);

                (self.port_a & self.port_a_direction)
                    | (input & !self.port_a_direction)
            }

            // DDRB
            2 => self.port_b_direction,

            // DDRA
            3 => self.port_a_direction,

            // IFR
            0xD => {
                let mut value = self.interrupt_flag;
                if self.irq_active() {
                    value |= IRQ_BIT;
                }
                value
            }

            // IER
            0xE => self.interrupt_enable | IRQ_BIT,

            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // Port B
            0 => self.port_b = value,

            // Port A
            1 => self.port_a = value,

            // DDRB
            2 => self.port_b_direction = value,

            // DDRA
            3 => self.port_a_direction = value,

            // IFR (writing clears bits)
            0xD => {
                self.interrupt_flag &= !value;
            }

            // IER
            0xE => {
                if value & IRQ_BIT != 0 {
                    self.interrupt_enable |= value & !IRQ_BIT;
                } else {
                    self.interrupt_enable &= !(value & !IRQ_BIT);
                }
            }

            _ => {}
        }
    }

    fn tick(&mut self) -> TickReturn {
        for row in 0..ROW_COUNT {
            let current = self.keyboard.borrow().get_row(row as u8).unwrap_or(0);

            if current != self.last_matrix[row] {
                self.last_matrix[row] = current;
                println!("Key change detected row {} {:02X}->{:02X}", row, self.last_matrix[row], current);
                self.raise_ca2();
                break;
            }
        }

        if self.irq_active() {
            TickReturn::IRQ
        } else {
            TickReturn::NONE
        }
    }
}