use super::cpu::CPU;

use crate::bus::Bus;

impl CPU{
    pub(super) fn set_pc(&mut self, bus: &Bus) {
        let lo = bus.read(0xFFFC) as u16;
        let hi = bus.read(0xFFFD) as u16;
        self.pc = (hi << 8) | lo;
    }

    pub(super) fn fetch_byte(&mut self, bus: &Bus) -> u8{
        let data = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        data
    }

    pub(super) fn read_byte(bus: &Bus, addr: u16) -> u8{
        bus.read(addr)
    }

    pub(super) fn adc(&mut self, value: u8){
        let carry_in = (self.status & 0x01) as u16;

        let a = self.a as u16;
        let v = value as u16;

        let sum = a.wrapping_add(v).wrapping_add(carry_in);

        let result = (sum & 0xFF) as u8;

        let carry_out = sum > 0xFF;

        let overflow =
            (!(self.a ^ value) & (self.a ^ result) & 0x80) != 0;

        self.a = result;
        self.adc_set_status(carry_out, overflow);
    }

    pub(super) fn and(&mut self, value: u8){
        self.a = value & self.a;
        self.and_set_status();
    }

    pub(super) fn asl_acc(&mut self, value: u8){
        let c = self.a & 0b10000000 != 0;
        self.a = value << 1;
        self.asl_set_status(self.a, c);
    }

    pub(super) fn asl_mem(&mut self, bus: &mut Bus, addr: u16, ticks: &mut u32){
        let i = bus.read(addr);
        let c = i & 0b10000000 != 0;
        let v = i << 1;
        *ticks += 3;
        bus.write(addr, v);
        self.asl_set_status(v, c);
    }

    pub(super) fn bit_test(&mut self, value: u8, ticks: &mut u32){
        self.set_status(self.a & value == 0, 1); // Set the zero flag
        self.set_status(value & 0b01000000 != 0, 6);
        self.set_status(value & 0b10000000 != 0, 7);
        *ticks += 1;
    }

    pub(super) fn brk(&mut self, bus: &mut Bus, ticks: &mut u32){
        self.set_status(true, 4);
        self.pc += 1;
        self.push_byte_stack(bus, (self.pc >> 8) as u8);
        self.push_byte_stack(bus, (self.pc & 0xFF) as u8);
        let status = self.status | 0b00110000;
        self.push_byte_stack(bus, status);

        self.pc = u16::from_le_bytes([bus.read(0xFFFE), bus.read(0xFFFF)]);
        *ticks += 7;
    }

    pub(super) fn dec(&mut self, bus: &mut Bus, ticks: &mut u32, addr: u16) {
        let value = bus.read(addr).wrapping_sub(1);
        *ticks += 3;
        bus.write(addr, value);
        self.dec_set_status(value);
    }

    pub(super) fn decx(&mut self, ticks: &mut u32){
        *ticks += 1;
        self.x = self.x.wrapping_sub(1);
        self.dec_set_status(self.x);
    }

    pub(super) fn decy(&mut self, ticks: &mut u32){
        *ticks += 1;
        self.y = self.y.wrapping_sub(1);
        self.dec_set_status(self.y);
    }

    pub(super) fn eor(&mut self, ticks: &mut u32, value: u8){
        *ticks += 1;
        self.a = self.a ^ value;
        self.eor_set_status(self.a);
    }

    pub(super) fn inc(&mut self, bus: &mut Bus, ticks: &mut u32, addr: u16) {
        let value = bus.read(addr).wrapping_add(1);
        *ticks += 3;
        bus.write(addr, value);
        self.dec_set_status(value);
    }

    pub(super) fn incx(&mut self, ticks: &mut u32){
        *ticks += 1;
        self.x = self.x.wrapping_add(1);
        self.dec_set_status(self.x);
    }

    pub(super) fn incy(&mut self, ticks: &mut u32){
        *ticks += 1;
        self.y = self.y.wrapping_add(1);
        self.dec_set_status(self.y);
    }
    
    pub(super) fn jsr(&mut self, bus: &mut Bus, ticks: &mut u32, target: u16){
        *ticks += 2;

        let ret_addr = self.pc - 1;
        self.push_byte_stack(bus, (ret_addr >> 8) as u8);
        self.push_byte_stack(bus, (ret_addr & 0xFF) as u8);

        self.pc = target;
    }

    pub(super) fn ora(&mut self, value: u8, ticks: &mut u32){
        *ticks += 1;
        self.a = self.a | value;
        self.eor_set_status(self.a);
    }

    pub(super) fn pha(&mut self, bus: &mut Bus, ticks: &mut u32){
        self.push_byte_stack(bus, self.a);
        *ticks += 2;
    }

    pub(super) fn php(&mut self, bus: &mut Bus, ticks: &mut u32){
        let status = self.status | 0b00110000;
        self.push_byte_stack(bus, status);
        *ticks += 2;
    }

    pub(super) fn pla(&mut self, bus: &Bus, ticks: &mut u32){
        self.a = self.pull_byte_stack(bus);
        self.ld_set_status(self.a);
        *ticks += 3;
    }

    pub(super) fn plp(&mut self, bus: &Bus, ticks: &mut u32){
        self.status = self.pull_byte_stack(bus) | 0b00100000;
        *ticks += 3;
    }

    pub(super) fn rol_a(&mut self, ticks: &mut u32){
        let msb = self.a & 0b10000000 != 0;
        self.a = (self.a.rotate_left(1) & 0b11111110) | (self.status & 0b00000001);
        *ticks += 1;
        self.rotate_set_status(self.a, msb);
    }

    pub(super) fn rol (&mut self, bus: &mut Bus, addr: u16, ticks: &mut u32){
        let ov = bus.read(addr);
        let msb = ov & 0b10000000 != 0;
        let v = (ov.rotate_left(1) & 0b11111110) | (self.status & 0b00000001);
        bus.write(addr, v);
        *ticks += 3;
        self.rotate_set_status(v, msb);
    }

    pub(super) fn ror_a(&mut self, ticks: &mut u32){
        let lsb = self.a & 0b00000001 != 0;
        let carry_in = (self.status & 0b00000001) != 0;
        self.a = (self.a >> 1) | ((carry_in as u8) << 7);
        *ticks += 1;
        self.rotate_set_status(self.a, lsb);
    }

    pub(super) fn ror(&mut self, bus: &mut Bus, addr: u16, ticks: &mut u32){
        let ov = bus.read(addr);
        let lsb = ov & 0b00000001 != 0;
        let carry_in = (self.status & 0b00000001) != 0;
        let v = (ov >> 1) | ((carry_in as u8) << 7);
        bus.write(addr, v);
        *ticks += 3;
        self.rotate_set_status(v, lsb);
    }

    pub(super) fn rti(&mut self, bus: &mut Bus, ticks: &mut u32) {
        *ticks += 5;
        self.status = self.pull_byte_stack(bus);
        self.set_status(false, 4);
        let pc_lsb = self.pull_byte_stack(bus) as u16;
        let pc_msb = self.pull_byte_stack(bus) as u16;
        self.pc = pc_msb << 8 | pc_lsb;
    }

    pub(super) fn rts(&mut self, bus: &mut Bus, ticks: &mut u32) {
        *ticks += 5;
        let pc_lsb = self.pull_byte_stack(bus) as u16;
        let pc_msb = self.pull_byte_stack(bus) as u16;
        self.pc = (pc_msb << 8 | pc_lsb).wrapping_add(1);
    }

    pub(super) fn sbc(&mut self, value: u8) {
        let carry_in =  (1-(self.status & 0x01)) as u16;

        let a = self.a as u16;
        let v = value as u16;

        let sum = a.wrapping_sub(v).wrapping_sub(carry_in);

        let result = (sum & 0xFF) as u8;

        let carry_out = sum < 0b01111111;

        let overflow =
            ((self.a ^ value) & (self.a ^ result) & 0x80) != 0;

        self.a = result;
        self.sbc_set_status(carry_out, overflow);
    }
}