use crate::bus::Bus;

pub struct CPU {
    pc: u16,
    sp: u8,

    a: u8,
    x: u8,
    y: u8,

    status: u8,
}

impl CPU {
    pub fn default() -> Self{
        Self { pc: 0, sp: 0, a: 0, x: 0, y: 0, status: 0 }
    }

    pub fn reset(&mut self, bus: &Bus ){
        self.a = 0;
        self.x = 0;
        self.y = 0;

        self.sp = 0x00FD;
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

    fn set_pc(&mut self, bus: &Bus) {
        let lo = bus.read(0xFFFC) as u16;
        let hi = bus.read(0xFFFD) as u16;
        self.pc = (hi << 8) | lo;
    }

    fn set_status(&mut self, v: bool, pos: u8){
        self.status = (self.status & !(1 << pos)) | ((v as u8) << pos);
    }

    fn fetch_byte(&mut self, bus: &Bus) -> u8{
        let data = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        data
    }

    fn read_byte(bus: &Bus, addr: u16) -> u8{
        bus.read(addr)
    }

    fn pull_byte_stack(&mut self, bus: &Bus) -> u8{
        self.sp = self.sp.wrapping_add(1);
        bus.read(0x0100 + self.sp as u16)
    }

    fn read_byte_stack(&self, bus: &Bus) -> u8{
        return bus.read(self.sp as u16 + 0x0100);
    }

    fn push_byte_stack(&mut self, bus: &mut Bus, value: u8){
        bus.write(self.sp as u16 + 0x0100, value);
        println!("Pushed byte {:X} to stack pointer {:X}", value, self.sp);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn ld_set_status(&mut self, value: u8){
        self.set_status(value == 0, 1); // set z flag
        self.set_status((value & 0b10000000) > 0, 7);
    }

    fn adc_set_status(&mut self, is_carry: bool, is_over: bool){
        self.set_status(is_carry, 0); // set c flag
        self.set_status(self.a == 0, 1); // set z flag
        self.set_status(is_over, 6); // set v
        self.set_status((self.a & 0b10000000) > 0, 7); // set n flag
    }

    fn and_set_status(&mut self){
        self.set_status(self.a == 0, 1); // set z flag
        self.set_status((self.a & 0b10000000) > 0, 7); // set n flag
    }

    fn asl_set_status(&mut self, value: u8, old_bit: bool){
        self.set_status(old_bit, 0); // set c flag
        self.set_status(value == 0, 1); // set z flag
        self.set_status((value & 0b10000000) > 0, 7); // set n flag
    }
    
    fn cmp_set_status(&mut self, value: u8){
        let out = self.a.wrapping_sub(value);

        self.set_status(self.a >= value, 0); // set c flag
        self.set_status(value == self.a, 1); // set z flag
        self.set_status((out & 0b10000000) > 0, 7); // set n flag
    }

    fn cpx_set_status(&mut self, value: u8) {
        let out = self.x.wrapping_sub(value);

        self.set_status(self.x >= value, 0); // set c flag
        self.set_status(value == self.x, 1); // set z flag
        self.set_status((out & 0b10000000) > 0, 7); // set n flag
    }

    fn cpy_set_status(&mut self, value: u8) {
        let out = self.y.wrapping_sub(value);

        self.set_status(self.y >= value, 0); // set c flag
        self.set_status(value == self.y, 1); // set z flag
        self.set_status((out & 0b10000000) > 0, 7); // set n flag
    }

    fn dec_set_status(&mut self, value: u8) {
        self.set_status(value == 0, 1); // set z flag
        self.set_status((value & 0b10000000) > 0, 7); // set n flag
    }

    fn eor_set_status(&mut self, value: u8) {
        self.set_status(value == 0, 1); // set z flag
        self.set_status((value & 0b10000000) > 0, 7); // set n flag
    }

    fn immediate_adressing(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        *ticks += 1;
        self.fetch_byte(bus)
    }

    fn accumulator_adressing(&mut self, ticks: &mut u32) -> u8{
        *ticks += 1;
        self.a
    }

    fn get_zp_adress(&mut self, bus: &Bus, ticks: &mut u32) -> u16{
        *ticks += 1;
        self.fetch_byte(bus) as u16
    }

    fn zero_page_adressing(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let addr = self.get_zp_adress(bus, ticks);
        *ticks += 1;
        Self::read_byte(bus, addr as u16)
    }

    fn get_zp_adress_x(&mut self, bus: &Bus, ticks: &mut u32) -> u16{
        *ticks += 2;
        self.fetch_byte(bus).wrapping_add(self.x) as u16
    }

    fn zero_page_adressing_x(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let addr = self.get_zp_adress_x(bus, ticks);
        *ticks += 1;
        Self::read_byte(bus, addr as u16)
    }

    fn zero_page_adressing_y(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let addr = self.fetch_byte(bus).wrapping_add(self.y);
        *ticks += 3;
        Self::read_byte(bus, addr as u16)
    }

    fn relative_adressing(&mut self, bus: &Bus, ticks: &mut u32){
        let page = (self.pc >> 4) & 0x0F;
        let value = i8::from_ne_bytes([self.immediate_adressing(bus, ticks)]);
        self.pc = self.pc.wrapping_add_signed(value.into());

        if page != (self.pc >> 4) & 0x0F{ // Crossed a page boundry
            *ticks += 1;
        }
    }

    fn get_absolute_adress(&mut self, bus: &Bus, ticks: &mut u32) -> u16 {
        let lo = self.fetch_byte(bus) as u16;
        let hi = self.fetch_byte(bus) as u16;
        *ticks += 3;
        (hi << 8) | lo
    }

    fn absolute_adressing(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        Self::read_byte(bus, self.get_absolute_adress(bus, ticks) as u16)
    }

    fn get_absolute_adress_x(&mut self, bus: &Bus, ticks: &mut u32) -> u16{
        self.get_absolute_adress(bus, ticks).wrapping_add(self.x as u16)
    }

    fn absolute_adressing_x(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let base = self.get_absolute_adress(bus, ticks);
        let addr = base.wrapping_add(self.x as u16);

        if (base & 0xFF00) != (addr & 0xFF00) {
            *ticks += 1;
        }

        Self::read_byte(bus, addr as u16)
    }

    fn absolute_adressing_y(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let base = self.get_absolute_adress(bus, ticks);
        let addr = base.wrapping_add(self.y as u16);

        if (base & 0xFF00) != (addr & 0xFF00) {
            *ticks += 1;
        }

        Self::read_byte(bus, addr as u16)
    }

    fn get_indirect_adress(&mut self, bus: &Bus, ticks: &mut u32) -> u16 {
        let lo = self.fetch_byte(bus);
        let hi = self.fetch_byte(bus);
        let in_addr = (hi as u16) << 8 | lo as u16;

        *ticks += 5;
        let addr_lo = Self::read_byte(bus, in_addr);
        let addr_hi = Self::read_byte(bus, in_addr + 1);
        ((addr_hi as u16) << 8) | (addr_lo as u16)
    }

    fn indirect_indexing_adressing_x(&mut self, bus: &Bus, ticks: &mut u32) -> u8{
        let zp_addr = self.fetch_byte(bus).wrapping_add(self.x);
        let lsb_addr = Self::read_byte(bus, zp_addr as u16);
        let msb_addr = Self::read_byte(bus, zp_addr as u16 + 1);
        let addr = ((msb_addr as u16) << 8) | (lsb_addr as u16);

        *ticks += 6;
        Self::read_byte(bus, addr)
    }

    fn indexing_indirect_adressing_y(&mut self, bus: &Bus, ticks: &mut u32) -> u8 {
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

    fn adc(&mut self, value: u8){
        let carry_in = (self.status & 0x01) as u16;

        let a = self.a as u16;
        let v = value as u16;

        let sum = a + v + carry_in;

        let result = (sum & 0xFF) as u8;

        let carry_out = sum > 0xFF;

        let overflow =
            (!(self.a ^ value) & (self.a ^ result) & 0x80) != 0;

        self.a = result;
        self.adc_set_status(carry_out, overflow);
    }

    fn and(&mut self, value: u8){
        self.a = value & self.a;
        self.and_set_status();
    }

    fn asl_acc(&mut self, value: u8){
        let c = self.a & 0b10000000 != 0;
        self.a = value << 1;
        self.asl_set_status(self.a, c);
    }

    fn asl_mem(&mut self, bus: &mut Bus, addr: u16, ticks: &mut u32){
        let i = bus.read(addr);
        let c = i & 0b10000000 != 0;
        let v = i << 1;
        *ticks += 3;
        bus.write(addr, v);
        self.asl_set_status(v, c);
    }

    fn bit_test(&mut self, value: u8, ticks: &mut u32){
        self.set_status(self.a & value == 0, 1); // Set the zero flag
        self.set_status(value & 0b01000000 != 0, 6);
        self.set_status(value & 0b10000000 != 0, 7);
        *ticks += 1;
    }

    fn brk(&mut self, bus: &mut Bus, ticks: &mut u32){
        self.set_status(true, 4);
        self.pc += 1;
        self.push_byte_stack(bus, (self.pc >> 8) as u8);
        self.push_byte_stack(bus, (self.pc & 0xFF) as u8);
        self.push_byte_stack(bus, self.status);

        self.pc = u16::from_le_bytes([bus.read(0xFFFE), bus.read(0xFFFF)]);
        *ticks += 7;
    }

    fn dec(&mut self, bus: &mut Bus, ticks: &mut u32, addr: u16) {
        let value = bus.read(addr).wrapping_sub(1);
        *ticks += 3;
        bus.write(addr, value);
        self.dec_set_status(value);
    }

    fn decx(&mut self, ticks: &mut u32){
        *ticks += 1;
        self.x = self.x.wrapping_sub(1);
        self.dec_set_status(self.x);
    }

    fn decy(&mut self, ticks: &mut u32){
        *ticks += 1;
        self.y = self.y.wrapping_sub(1);
        self.dec_set_status(self.y);
    }

    fn eor(&mut self, ticks: &mut u32, value: u8){
        *ticks += 1;
        self.a = self.a ^ value;
        self.eor_set_status(self.a);
    }

    fn inc(&mut self, bus: &mut Bus, ticks: &mut u32, addr: u16) {
        let value = bus.read(addr).wrapping_add(1);
        *ticks += 3;
        bus.write(addr, value);
        self.dec_set_status(value);
    }

    fn incx(&mut self, ticks: &mut u32){
        *ticks += 1;
        self.x = self.x.wrapping_add(1);
        self.dec_set_status(self.x);
    }

    fn incy(&mut self, ticks: &mut u32){
        *ticks += 1;
        self.y = self.y.wrapping_add(1);
        self.dec_set_status(self.y);
    }
    
    fn jsr(&mut self, bus: &mut Bus, ticks: &mut u32, target: u16){
        *ticks += 2;

        let ret_addr = self.pc - 1;
        self.push_byte_stack(bus, (ret_addr >> 8) as u8);
        self.push_byte_stack(bus, (ret_addr & 0xFF) as u8);

        self.pc = target;
    }

    pub fn execute(&mut self, bus: &mut Bus, min_ticks: u32){
        let mut ticks  = 0;
        while min_ticks > ticks{
            let ins = self.fetch_byte(bus);
            ticks += 1;
            match ins{
                0xA9 => {
                    // LDA_IM
                    self.a = self.immediate_adressing(bus, &mut ticks);
                    self.ld_set_status(self.a);
                    println!("Loaded {:X} into the A register!", self.a);
                }
                0xA5 => {
                    // LDA_ZP
                    self.a = self.zero_page_adressing(bus, &mut ticks);
                    self.ld_set_status(self.a);
                    println!("Loaded {:X} into the A register!", self.a);
                }
                0xB5 => {
                    // LDA_ZP_X
                    self.a = self.zero_page_adressing_x(bus, &mut ticks);
                    self.ld_set_status(self.a);
                    println!("Loaded {:X} into the A register!", self.a);
                }
                0xAD => {
                    // LDA_ABSOLUTE
                    self.a = self.absolute_adressing(bus, &mut ticks);
                    self.ld_set_status(self.a);
                    println!("Loaded {:X} into the A register!", self.a);
                }
                0xBD => {
                    // LDA_ABSOLUTE_X
                    self.a = self.absolute_adressing_x(bus, &mut ticks);
                    self.ld_set_status(self.a);
                    println!("Loaded {:X} into the A register!", self.a);
                }
                0xB9 => {
                    // LDA_ABSOLUTE_Y
                    self.a = self.absolute_adressing_y(bus, &mut ticks);
                    self.ld_set_status(self.a);
                    println!("Loaded {:X} into the A register!", self.a);
                }
                0xA1 => {
                    // LDA_INDIRECT_INDEXING
                    self.a = self.indirect_indexing_adressing_x(bus, &mut ticks);
                    self.ld_set_status(self.a);
                    println!("Loaded {:X} into the A register!", self.a);
                }
                0xB1 => {
                    // LDA_INDEXING_INDIRECT
                    self.a = self.indexing_indirect_adressing_y(bus, &mut ticks);
                    self.ld_set_status(self.a);
                    println!("Loaded {:X} into the A register!", self.a);
                }
                0xA2 => {
                    // IMMEDIATE_LDX
                    self.x = self.immediate_adressing(bus, &mut ticks);
                    self.ld_set_status(self.x);
                    println!("Loaded {:X} into the X register!", self.x);
                }
                0xA6 => {
                    // LDX_ZP
                    self.x = self.zero_page_adressing(bus, &mut ticks);
                    self.ld_set_status(self.x);
                    println!("Loaded {:X} into the X register!", self.x);
                }
                0xB6 => {
                    // LDX_ZP_Y
                    self.x = self.zero_page_adressing_y(bus, &mut ticks);
                    self.ld_set_status(self.x);
                    println!("Loaded {:X} into the X register!", self.x);
                }
                0xAE => {
                    // LDX_ABSOLUTE
                    self.x = self.absolute_adressing(bus, &mut ticks);
                    self.ld_set_status(self.x);
                    println!("Loaded {:X} into the X register!", self.x);
                }
                0xBE => {
                    // LDX_ABSOLUTE_Y
                    self.x = self.absolute_adressing_y(bus, &mut ticks);
                    self.ld_set_status(self.x);
                    println!("Loaded {:X} into the X register!", self.x);
                }
                0xA0 => {
                    // IMMEDIATE_LDY
                    self.y = self.immediate_adressing(bus, &mut ticks);
                    self.ld_set_status(self.y);
                    println!("Loaded {:X} into the Y register!", self.y);
                }
                0xA4 => {
                    // LDY_ZP
                    self.y = self.zero_page_adressing(bus, &mut ticks);
                    self.ld_set_status(self.y);
                    println!("Loaded {:X} into the Y register!", self.y);
                }
                0xB4 => {
                    // LDY_ZP_X
                    self.y = self.zero_page_adressing_x(bus, &mut ticks);
                    self.ld_set_status(self.y);
                    println!("Loaded {:X} into the Y register!", self.y);
                }
                0xAC => {
                    // LDY_ABSOLUTE
                    self.y = self.absolute_adressing(bus, &mut ticks);
                    self.ld_set_status(self.y);
                    println!("Loaded {:X} into the Y register!", self.y);
                }
                0xBC => {
                    // LDY_ABSOLUTE_X
                    self.y = self.absolute_adressing_x(bus, &mut ticks);
                    self.ld_set_status(self.y);
                    println!("Loaded {:X} into the Y register!", self.y);
                }
                0x4C => {
                    // ABSOLUTE_JMP
                    self.pc = self.get_absolute_adress(bus, &mut ticks);
                    println!("Jumped to {:X}", self.pc);
                }
                0x6C => {
                    // INDIRECT_JMP
                    self.pc = self.get_indirect_adress(bus, &mut ticks);
                    println!("Jumped to {:X}", self.pc);
                }
                0x20 => {
                    // JSR
                    let target = self.get_absolute_adress(bus, &mut ticks);
                    self.jsr(bus, &mut ticks, target);
                    println!("JSR to {:X}", target);
                }
                0x69 => {
                    // ADC_IMMEDIATE
                    let value = self.immediate_adressing(bus, &mut ticks);
                    self.adc(value);
                    println!("Got {:X} from adc", self.a);
                }
                0x65 => {
                    // ADC_ZP
                    let value = self.zero_page_adressing(bus, &mut ticks);
                    self.adc(value);
                    println!("Got {:X} from adc", self.a);
                }
                0x75 => {
                    // ADC_ZP_X
                    let value = self.zero_page_adressing_x(bus, &mut ticks);
                    self.adc(value);
                    println!("Got {:X} from adc", self.a);
                }
                0x6D => {
                    // ADC_ABSOLUTE
                    let value = self.absolute_adressing(bus, &mut ticks);
                    self.adc(value);
                    println!("Got {:X} from adc", self.a);
                }
                0x7D => {
                    // ADC_ABSOLUTE_X
                    let value = self.absolute_adressing_x(bus, &mut ticks);
                    self.adc(value);
                    println!("Got {:X} from adc", self.a);
                }
                0x79 => {
                    // ADC_ABSOLUTE_Y
                    let value = self.absolute_adressing_y(bus, &mut ticks);
                    self.adc(value);
                    println!("Got {:X} from adc", self.a);
                }
                0x61 => {
                    // ADC_INDIRECT_X
                    let value = self.indirect_indexing_adressing_x(bus, &mut ticks);
                    self.adc(value);
                    println!("Got {:X} from adc", self.a);
                }
                0x71 => {
                    // ADC_INDIRECT_Y
                    let value = self.indexing_indirect_adressing_y(bus, &mut ticks);
                    self.adc(value);
                    println!("Got {:X} from adc", self.a);
                }
                0x29 => {
                    // AND_IMMEDIATE
                    let value = self.immediate_adressing(bus, &mut ticks);
                    self.and(value);
                    println!("Got {:X} from and", self.a);
                }
                0x25 => {
                    // AND_ZP
                    let value = self.zero_page_adressing(bus, &mut ticks);
                    self.and(value);
                    println!("Got {:X} from and", self.a);
                }
                0x35 => {
                    // AND_ZP_X
                    let value = self.zero_page_adressing_x(bus, &mut ticks);
                    self.and(value);
                    println!("Got {:X} from and", self.a);
                }
                0x2D => {
                    // AND_ABSOLUTE
                    let value = self.absolute_adressing(bus, &mut ticks);
                    self.and(value);
                    println!("Got {:X} from and", self.a);
                }
                0x3D => {
                    // AND_ABSOLUTE_X
                    let value = self.absolute_adressing_x(bus, &mut ticks);
                    self.and(value);
                    println!("Got {:X} from and", self.a);
                }
                0x39 => {
                    // AND_ABSOLUTE_Y
                    let value = self.absolute_adressing_y(bus, &mut ticks);
                    self.and(value);
                    println!("Got {:X} from and", self.a);
                }
                0x21 => {
                    // AND_INDIRECT_X
                    let value = self.indirect_indexing_adressing_x(bus, &mut ticks);
                    self.and(value);
                    println!("Got {:X} from and", self.a);
                }
                0x31 => {
                    // AND_INDIRECT_Y
                    let value = self.indexing_indirect_adressing_y(bus, &mut ticks);
                    self.and(value);
                    println!("Got {:X} from and", self.a);
                }
                0x0A => {
                    // ASL_ACCUMULATOR
                    let value = self.accumulator_adressing(&mut ticks);
                    self.asl_acc(value);
                    println!("Got {:X} from asl on {:X}", self.a, value);
                }
                0x06 => {
                    // ASL_ZP
                    let addr = self.get_zp_adress(bus, &mut ticks);
                    self.asl_mem(bus, addr, &mut ticks);
                    println!("Got {:X} from asl at addr {:X}", self.a, addr);
                }
                0x16 => {
                    // ASL_ZP_X
                    let addr = self.get_zp_adress_x(bus, &mut ticks);
                    self.asl_mem(bus, addr, &mut ticks);
                    println!("Got {:X} from asl at addr {:X}", self.a, addr);
                }
                0x0E => {
                    // ASL_ABSOLUTE
                    let addr = self.get_absolute_adress(bus, &mut ticks);
                    self.asl_mem(bus, addr, &mut ticks);
                    println!("Got {:X} from asl at addr {:X}", self.a, addr);
                }
                0x1E => {
                    // ASL_ABSOLUTE_X
                    let addr = self.get_absolute_adress_x(bus, &mut ticks);
                    self.asl_mem(bus, addr, &mut ticks);
                    println!("Got {:X} from asl at addr {:X}", self.a, addr);
                }
                0x90 => {
                    // BRANCH_CARRY
                    ticks += 2;
                    if self.status & 0b00000001 == 0{
                        self.relative_adressing(bus, &mut ticks);
                    } else {
                        self.pc = self.pc.wrapping_add(1); // skip offset byte
                    }
                }
                0xB0 => {
                    // BRANCH_NOT_CARRAY
                    ticks += 2;
                    if self.status & 0b00000001 != 0{
                        self.relative_adressing(bus, &mut ticks);
                    } else {
                        self.pc = self.pc.wrapping_add(1); // skip offset byte
                    }
                }
                0xF0 => {
                    // BRANCH_EQUAL
                    ticks += 2;
                    if self.status & 0b00000010 != 0{
                        self.relative_adressing(bus, &mut ticks);
                    } else {
                        self.pc = self.pc.wrapping_add(1); // skip offset byte
                    }
                }
                0xD0 => {
                    // BRANCH_NOT_EQUAL
                    ticks += 2;
                    if self.status & 0b00000010 == 0{
                        self.relative_adressing(bus, &mut ticks);
                    } else {
                        self.pc = self.pc.wrapping_add(1); // skip offset byte
                    }
                }
                0x30 => {
                    // BRANCH_MINUS
                    ticks += 2;
                    if self.status & 0b10000000 != 0{
                        self.relative_adressing(bus, &mut ticks);
                    } else {
                        self.pc = self.pc.wrapping_add(1); // skip offset byte
                    }
                }
                0x10 => {
                    // BRANCH_POSITIVE
                    ticks += 2;
                    if self.status & 0b10000000 == 0{
                        self.relative_adressing(bus, &mut ticks);
                    } else {
                        self.pc = self.pc.wrapping_add(1); // skip offset byte
                    }
                }
                0x50 => {
                    // BRANCH_NOT_OVERFLOW
                    ticks += 2;
                    if self.status & 0b01000000 == 0{
                        self.relative_adressing(bus, &mut ticks);
                    } else {
                        self.pc = self.pc.wrapping_add(1); // skip offset byte
                    }
                }
                0x70 => {
                    // BRANCH_OVERFLOW
                    ticks += 2;
                    if self.status & 0b01000000 != 0{
                        self.relative_adressing(bus, &mut ticks);
                    } else {
                        self.pc = self.pc.wrapping_add(1); // skip offset byte
                    }
                }
                0x24 => {
                    // BIT_TEST_ZP
                    let value = self.zero_page_adressing(bus, &mut ticks);
                    self.bit_test(value, &mut ticks);
                    println!("Bit test result: Z: {}, V: {}, N: {}", 
                        (self.status >> 1) & 1 == 1,
                        (self.status >> 6) & 1 == 1,
                        (self.status >> 7) & 1 == 1);
                }
                0x2C => {
                    // BIT_TEST_ABS
                    let value = self.absolute_adressing(bus, &mut ticks);
                    self.bit_test(value, &mut ticks);
                    println!("Bit test result: Z: {}, V: {}, N: {}", 
                        (self.status >> 1) & 1 == 1,
                        (self.status >> 6) & 1 == 1,
                        (self.status >> 7) & 1 == 1);
                }
                0x00 => {
                    // BREAK
                    self.brk(bus, &mut ticks);
                    println!("Interupt happend");
                }
                0x18 => {
                    // CLEAR_CARRY_FLAG
                    self.set_status(false, 0);
                    println!("Carry cleared");
                }
                0xD8 => {
                    // CLEAR_DECIMAL_MODE
                    self.set_status(false, 3);
                    println!("Decimal mode cleared");
                }
                0x58 => {
                    // CLEAR_INTERUPT_DISABLE
                    self.set_status(false, 2);
                    println!("Interupt disable cleared");
                }
                0xB8 => {
                    // CLEAR_OVERFLOW_FLAG
                    self.set_status(false, 6);
                    println!("Overflow cleared");
                }
                0xC9 => {
                    // COMPARE_IMMEDIATE
                    let value = self.immediate_adressing(bus, &mut ticks);
                    self.cmp_set_status(value);
                    ticks += 1;
                    println!("Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000)
                }
                0xC5 => {
                    // COMPARE_ZP
                    let value = self.zero_page_adressing(bus, &mut ticks);
                    self.cmp_set_status(value);
                    ticks += 1;
                    println!("Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000)
                }
                0xD5 => {
                    // COMPARE_ZP_X
                    let value = self.zero_page_adressing_x(bus, &mut ticks);
                    self.cmp_set_status(value);
                    ticks += 1;
                    println!("Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000)
                }
                0xCD => {
                    // COMPARE_ABSOLUTE
                    let value = self.absolute_adressing(bus, &mut ticks);
                    self.cmp_set_status(value);
                    ticks += 1;
                    println!("Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000)
                }
                0xDD => {
                    // COMPARE_ABSOLUTE_X
                    let value = self.absolute_adressing_x(bus, &mut ticks);
                    self.cmp_set_status(value);
                    ticks += 1;
                    println!("Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000)
                }
                0xD9 => {
                    // COMPARE_ABSOLUTE_Y
                    let value = self.absolute_adressing_y(bus, &mut ticks);
                    self.cmp_set_status(value);
                    ticks += 1;
                    println!("Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000)
                }
                0xC1 => {
                    // COMPARE_INDIRECT_X
                    let value = self.indirect_indexing_adressing_x(bus, &mut ticks);
                    self.cmp_set_status(value);
                    ticks += 1;
                    println!("Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000)
                }
                0xD1 => {
                    // COMPARE_INDIRECT_Y
                    let value = self.indexing_indirect_adressing_y(bus, &mut ticks);
                    self.cmp_set_status(value);
                    ticks += 1;
                    println!("Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000)
                }
                0xE0 => {
                    // CPX_IMMEDIATE
                    let value = self.immediate_adressing(bus, &mut ticks);
                    self.cpx_set_status(value);
                    ticks += 1;
                    println!("Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000)
                }
                0xE4 => {
                    // CPX_ZP
                    let value = self.zero_page_adressing(bus, &mut ticks);
                    self.cpx_set_status(value);
                    ticks += 1;
                    println!("Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000)
                }
                0xEC => {
                    // CPX_ABSOLUTE
                    let value = self.absolute_adressing(bus, &mut ticks);
                    self.cpx_set_status(value);
                    ticks += 1;
                    println!("Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000);
                }
                0xC0 => {
                    // CPY_IMMEDIATE
                    let value = self.immediate_adressing(bus, &mut ticks);
                    self.cpy_set_status(value);
                    ticks += 1;
                    println!("Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000);
                }
                0xC4 => {
                    // CPY_ZP
                    let value = self.zero_page_adressing(bus, &mut ticks);
                    self.cpy_set_status(value);
                    ticks += 1;
                    println!("Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000);
                }
                0xCC => {
                    // CPY_ABSOLUTE
                    let value = self.absolute_adressing(bus, &mut ticks);
                    self.cpy_set_status(value);
                    ticks += 1;
                    println!("Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                        value,
                        self.status & 0b00000001,
                        self.status & 0b00000010,
                        self.status & 0b10000000);
                }
                0xC6 => {
                    // DEC_ZP
                    let addr = self.get_zp_adress(bus, &mut ticks);
                    self.dec(bus, &mut ticks, addr as u16);
                    println!("Decremented addr: {:X}", addr);
                }
                0xD6 => {
                    // DEC_ZP_X
                    let addr = self.get_zp_adress_x(bus, &mut ticks);
                    self.dec(bus, &mut ticks, addr as u16);
                    println!("Decremented addr: {:X}", addr);
                }
                0xCE => {
                    // DEC_ABSOLUTE
                    let addr = self.get_absolute_adress(bus, &mut ticks);
                    self.dec(bus, &mut ticks, addr as u16);
                    println!("Decremented addr: {:X}", addr);
                }
                0xDE => {
                    // DEC_ABSOLUTE_X
                    let addr = self.get_absolute_adress_x(bus, &mut ticks);
                    self.dec(bus, &mut ticks, addr as u16);
                    println!("Decremented addr: {:X}", addr);
                }
                0xCA => {
                    // DEC_X
                    self.decx(&mut ticks);
                    println!("Decremented the x register to {:X}", self.x);
                }
                0x88 => {
                    // DEC_Y
                    self.decy(&mut ticks);
                    println!("Decremented the y register to {:X}", self.x);
                }
                0x49 => {
                    // EOR_IMMEDIATE
                    let value = self.immediate_adressing(bus, &mut ticks);
                    self.eor(&mut ticks, value);
                    println!("Exlusive ORe'd {:X} with {:X}", self.a, value);
                }
                0x45 => {
                    // EOR_ZERO_PAGE
                    let value = self.zero_page_adressing(bus, &mut ticks);
                    self.eor(&mut ticks, value);
                    println!("Exlusive ORe'd {:X} with {:X}", self.a, value);
                }
                0x55 => {
                    // EOR_ZERO_PAGE_X
                    let value = self.zero_page_adressing_x(bus, &mut ticks);
                    self.eor(&mut ticks, value);
                    println!("Exlusive ORe'd {:X} with {:X}", self.a, value);
                }
                0x4D => {
                    // EOR_ABSOLUTE
                    let value = self.absolute_adressing(bus, &mut ticks);
                    self.eor(&mut ticks, value);
                    println!("Exlusive ORe'd {:X} with {:X}", self.a, value);
                }
                0x5D => {
                    // EOR_ABSOLUTE_X
                    let value = self.absolute_adressing_x(bus, &mut ticks);
                    self.eor(&mut ticks, value);
                    println!("Exlusive ORe'd {:X} with {:X}", self.a, value);
                }
                0x59 => {
                    // EOR_ABSOLUTE_Y
                    let value = self.absolute_adressing_y(bus, &mut ticks);
                    self.eor(&mut ticks, value);
                    println!("Exlusive ORe'd {:X} with {:X}", self.a, value);
                }
                0x41 => {
                    // EOR_INDIRECT_X
                    let value = self.indirect_indexing_adressing_x(bus, &mut ticks);
                    self.eor(&mut ticks, value);
                    println!("Exlusive ORe'd {:X} with {:X}", self.a, value);
                }
                0x51 => {
                    // EOR_INDIRECT_Y
                    let value = self.indexing_indirect_adressing_y(bus, &mut ticks);
                    self.eor(&mut ticks, value);
                    println!("Exlusive ORe'd {:X} with {:X}", self.a, value);
                }
                0xE6 => {
                    // INC_ZP
                    let addr = self.get_zp_adress(bus, &mut ticks);
                    self.inc(bus, &mut ticks, addr as u16);
                    println!("Increment addr: {:X}", addr);
                }
                0xF6 => {
                    // INC_ZP_X
                    let addr = self.get_zp_adress_x(bus, &mut ticks);
                    self.inc(bus, &mut ticks, addr as u16);
                    println!("Increment addr: {:X}", addr);
                }
                0xEE => {
                    // INC_ABSOLUTE
                    let addr = self.get_absolute_adress(bus, &mut ticks);
                    self.inc(bus, &mut ticks, addr as u16);
                    println!("Increment addr: {:X}", addr);
                }
                0xFE => {
                    // INC_ABSOLUTE_X
                    let addr = self.get_absolute_adress_x(bus, &mut ticks);
                    self.inc(bus, &mut ticks, addr as u16);
                    println!("Increment addr: {:X}", addr);
                }
                0xE8 => {
                    // INC_X
                    self.incx(&mut ticks);
                    println!("Increment the x register to {:X}", self.x);
                }
                0xC8 => {
                    // INC_Y
                    self.incy(&mut ticks);
                    println!("Increment the y register to {:X}", self.x);
                }
                0xEA => {
                    // NOP
                    ticks += 1;
                    println!("NOP");
                }
                _ => {}
            }
        }
    }
}