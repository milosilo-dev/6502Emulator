use super::cpu::CPU;

impl CPU{
    pub(super) fn set_status(&mut self, v: bool, pos: u8){
        self.status = (self.status & !(1 << pos)) | ((v as u8) << pos);
    }

    pub(super) fn ld_set_status(&mut self, value: u8){
        self.set_status(value == 0, 1); // set z flag
        self.set_status((value & 0b10000000) > 0, 7);
    }

    pub(super) fn adc_set_status(&mut self, is_carry: bool, is_over: bool){
        self.set_status(is_carry, 0); // set c flag
        self.set_status(self.a == 0, 1); // set z flag
        self.set_status(is_over, 6); // set v
        self.set_status((self.a & 0b10000000) > 0, 7); // set n flag
    }

    pub(super) fn and_set_status(&mut self){
        self.set_status(self.a == 0, 1); // set z flag
        self.set_status((self.a & 0b10000000) > 0, 7); // set n flag
    }

    pub(super) fn asl_set_status(&mut self, value: u8, old_bit: bool){
        self.set_status(old_bit, 0); // set c flag
        self.set_status(value == 0, 1); // set z flag
        self.set_status((value & 0b10000000) > 0, 7); // set n flag
    }
    
    pub(super) fn cmp_set_status(&mut self, value: u8){
        let out = self.a.wrapping_sub(value);

        self.set_status(self.a >= value, 0); // set c flag
        self.set_status(value == self.a, 1); // set z flag
        self.set_status((out & 0b10000000) > 0, 7); // set n flag
    }

    pub(super) fn cpx_set_status(&mut self, value: u8) {
        let out = self.x.wrapping_sub(value);

        self.set_status(self.x >= value, 0); // set c flag
        self.set_status(value == self.x, 1); // set z flag
        self.set_status((out & 0b10000000) > 0, 7); // set n flag
    }

    pub(super) fn cpy_set_status(&mut self, value: u8) {
        let out = self.y.wrapping_sub(value);

        self.set_status(self.y >= value, 0); // set c flag
        self.set_status(value == self.y, 1); // set z flag
        self.set_status((out & 0b10000000) > 0, 7); // set n flag
    }

    pub(super) fn dec_set_status(&mut self, value: u8) {
        self.set_status(value == 0, 1); // set z flag
        self.set_status((value & 0b10000000) > 0, 7); // set n flag
    }

    pub(super) fn eor_set_status(&mut self, value: u8) {
        self.set_status(value == 0, 1); // set z flag
        self.set_status((value & 0b10000000) > 0, 7); // set n flag
    }

    pub(super) fn rotate_set_status(&mut self, value: u8, old_msb: bool) {
        self.set_status(old_msb, 0); // set c flag
        self.set_status(value == 0, 1); // set z flag
        self.set_status((value & 0b10000000) > 0, 7); // set n flag
    }

    pub(super) fn sbc_set_status(&mut self, carry_out: bool, is_over: bool){
        self.set_status(carry_out && !is_over, 0); // set c flag
        self.set_status(self.a == 0, 1); // set z flag
        self.set_status(is_over, 6); // set v
        self.set_status((self.a & 0b10000000) > 0, 7); // set n flag
    }
}