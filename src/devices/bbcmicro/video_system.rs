use std::{cell::RefCell, rc::Rc};

use crate::{bus::{Device, TickReturn}, devices::mem::Mem, platform::framebuffer::Fb};

pub const PALETTE: [u32; 16] = [
    0x000000, // 0 black
    0xFF0000, // 1 red
    0x00FF00, // 2 green
    0xFFFF00, // 3 yellow
    0x0000FF, // 4 blue
    0xFF00FF, // 5 magenta
    0x00FFFF, // 6 cyan
    0xFFFFFF, // 7 white

    // flashing versions (usually same colour for now)
    0x000000, // 8
    0xFF0000, // 9
    0x00FF00, // 10
    0xFFFF00, // 11
    0x0000FF, // 12
    0xFF00FF, // 13
    0x00FFFF, // 14
    0xFFFFFF, // 15
];

pub struct VideoSystem {
    framebuffer: Box<Fb>,
    mem: Rc<RefCell<Mem>>,

    crtc_selected: u8,
    crtc: [u8; 18],

    pub mode: u8,
    ticks_per_frame: u16,
}

impl VideoSystem {
    pub fn default(fb: Box<Fb>, mem: Rc<RefCell<Mem>>) -> Self{
        Self { framebuffer: fb, mem, crtc_selected: 0, crtc: [0; 18], mode: 0, ticks_per_frame: 0 }
    }

    fn screen_base(&self) -> u16 {
        match self.mode{
            7 => {
                0x7C00
            }
            _ => {
                let start = ((self.crtc[12] as u16) << 8) | self.crtc[13] as u16;
                start << 3
            }
        }
    }

    fn render_frame(&mut self) -> bool {
        match self.mode {
            7 => self.render_mode7(),
            2 => self.render_mode2(),
            _ => {}
        };

        self.framebuffer.update()
    }

    fn render_mode7(&mut self) {
        let base = self.screen_base();
        let mut mem = self.mem.borrow_mut();

        for row in 0..25 {
            let mut s = String::new();
            for col in 0..40 {
                let addr = base + (row * 40 + col) as u16;
                let byte = mem.read(addr);

                let c = if byte >= 32 && byte < 127 {
                    byte as char
                } else {
                    ' '
                };

                s = format!("{}{}", s, c);
            }

            self.framebuffer.draw_text(
                0,
                row * 8,
                s.as_str()
            );
        }
    }

    fn render_mode2(&mut self) {
        let base = self.screen_base();
        let mut mem = self.mem.borrow_mut();

        for y in 0..120 {
            let row_base = base + (y * 80) as u16;

            for x in 0..80 {
                let byte = mem.read(row_base + x);

                let p1 = (byte >> 4) & 0x0F;
                let p2 = byte & 0x0F;

                self.framebuffer.set_pixel((x * 2) as usize, y, PALETTE[p1 as usize]);
                self.framebuffer.set_pixel((x * 2 + 1) as usize, y, PALETTE[p2 as usize]);
            }
        }
    }
}

impl Device for Rc<RefCell<VideoSystem>> {
    fn read(&mut self, _addr: u16) -> u8 {
        0
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x00 => {
                self.borrow_mut().crtc_selected = value & 0x1F;
            }

            0x01 => {
                let mut this = self.borrow_mut();
                let crtc_selected = this.crtc_selected;
                if (crtc_selected as usize) < this.crtc.len() {
                    this.crtc[crtc_selected as usize] = value;
                }
            }

            0x20 => {
                // Video ULA control (simplified)
                self.borrow_mut().mode = value & 0x07;
            }

            _ => {}
        } 
    }

    fn tick(&mut self) -> TickReturn {
        let mut this = self.borrow_mut();

        this.ticks_per_frame += 1;
        if this.ticks_per_frame >= 10000 {
            this.ticks_per_frame = 0;
            if this.render_frame() {
                return TickReturn::NONE;
            } else {
                return TickReturn::SHUTDOWN;
            }
        }
        TickReturn::NONE
    }
}

impl Device for VideoSystem {
    fn read(&mut self, _addr: u16) -> u8 {
        0
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x00 => {
                self.crtc_selected = value & 0x1F;
            }

            0x01 => {
                if (self.crtc_selected as usize) < self.crtc.len() {
                    self.crtc[self.crtc_selected as usize] = value;
                }
            }

            0x20 => {
                // Video ULA control (simplified)
                self.mode = value & 0x07;
            }

            _ => {}
        } 
    }

    fn tick(&mut self) -> TickReturn {
        self.ticks_per_frame += 1;
        if self.ticks_per_frame >= 10000 {
            self.ticks_per_frame = 0;
            if self.render_frame() {
                return TickReturn::NONE;
            } else {
                return TickReturn::SHUTDOWN;
            }
        }
        TickReturn::NONE
    }
}