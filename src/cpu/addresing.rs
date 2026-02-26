use super::cpu::CPU;

use crate::bus::Bus;

impl CPU{
    pub(super) fn immediate_adressing(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        *ticks += 1;
        self.fetch_byte(bus)
    }

    pub(super) fn accumulator_adressing(&mut self, ticks: &mut u32) -> u8{
        *ticks += 1;
        self.a
    }

    pub(super) fn get_zp_adress(&mut self, bus: &Bus, ticks: &mut u32) -> u16{
        *ticks += 1;
        self.fetch_byte(bus) as u16
    }

    pub(super) fn zero_page_adressing(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let addr = self.get_zp_adress(bus, ticks);
        *ticks += 1;
        Self::read_byte(bus, addr as u16)
    }

    pub(super) fn get_zp_adress_x(&mut self, bus: &Bus, ticks: &mut u32) -> u16{
        *ticks += 2;
        self.fetch_byte(bus).wrapping_add(self.x) as u16
    }

    pub(super) fn zero_page_adressing_x(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let addr = self.get_zp_adress_x(bus, ticks);
        *ticks += 1;
        Self::read_byte(bus, addr as u16)
    }

    pub(super) fn zero_page_adressing_y(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let addr = self.fetch_byte(bus).wrapping_add(self.y);
        *ticks += 3;
        Self::read_byte(bus, addr as u16)
    }

    pub(super) fn relative_adressing(&mut self, bus: &Bus, ticks: &mut u32){
        let old_pc = self.pc;

        let offset = i8::from_ne_bytes([self.immediate_adressing(bus, ticks)]);
        self.pc = self.pc.wrapping_add_signed(offset.into());

        if (old_pc & 0xFF00) != (self.pc & 0xFF00) {
            *ticks += 1;
        }
    }

    pub(super) fn get_absolute_adress(&mut self, bus: &Bus, ticks: &mut u32) -> u16 {
        let lo = self.fetch_byte(bus) as u16;
        let hi = self.fetch_byte(bus) as u16;
        *ticks += 3;
        (hi << 8) | lo
    }

    pub(super) fn absolute_adressing(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        Self::read_byte(bus, self.get_absolute_adress(bus, ticks) as u16)
    }

    pub(super) fn get_absolute_adress_x(&mut self, bus: &Bus, ticks: &mut u32) -> u16{
        self.get_absolute_adress(bus, ticks).wrapping_add(self.x as u16)
    }

    pub(super) fn absolute_adressing_x(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let base = self.get_absolute_adress(bus, ticks);
        let addr = base.wrapping_add(self.x as u16);

        if (base & 0xFF00) != (addr & 0xFF00) {
            *ticks += 1;
        }

        Self::read_byte(bus, addr as u16)
    }

    pub(super) fn absolute_adressing_y(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let base = self.get_absolute_adress(bus, ticks);
        let addr = base.wrapping_add(self.y as u16);

        if (base & 0xFF00) != (addr & 0xFF00) {
            *ticks += 1;
        }

        Self::read_byte(bus, addr as u16)
    }

    pub(super) fn get_indirect_adress(&mut self, bus: &Bus, ticks: &mut u32) -> u16 {
        let lo = self.fetch_byte(bus);
        let hi = self.fetch_byte(bus);
        let in_addr = (hi as u16) << 8 | lo as u16;

        *ticks += 5;
        let addr_lo = Self::read_byte(bus, in_addr);
        let addr_hi = Self::read_byte(bus, in_addr + 1);
        ((addr_hi as u16) << 8) | (addr_lo as u16)
    }

    pub(super) fn indirect_indexing_adressing_x(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let zp_addr = self.fetch_byte(bus).wrapping_add(self.x);
        let lsb_addr = Self::read_byte(bus, zp_addr as u16);
        let msb_addr = Self::read_byte(bus, zp_addr as u16 + 1);
        let addr = ((msb_addr as u16) << 8) | (lsb_addr as u16);

        *ticks += 6;
        Self::read_byte(bus, addr)
    }

    pub(super) fn indexing_indirect_adressing_y(&mut self, bus: &Bus, ticks: &mut u32) -> u8 {
        let zp = self.fetch_byte(bus);

        let lo = Self::read_byte(bus, zp as u16) as u16;
        let hi = Self::read_byte(bus, zp.wrapping_add(1) as u16) as u16; 

        let base = (hi << 8) | lo;
        let addr = base.wrapping_add(self.y as u16);

        if (base & 0xFF00) != (addr & 0xFF00) {
            *ticks += 1;
        }

        *ticks += 5;
        Self::read_byte(bus, addr as u16)
    }
}