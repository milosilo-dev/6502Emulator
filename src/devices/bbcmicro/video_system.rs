use std::{cell::RefCell, rc::Rc};

use crate::{bus::Device, devices::mem::Mem, platform::framebuffer::Fb};

const PALETTE: [u32; 16] = [
    0x000000, // black
    0xFF0000, // red
    0x00FF00, // green
    0xFFFF00, // yellow
    0x0000FF, // blue
    0xFF00FF, // magenta
    0x00FFFF, // cyan
    0xFFFFFF, // white
    0x555555,
    0xAA0000,
    0x00AA00,
    0xAAAA00,
    0x0000AA,
    0xAA00AA,
    0x00AAAA,
    0xAAAAAA,
];

pub struct VideoSystem {
    framebuffer: Box<Fb>,
    mem: Rc<RefCell<Mem>>,

    base_adress: u16,
    mode: u8,
    row: u8,
}

impl VideoSystem {
    pub fn default(fb : Box<Fb>, ram: Rc<RefCell<Mem>>) -> Self {
        Self {
            framebuffer: fb,
            mem: ram,
            base_adress: 0,
            mode: 0,
            row: 0,
        }
    }

    pub fn render_scanline(&mut self) -> bool {
        match self.mode{
            0x02 => {
                // Mode 2, 160 x 120 @ 4bbp (16 colours)
                let mem = self.mem.borrow();
                let fb = &mut self.framebuffer;

                if self.row > 120{
                    self.row = 0;
                    return fb.update();
                }
                
                let bytes_per_row = 160 / 2;
                let row_base = self.base_adress as usize + (self.row as usize * bytes_per_row);

                for x in 0..bytes_per_row {
                    let byte = mem.read((row_base + x) as u16);
                    let pixel1 = (byte >> 4) & 0x0F;
                    let pixel2 = byte & 0x0F;

                    fb.set_pixel(x*2, self.row as usize, PALETTE[pixel1 as usize]);
                    fb.set_pixel(x*2 + 1, self.row as usize, PALETTE[pixel2 as usize]);
                }

                self.row += 1;
            }
            0x07 => {
                // Mode 7, Tele-type mode
                let mem = self.mem.borrow();
                let fb = &mut self.framebuffer;

                if self.row >= 25{
                    self.row = 0;
                    return fb.update();
                }

                for x in 0..40 {
                    let byte = mem.read(self.base_adress + (self.row * 40) as u16 + x);
                    let stripped = byte & 0b01111111;
                    let chr = &[stripped];
                    let s = std::str::from_utf8(chr).unwrap_or(""); 
                    fb.draw_text((x * 6) as usize, (self.row * 10) as usize, s);
                }

                self.row += 1;
            }
            _ => {}
        }
        return true;
    }
}

impl Device for VideoSystem {
    #[allow(unused_variables)]
    fn read(&self, addr: u16) -> u8 {0}

    fn write(&mut self, addr: u16, value: u8) {
        match addr{
            0x00 => {
                // Lowwer byte of vm adress
                self.base_adress = (self.base_adress & 0xFF00) + value as u16;
            }
            0x01 => {
                // Upper byte of vm adress
                self.base_adress = (self.base_adress & 0x00FF) | ((value as u16) << 8);
            }
            0x20 => {
                self.mode = value;
                self.row = 0;
            }
            _ => {}
        }
    }

    fn tick(&mut self) -> bool {
        self.render_scanline()
    }
}