use std::{cell::RefCell, rc::Rc};

use crate::{bus::{Device, TickReturn}, devices::bbcmicro::video_system::VideoSystem};

pub struct VideoULA{
    pub video_system: Rc<RefCell<VideoSystem>>,
}

impl Device for VideoULA {
    fn read(&mut self, addr: u16) -> u8 {
        println!("VideoULA {:02X}", addr);
        0
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr{
            0 => {
                match value{
                    0x4B => {
                        self.video_system.borrow_mut().mode = 7;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    #[allow(unused_variables)]
    fn tick(&mut self) -> TickReturn {TickReturn::NONE}
}