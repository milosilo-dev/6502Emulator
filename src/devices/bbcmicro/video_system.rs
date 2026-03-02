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
    cur_sl: u8,
}

impl VideoSystem {
    pub fn default(fb : Box<Fb>, ram: Rc<RefCell<Mem>>) -> Self {
        Self {
            framebuffer: fb,
            mem: ram,
            base_adress: 0,
            mode: 0,
            cur_sl: 0,
        }
    }

    pub fn render_scanline(&mut self) {
        match self.mode{
            0x02 => {
                // Mode 2, 160 x 120 @ 4bbp (16 colours)
                let mem = self.mem.borrow();
                let fb = &mut self.framebuffer;
                
                let bytes_per_row = 160 / 2; // 160 pixels, 2 pixels per byte
                let row_base = self.base_adress as usize + (self.cur_sl as usize * bytes_per_row);

                for x in 0..bytes_per_row {
                    let byte = mem.read((row_base + x) as u16);
                    let pixel1 = (byte >> 4) & 0x0F;
                    let pixel2 = byte & 0x0F;

                    fb.set_pixel(x*2, self.cur_sl as usize, PALETTE[pixel1 as usize]);
                    fb.set_pixel(x*2 + 1, self.cur_sl as usize, PALETTE[pixel2 as usize]);
                }

                self.cur_sl += 1;
                if self.cur_sl >= 120{
                    fb.update();
                    self.cur_sl = 0;
                }
            }
            _ => {}
        }
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
            }
            _ => {}
        }
    }

    fn tick(&mut self) {
        self.render_scanline();
    }
}