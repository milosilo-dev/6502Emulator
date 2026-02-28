use crate::{bus::Bus, cpu::config::CpuConfig};

pub struct CPU {
    pub(super) pc: u16,
    pub(super) sp: u8,

    pub(super) a: u8,
    pub(super) x: u8,
    pub(super) y: u8,

    pub(super) status: u8,

    pub config: CpuConfig,
}

impl CPU {
    pub fn default() -> Self {
        Self {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            status: 0,
            config: CpuConfig::default(),
        }
    }

    pub fn reset(&mut self, bus: &Bus) {
        self.a = 0;
        self.x = 0;
        self.y = 0;

        self.sp = 0xFD;
        self.status = 0x24;
        self.set_pc(bus);
    }

    pub fn read_acc(&self) -> u8 {
        self.a
    }

    pub fn read_x(&self) -> u8 {
        self.x
    }

    pub fn read_y(&self) -> u8 {
        self.y
    }

    pub fn read_sp(&self) -> u8 {
        self.sp
    }

    pub fn read_status(&self) -> u8 {
        self.status
    }
}
