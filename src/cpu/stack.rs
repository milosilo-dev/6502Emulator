use super::cpu::CPU;
use crate::bus::Bus;

impl CPU {
    pub(super) fn pull_byte_stack(&mut self, bus: &Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        bus.read(0x0100 + self.sp as u16)
    }

    pub(super) fn push_byte_stack(&mut self, bus: &mut Bus, value: u8) {
        bus.write(self.sp as u16 + 0x0100, value);
        self.sp = self.sp.wrapping_sub(1);
    }
}
