use std::{cell::RefCell, rc::Rc};

use crate::{bus::Device, platform::keyboard::Keyboard};

pub struct SystemVIA {
    keyboard: Rc<RefCell<Keyboard>>,

    ora: u8,
    orb: u8,

    ddra: u8,
    ddrb: u8,

    // Interrupt flag register and enable register (basic)
    ifr: u8,
    ier: u8,
}

impl SystemVIA {
    pub fn default(keyboard: Rc<RefCell<Keyboard>>) -> Self {
        Self {
            keyboard,
            ora: 0,
            orb: 0,

            // BBC Micro boots with Port A as output (keyboard row/col select),
            // Port B bit 7 as input (key detect), rest output.
            ddra: 0xFF,
            ddrb: 0x7F,

            ifr: 0,
            ier: 0,
        }
    }

    /// Scan the keyboard matrix using the current ORA value.
    ///
    /// ORA layout (written by the OS before reading ORB):
    ///   bits 0-2 : column select (0-9, only 3 bits used by VIA but keyboard decodes 0-9)
    ///   bits 3-6 : row select    (0-7)
    ///   bit  7   : must be 0 to enable keyboard scan; 1 disables it
    ///
    /// Returns true if the key at (row, col) is pressed.
    fn key_pressed(&self) -> bool {
        // Bit 7 of ORA must be low for keyboard scanning to be active.
        if self.ora & 0x80 != 0 {
            return false;
        }

        let col = (self.ora & 0x07) as u8;       // bits 0-2
        let row = ((self.ora >> 3) & 0x0F) as u8; // bits 3-6

        self.keyboard.borrow().get_key(row, col)
    }
}

// MMIO Device to be mapped from 0xFE40 - 0xFE4F
impl Device for SystemVIA {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            // ORB — Port B data register
            // Bit 7 is the key-detect input: 1 = no key, 0 = key pressed.
            // All other bits are driven by ORB/DDRB as outputs.
            0 => {
                let key_bit = if self.key_pressed() { 0x00 } else { 0x80 };
                // Output bits come from ORB (masked by DDRB); input bit 7 from keyboard.
                let output_bits = self.orb & self.ddrb & 0x7F;
                let input_bits  = key_bit & !self.ddrb;
                output_bits | input_bits
            }

            // ORA — Port A data register (row/col select written by OS)
            1 => self.ora,

            // DDRB / DDRA
            2 => self.ddrb,
            3 => self.ddra,

            // IFR / IER (minimal — enough for OS to check/clear keyboard IRQ)
            13 => self.ifr,
            14 => self.ier | 0x80, // bit 7 always reads as 1 per 6522 spec

            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // ORB write — only output pins (DDRB=1) are affected
            0 => self.orb = (self.orb & !self.ddrb) | (value & self.ddrb),

            // ORA write — selects the keyboard row/column for next ORB read
            1 => self.ora = value,

            // DDRB / DDRA
            2 => self.ddrb = value,
            3 => self.ddra = value,

            // IFR — writing 1 to a bit clears that interrupt flag
            13 => self.ifr &= !value,

            // IER — bit 7 determines set (1) or clear (0) for the masked bits
            14 => {
                if value & 0x80 != 0 {
                    self.ier |= value & 0x7F;
                } else {
                    self.ier &= !(value & 0x7F);
                }
            }

            _ => {}
        }
    }

    fn tick(&mut self) -> bool {
        // Raise CA1 keyboard interrupt (IFR bit 1) if any key is pressed
        // and the corresponding interrupt is enabled.
        if self.key_pressed() {
            self.ifr |= 0x02; // CA1 interrupt flag
        }
        true
    }
}