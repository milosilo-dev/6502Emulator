use crate::bus::Bus;

use super::cpu::CPU;

impl CPU {
    // Executes `steps` number of instructions
    pub fn step(&mut self, bus: &mut Bus, steps: u32) -> u32 {
        let mut ret: u32 = 0;
        for _ in 0..steps {
            ret += self.execute(bus);
        }
        ret
    }

    pub fn execute(&mut self, bus: &mut Bus) -> u32 {
        let mut ticks = 0;
        let ins = self.fetch_byte(bus);
        ticks += 1;
        match ins {
            0xA9 => {
                // LDA_IM
                self.a = self.immediate_adressing(bus, &mut ticks);
                self.ld_set_status(self.a);
                self.config.logger.log(format!("Loaded {:X} into the A register!", self.a));
            }
            0xA5 => {
                // LDA_ZP
                self.a = self.zero_page_adressing(bus, &mut ticks);
                self.ld_set_status(self.a);
                self.config.logger.log(format!("Loaded {:X} into the A register!", self.a));
            }
            0xB5 => {
                // LDA_ZP_X
                self.a = self.zero_page_adressing_x(bus, &mut ticks);
                self.ld_set_status(self.a);
                self.config.logger.log(format!("Loaded {:X} into the A register!", self.a));
            }
            0xAD => {
                // LDA_ABSOLUTE
                self.a = self.absolute_adressing(bus, &mut ticks);
                self.ld_set_status(self.a);
                self.config.logger.log(format!("Loaded {:X} into the A register!", self.a));
            }
            0xBD => {
                // LDA_ABSOLUTE_X
                self.a = self.absolute_adressing_x(bus, &mut ticks);
                self.ld_set_status(self.a);
                self.config.logger.log(format!("Loaded {:X} into the A register!", self.a));
            }
            0xB9 => {
                // LDA_ABSOLUTE_Y
                self.a = self.absolute_adressing_y(bus, &mut ticks);
                self.ld_set_status(self.a);
                self.config.logger.log(format!("Loaded {:X} into the A register!", self.a));
            }
            0xA1 => {
                // LDA_INDIRECT_INDEXING
                self.a = self.indirect_indexing_adressing_x(bus, &mut ticks);
                self.ld_set_status(self.a);
                self.config.logger.log(format!("Loaded {:X} into the A register!", self.a));
            }
            0xB1 => {
                // LDA_INDEXING_INDIRECT
                self.a = self.indexing_indirect_adressing_y(bus, &mut ticks);
                self.ld_set_status(self.a);
                self.config.logger.log(format!("Loaded {:X} into the A register!", self.a));
            }
            0xA2 => {
                // IMMEDIATE_LDX
                self.x = self.immediate_adressing(bus, &mut ticks);
                self.ld_set_status(self.x);
                self.config.logger.log(format!("Loaded {:X} into the X register!", self.x));
            }
            0xA6 => {
                // LDX_ZP
                self.x = self.zero_page_adressing(bus, &mut ticks);
                self.ld_set_status(self.x);
                self.config.logger.log(format!("Loaded {:X} into the X register!", self.x));
            }
            0xB6 => {
                // LDX_ZP_Y
                self.x = self.zero_page_adressing_y(bus, &mut ticks);
                self.ld_set_status(self.x);
                self.config.logger.log(format!("Loaded {:X} into the X register!", self.x));
            }
            0xAE => {
                // LDX_ABSOLUTE
                self.x = self.absolute_adressing(bus, &mut ticks);
                self.ld_set_status(self.x);
                self.config.logger.log(format!("Loaded {:X} into the X register!", self.x));
            }
            0xBE => {
                // LDX_ABSOLUTE_Y
                self.x = self.absolute_adressing_y(bus, &mut ticks);
                self.ld_set_status(self.x);
                self.config.logger.log(format!("Loaded {:X} into the X register!", self.x));
            }
            0xA0 => {
                // IMMEDIATE_LDY
                self.y = self.immediate_adressing(bus, &mut ticks);
                self.ld_set_status(self.y);
                self.config.logger.log(format!("Loaded {:X} into the Y register!", self.y));
            }
            0xA4 => {
                // LDY_ZP
                self.y = self.zero_page_adressing(bus, &mut ticks);
                self.ld_set_status(self.y);
                self.config.logger.log(format!("Loaded {:X} into the Y register!", self.y));
            }
            0xB4 => {
                // LDY_ZP_X
                self.y = self.zero_page_adressing_x(bus, &mut ticks);
                self.ld_set_status(self.y);
                self.config.logger.log(format!("Loaded {:X} into the Y register!", self.y));
            }
            0xAC => {
                // LDY_ABSOLUTE
                self.y = self.absolute_adressing(bus, &mut ticks);
                self.ld_set_status(self.y);
                self.config.logger.log(format!("Loaded {:X} into the Y register!", self.y));
            }
            0xBC => {
                // LDY_ABSOLUTE_X
                self.y = self.absolute_adressing_x(bus, &mut ticks);
                self.ld_set_status(self.y);
                self.config.logger.log(format!("Loaded {:X} into the Y register!", self.y));
            }
            0x4C => {
                // ABSOLUTE_JMP
                self.pc = self.get_absolute_adress(bus, &mut ticks);
                self.config.logger.log(format!("Jumped to {:X}", self.pc));
            }
            0x6C => {
                // INDIRECT_JMP
                self.pc = self.get_indirect_adress(bus, &mut ticks);
                self.config.logger.log(format!("Jumped to {:X}", self.pc));
            }
            0x20 => {
                // JSR
                let target = self.get_absolute_adress(bus, &mut ticks);
                self.jsr(bus, &mut ticks, target);
                self.config.logger.log(format!("JSR to {:X}", target));
            }
            0x69 => {
                // ADC_IMMEDIATE
                let value = self.immediate_adressing(bus, &mut ticks);
                self.adc(value);
                self.config.logger.log(format!("Got {:X} from adc", self.a));
            }
            0x65 => {
                // ADC_ZP
                let value = self.zero_page_adressing(bus, &mut ticks);
                self.adc(value);
                self.config.logger.log(format!("Got {:X} from adc", self.a));
            }
            0x75 => {
                // ADC_ZP_X
                let value = self.zero_page_adressing_x(bus, &mut ticks);
                self.adc(value);
                self.config.logger.log(format!("Got {:X} from adc", self.a));
            }
            0x6D => {
                // ADC_ABSOLUTE
                let value = self.absolute_adressing(bus, &mut ticks);
                self.adc(value);
                self.config.logger.log(format!("Got {:X} from adc", self.a));
            }
            0x7D => {
                // ADC_ABSOLUTE_X
                let value = self.absolute_adressing_x(bus, &mut ticks);
                self.adc(value);
                self.config.logger.log(format!("Got {:X} from adc", self.a));
            }
            0x79 => {
                // ADC_ABSOLUTE_Y
                let value = self.absolute_adressing_y(bus, &mut ticks);
                self.adc(value);
                self.config.logger.log(format!("Got {:X} from adc", self.a));
            }
            0x61 => {
                // ADC_INDIRECT_X
                let value = self.indirect_indexing_adressing_x(bus, &mut ticks);
                self.adc(value);
                self.config.logger.log(format!("Got {:X} from adc", self.a));
            }
            0x71 => {
                // ADC_INDIRECT_Y
                let value = self.indexing_indirect_adressing_y(bus, &mut ticks);
                self.adc(value);
                self.config.logger.log(format!("Got {:X} from adc", self.a));
            }
            0x29 => {
                // AND_IMMEDIATE
                let value = self.immediate_adressing(bus, &mut ticks);
                self.and(value);
                self.config.logger.log(format!("Got {:X} from and", self.a));
            }
            0x25 => {
                // AND_ZP
                let value = self.zero_page_adressing(bus, &mut ticks);
                self.and(value);
                self.config.logger.log(format!("Got {:X} from and", self.a));
            }
            0x35 => {
                // AND_ZP_X
                let value = self.zero_page_adressing_x(bus, &mut ticks);
                self.and(value);
                self.config.logger.log(format!("Got {:X} from and", self.a));
            }
            0x2D => {
                // AND_ABSOLUTE
                let value = self.absolute_adressing(bus, &mut ticks);
                self.and(value);
                self.config.logger.log(format!("Got {:X} from and", self.a));
            }
            0x3D => {
                // AND_ABSOLUTE_X
                let value = self.absolute_adressing_x(bus, &mut ticks);
                self.and(value);
                self.config.logger.log(format!("Got {:X} from and", self.a));
            }
            0x39 => {
                // AND_ABSOLUTE_Y
                let value = self.absolute_adressing_y(bus, &mut ticks);
                self.and(value);
                self.config.logger.log(format!("Got {:X} from and", self.a));
            }
            0x21 => {
                // AND_INDIRECT_X
                let value = self.indirect_indexing_adressing_x(bus, &mut ticks);
                self.and(value);
                self.config.logger.log(format!("Got {:X} from and", self.a));
            }
            0x31 => {
                // AND_INDIRECT_Y
                let value = self.indexing_indirect_adressing_y(bus, &mut ticks);
                self.and(value);
                self.config.logger.log(format!("Got {:X} from and", self.a));
            }
            0x0A => {
                // ASL_ACCUMULATOR
                let value = self.accumulator_adressing(&mut ticks);
                self.asl_acc(value);
                self.config.logger.log(format!("Got {:X} from asl on {:X}", self.a, value));
            }
            0x06 => {
                // ASL_ZP
                let addr = self.get_zp_adress(bus, &mut ticks);
                self.asl_mem(bus, addr, &mut ticks);
                self.config.logger.log(format!("Got {:X} from asl at addr {:X}", self.a, addr));
            }
            0x16 => {
                // ASL_ZP_X
                let addr = self.get_zp_adress_x(bus, &mut ticks);
                self.asl_mem(bus, addr, &mut ticks);
                self.config.logger.log(format!("Got {:X} from asl at addr {:X}", self.a, addr));
            }
            0x0E => {
                // ASL_ABSOLUTE
                let addr = self.get_absolute_adress(bus, &mut ticks);
                self.asl_mem(bus, addr, &mut ticks);
                self.config.logger.log(format!("Got {:X} from asl at addr {:X}", self.a, addr));
            }
            0x1E => {
                // ASL_ABSOLUTE_X
                let addr = self.get_absolute_adress_x(bus, &mut ticks);
                self.asl_mem(bus, addr, &mut ticks);
                self.config.logger.log(format!("Got {:X} from asl at addr {:X}", self.a, addr));
            }
            0x4A => {
                // LSR_ACCUMULATOR
                let value = self.accumulator_adressing(&mut ticks);
                self.lsr_acc(value);
                self.config.logger.log(format!("Got {:X} from lsr on {:X}", self.a, value));
            }
            0x46 => {
                // LSR_ZP
                let addr = self.get_zp_adress(bus, &mut ticks);
                self.lsr_mem(bus, addr, &mut ticks);
                self.config.logger.log(format!("Got {:X} from lsr at addr {:X}", self.a, addr));
            }
            0x56 => {
                // LSR_ZP_X
                let addr = self.get_zp_adress_x(bus, &mut ticks);
                self.lsr_mem(bus, addr, &mut ticks);
                self.config.logger.log(format!("Got {:X} from lsr at addr {:X}", self.a, addr));
            }
            0x4E => {
                // LSR_ABSOLUTE
                let addr = self.get_absolute_adress(bus, &mut ticks);
                self.lsr_mem(bus, addr, &mut ticks);
                self.config.logger.log(format!("Got {:X} from lsr at addr {:X}", self.a, addr));
            }
            0x5E => {
                // LSR_ABSOLUTE_X
                let addr = self.get_absolute_adress_x(bus, &mut ticks);
                self.lsr_mem(bus, addr, &mut ticks);
                self.config.logger.log(format!("Got {:X} from lsr at addr {:X}", self.a, addr));
            }
            0xB0 => {
                // BRANCH_CARRY
                ticks += 2;
                if self.status & 0b00000001 != 0 {
                    self.relative_adressing(bus, &mut ticks);
                } else {
                    self.pc = self.pc.wrapping_add(1); // skip offset byte
                }
            }
            0x90 => {
                // BRANCH_NOT_CARRAY
                ticks += 2;
                if self.status & 0b00000001 == 0 {
                    self.relative_adressing(bus, &mut ticks);
                } else {
                    self.pc = self.pc.wrapping_add(1); // skip offset byte
                }
            }
            0xF0 => {
                // BRANCH_EQUAL
                ticks += 2;
                if self.status & 0b00000010 != 0 {
                    self.relative_adressing(bus, &mut ticks);
                } else {
                    self.pc = self.pc.wrapping_add(1); // skip offset byte
                }
            }
            0xD0 => {
                // BRANCH_NOT_EQUAL
                ticks += 2;
                if self.status & 0b00000010 == 0 {
                    self.relative_adressing(bus, &mut ticks);
                } else {
                    self.pc = self.pc.wrapping_add(1); // skip offset byte
                }
            }
            0x30 => {
                // BRANCH_MINUS
                ticks += 2;
                if self.status & 0b10000000 != 0 {
                    self.relative_adressing(bus, &mut ticks);
                } else {
                    self.pc = self.pc.wrapping_add(1); // skip offset byte
                }
            }
            0x10 => {
                // BRANCH_POSITIVE
                ticks += 2;
                if self.status & 0b10000000 == 0 {
                    self.relative_adressing(bus, &mut ticks);
                } else {
                    self.pc = self.pc.wrapping_add(1); // skip offset byte
                }
            }
            0x50 => {
                // BRANCH_NOT_OVERFLOW
                ticks += 2;
                if self.status & 0b01000000 == 0 {
                    self.relative_adressing(bus, &mut ticks);
                } else {
                    self.pc = self.pc.wrapping_add(1); // skip offset byte
                }
            }
            0x70 => {
                // BRANCH_OVERFLOW
                ticks += 2;
                if self.status & 0b01000000 != 0 {
                    self.relative_adressing(bus, &mut ticks);
                } else {
                    self.pc = self.pc.wrapping_add(1); // skip offset byte
                }
            }
            0x24 => {
                // BIT_TEST_ZP
                let value = self.zero_page_adressing(bus, &mut ticks);
                self.bit_test(value, &mut ticks);
                self.config.logger.log(format!(
                    "Bit test result: Z: {}, V: {}, N: {}",
                    (self.status >> 1) & 1 == 1,
                    (self.status >> 6) & 1 == 1,
                    (self.status >> 7) & 1 == 1
                ));
            }
            0x2C => {
                // BIT_TEST_ABS
                let value = self.absolute_adressing(bus, &mut ticks);
                self.bit_test(value, &mut ticks);
                self.config.logger.log(format!(
                    "Bit test result: Z: {}, V: {}, N: {}",
                    (self.status >> 1) & 1 == 1,
                    (self.status >> 6) & 1 == 1,
                    (self.status >> 7) & 1 == 1
                ));
            }
            0x00 => {
                // BREAK
                self.brk(bus, &mut ticks);
                self.config.logger.log(format!("Interupt happend"));
            }
            0x18 => {
                // CLEAR_CARRY_FLAG
                self.set_status(false, 0);
                self.config.logger.log(format!("Carry cleared"));
            }
            0xD8 => {
                // CLEAR_DECIMAL_MODE
                self.set_status(false, 3);
                self.config.logger.log(format!("Decimal mode cleared"));
            }
            0x58 => {
                // CLEAR_INTERUPT_DISABLE
                self.set_status(false, 2);
                self.config.logger.log(format!("Interupt disable cleared"));
            }
            0xB8 => {
                // CLEAR_OVERFLOW_FLAG
                self.set_status(false, 6);
                self.config.logger.log(format!("Overflow cleared"));
            }
            0xC9 => {
                // COMPARE_IMMEDIATE
                let value = self.immediate_adressing(bus, &mut ticks);
                self.cmp_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xC5 => {
                // COMPARE_ZP
                let value = self.zero_page_adressing(bus, &mut ticks);
                self.cmp_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xD5 => {
                // COMPARE_ZP_X
                let value = self.zero_page_adressing_x(bus, &mut ticks);
                self.cmp_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xCD => {
                // COMPARE_ABSOLUTE
                let value = self.absolute_adressing(bus, &mut ticks);
                self.cmp_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xDD => {
                // COMPARE_ABSOLUTE_X
                let value = self.absolute_adressing_x(bus, &mut ticks);
                self.cmp_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xD9 => {
                // COMPARE_ABSOLUTE_Y
                let value = self.absolute_adressing_y(bus, &mut ticks);
                self.cmp_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xC1 => {
                // COMPARE_INDIRECT_X
                let value = self.indirect_indexing_adressing_x(bus, &mut ticks);
                self.cmp_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xD1 => {
                // COMPARE_INDIRECT_Y
                let value = self.indexing_indirect_adressing_y(bus, &mut ticks);
                self.cmp_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared acc and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xE0 => {
                // CPX_IMMEDIATE
                let value = self.immediate_adressing(bus, &mut ticks);
                self.cpx_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xE4 => {
                // CPX_ZP
                let value = self.zero_page_adressing(bus, &mut ticks);
                self.cpx_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xEC => {
                // CPX_ABSOLUTE
                let value = self.absolute_adressing(bus, &mut ticks);
                self.cpx_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xC0 => {
                // CPY_IMMEDIATE
                let value = self.immediate_adressing(bus, &mut ticks);
                self.cpy_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xC4 => {
                // CPY_ZP
                let value = self.zero_page_adressing(bus, &mut ticks);
                self.cpy_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xCC => {
                // CPY_ABSOLUTE
                let value = self.absolute_adressing(bus, &mut ticks);
                self.cpy_set_status(value);
                ticks += 1;
                self.config.logger.log(format!(
                    "Compared X and {:X}, C: {:X}, Z: {:X}, N: {:X}",
                    value,
                    self.status & 0b00000001,
                    self.status & 0b00000010,
                    self.status & 0b10000000
                ));
            }
            0xC6 => {
                // DEC_ZP
                let addr = self.get_zp_adress(bus, &mut ticks);
                self.dec(bus, &mut ticks, addr as u16);
                self.config.logger.log(format!("Decremented addr: {:X}", addr));
            }
            0xD6 => {
                // DEC_ZP_X
                let addr = self.get_zp_adress_x(bus, &mut ticks);
                self.dec(bus, &mut ticks, addr as u16);
                self.config.logger.log(format!("Decremented addr: {:X}", addr));
            }
            0xCE => {
                // DEC_ABSOLUTE
                let addr = self.get_absolute_adress(bus, &mut ticks);
                self.dec(bus, &mut ticks, addr as u16);
                self.config.logger.log(format!("Decremented addr: {:X}", addr));
            }
            0xDE => {
                // DEC_ABSOLUTE_X
                let addr = self.get_absolute_adress_x(bus, &mut ticks);
                self.dec(bus, &mut ticks, addr as u16);
                self.config.logger.log(format!("Decremented addr: {:X}", addr));
            }
            0xCA => {
                // DEC_X
                self.decx(&mut ticks);
                self.config.logger.log(format!("Decremented the x register to {:X}", self.x));
            }
            0x88 => {
                // DEC_Y
                self.decy(&mut ticks);
                self.config.logger.log(format!("Decremented the y register to {:X}", self.y));
            }
            0x49 => {
                // EOR_IMMEDIATE
                let value = self.immediate_adressing(bus, &mut ticks);
                self.eor(&mut ticks, value);
                self.config.logger.log(format!("Exlusive ORe'd {:X} with {:X}", self.a, value));
            }
            0x45 => {
                // EOR_ZERO_PAGE
                let value = self.zero_page_adressing(bus, &mut ticks);
                self.eor(&mut ticks, value);
                self.config.logger.log(format!("Exlusive ORe'd {:X} with {:X}", self.a, value));
            }
            0x55 => {
                // EOR_ZERO_PAGE_X
                let value = self.zero_page_adressing_x(bus, &mut ticks);
                self.eor(&mut ticks, value);
                self.config.logger.log(format!("Exlusive ORe'd {:X} with {:X}", self.a, value));
            }
            0x4D => {
                // EOR_ABSOLUTE
                let value = self.absolute_adressing(bus, &mut ticks);
                self.eor(&mut ticks, value);
                self.config.logger.log(format!("Exlusive ORe'd {:X} with {:X}", self.a, value));
            }
            0x5D => {
                // EOR_ABSOLUTE_X
                let value = self.absolute_adressing_x(bus, &mut ticks);
                self.eor(&mut ticks, value);
                self.config.logger.log(format!("Exlusive ORe'd {:X} with {:X}", self.a, value));
            }
            0x59 => {
                // EOR_ABSOLUTE_Y
                let value = self.absolute_adressing_y(bus, &mut ticks);
                self.eor(&mut ticks, value);
                self.config.logger.log(format!("Exlusive ORe'd {:X} with {:X}", self.a, value));
            }
            0x41 => {
                // EOR_INDIRECT_X
                let value = self.indirect_indexing_adressing_x(bus, &mut ticks);
                self.eor(&mut ticks, value);
                self.config.logger.log(format!("Exlusive ORe'd {:X} with {:X}", self.a, value));
            }
            0x51 => {
                // EOR_INDIRECT_Y
                let value = self.indexing_indirect_adressing_y(bus, &mut ticks);
                self.eor(&mut ticks, value);
                self.config.logger.log(format!("Exlusive ORe'd {:X} with {:X}", self.a, value));
            }
            0xE6 => {
                // INC_ZP
                let addr = self.get_zp_adress(bus, &mut ticks);
                self.inc(bus, &mut ticks, addr as u16);
                self.config.logger.log(format!("Increment addr: {:X}", addr));
            }
            0xF6 => {
                // INC_ZP_X
                let addr = self.get_zp_adress_x(bus, &mut ticks);
                self.inc(bus, &mut ticks, addr as u16);
                self.config.logger.log(format!("Increment addr: {:X}", addr));
            }
            0xEE => {
                // INC_ABSOLUTE
                let addr = self.get_absolute_adress(bus, &mut ticks);
                self.inc(bus, &mut ticks, addr as u16);
                self.config.logger.log(format!("Increment addr: {:X}", addr));
            }
            0xFE => {
                // INC_ABSOLUTE_X
                let addr = self.get_absolute_adress_x(bus, &mut ticks);
                self.inc(bus, &mut ticks, addr as u16);
                self.config.logger.log(format!("Increment addr: {:X}", addr));
            }
            0xE8 => {
                // INC_X
                self.incx(&mut ticks);
                self.config.logger.log(format!("Increment the x register to {:X}", self.x));
            }
            0xC8 => {
                // INC_Y
                self.incy(&mut ticks);
                self.config.logger.log(format!("Increment the y register to {:X}", self.x));
            }
            0x09 => {
                // ORA_IMMEDIATE
                let value = self.immediate_adressing(bus, &mut ticks);
                self.ora(value, &mut ticks);
                self.config.logger.log(format!("Self.config.logger.logical Inclusive OR on {:X}", self.x));
            }
            0x05 => {
                // ORA_ZERO_PAGE
                let value = self.zero_page_adressing(bus, &mut ticks);
                self.ora(value, &mut ticks);
                self.config.logger.log(format!("Self.config.logger.logical Inclusive OR on {:X}", self.x));
            }
            0x15 => {
                // ORA_ZERO_PAGE_X
                let value = self.zero_page_adressing_x(bus, &mut ticks);
                self.ora(value, &mut ticks);
                self.config.logger.log(format!("Self.config.logger.logical Inclusive OR on {:X}", self.x));
            }
            0x0D => {
                // ORA_ABSOLUTE
                let value = self.absolute_adressing(bus, &mut ticks);
                self.ora(value, &mut ticks);
                self.config.logger.log(format!("Self.config.logger.logical Inclusive OR on {:X}", self.x));
            }
            0x1D => {
                // ORA_ABSOLUTE_X
                let value = self.absolute_adressing_x(bus, &mut ticks);
                self.ora(value, &mut ticks);
                self.config.logger.log(format!("Self.config.logger.logical Inclusive OR on {:X}", self.x));
            }
            0x19 => {
                // ORA_ABSOLUTE_Y
                let value = self.absolute_adressing_y(bus, &mut ticks);
                self.ora(value, &mut ticks);
                self.config.logger.log(format!("Self.config.logger.logical Inclusive OR on {:X}", self.x));
            }
            0x01 => {
                // ORA_INDIRECT_X
                let value = self.indirect_indexing_adressing_x(bus, &mut ticks);
                self.ora(value, &mut ticks);
                self.config.logger.log(format!("Self.config.logger.logical Inclusive OR on {:X}", self.x));
            }
            0x11 => {
                // ORA_INDIRECT_Y
                let value = self.indexing_indirect_adressing_y(bus, &mut ticks);
                self.ora(value, &mut ticks);
                self.config.logger.log(format!("Self.config.logger.logical Inclusive OR on {:X}", self.x));
            }
            0x48 => {
                // PHA
                self.pha(bus, &mut ticks);
                self.config.logger.log(format!(
                    "Pushed the contense of the accumulator to the stack"
                ));
            }
            0x08 => {
                // PHP
                self.php(bus, &mut ticks);
                self.config.logger.log(format!("Pushed the processor status to the stack"));
            }
            0x68 => {
                // PLA
                self.pla(bus, &mut ticks);
                self.config.logger.log(format!("Got {:X} from the stack", self.a));
            }
            0x28 => {
                // PLP
                self.plp(bus, &mut ticks);
                self.config.logger.log(format!("Got {:X} from the stack", self.status));
            }
            0x2A => {
                // ROL_ACC
                self.rol_a(&mut ticks);
                self.config.logger.log(format!("Rotated the acc left"));
            }
            0x26 => {
                // ROL_ZP
                let addr = self.get_zp_adress(bus, &mut ticks);
                self.rol(bus, addr as u16, &mut ticks);
                self.config.logger.log(format!("Rotated the {:X} left", addr));
            }
            0x36 => {
                // ROL_ZP_X
                let addr = self.get_zp_adress_x(bus, &mut ticks);
                self.rol(bus, addr as u16, &mut ticks);
                self.config.logger.log(format!("Rotated the {:X} left", addr));
            }
            0x2E => {
                // ROL_ABS
                let addr = self.get_absolute_adress(bus, &mut ticks);
                self.rol(bus, addr as u16, &mut ticks);
                self.config.logger.log(format!("Rotated the {:X} left", addr));
            }
            0x3E => {
                // ROL_ABS_X
                let addr = self.get_absolute_adress_x(bus, &mut ticks);
                self.rol(bus, addr as u16, &mut ticks);
                self.config.logger.log(format!("Rotated the {:X} left", addr));
            }
            0x6A => {
                // ROR_ACC
                self.ror_a(&mut ticks);
                self.config.logger.log(format!("Rotated the acc right"));
            }
            0x66 => {
                // ROR_ZP
                let addr = self.get_zp_adress(bus, &mut ticks);
                self.ror(bus, addr as u16, &mut ticks);
                self.config.logger.log(format!("Rotated the {:X} right", addr));
            }
            0x76 => {
                // ROR_ZP_X
                let addr = self.get_zp_adress_x(bus, &mut ticks);
                self.ror(bus, addr as u16, &mut ticks);
                self.config.logger.log(format!("Rotated the {:X} right", addr));
            }
            0x6E => {
                // ROR_ABS
                let addr = self.get_absolute_adress(bus, &mut ticks);
                self.ror(bus, addr as u16, &mut ticks);
                self.config.logger.log(format!("Rotated the {:X} right", addr));
            }
            0x7E => {
                // ROR_ABS_X
                let addr = self.get_absolute_adress_x(bus, &mut ticks);
                self.ror(bus, addr as u16, &mut ticks);
                self.config.logger.log(format!("Rotated the {:X} right", addr));
            }
            0x40 => {
                // RTI
                self.rti(bus, &mut ticks);
                self.config.logger.log(format!("Returned from Interrupt"));
            }
            0x60 => {
                // RTS
                self.rts(bus, &mut ticks);
                self.config.logger.log(format!("Returned from subroutine"));
            }
            0x38 => {
                // SEC
                self.set_status(true, 0);
                ticks += 1;
                self.config.logger.log(format!("Set carry flag"));
            }
            0xF8 => {
                // SED
                self.set_status(true, 3);
                ticks += 1;
                self.config.logger.log(format!("Set decimal flag"));
            }
            0x78 => {
                // SEI
                self.set_status(true, 2);
                ticks += 1;
                self.config.logger.log(format!("Set interrupt disable"));
            }
            0xE9 => {
                // SBC_IMMD
                let value = self.immediate_adressing(bus, &mut ticks);
                self.sbc(value);
                self.config.logger.log(format!("Subtracted {:X} from the acc", value));
            }
            0xE5 => {
                // SBC_ZP
                let value = self.zero_page_adressing(bus, &mut ticks);
                self.sbc(value);
                self.config.logger.log(format!("Subtracted {:X} from the acc", value));
            }
            0xF5 => {
                // SBC_ZP_X
                let value = self.zero_page_adressing_x(bus, &mut ticks);
                self.sbc(value);
                self.config.logger.log(format!("Subtracted {:X} from the acc", value));
            }
            0xED => {
                // SBC_ABSOLUTE
                let value = self.absolute_adressing(bus, &mut ticks);
                self.sbc(value);
                self.config.logger.log(format!("Subtracted {:X} from the acc", value));
            }
            0xFD => {
                // SBC_ABSOLUTE_X
                let value = self.absolute_adressing_x(bus, &mut ticks);
                self.sbc(value);
                self.config.logger.log(format!("Subtracted {:X} from the acc", value));
            }
            0xF9 => {
                // SBC_ABSOLUTE_Y
                let value = self.absolute_adressing_y(bus, &mut ticks);
                self.sbc(value);
                self.config.logger.log(format!("Subtracted {:X} from the acc", value));
            }
            0xE1 => {
                // SBC_INDIRECT_X
                let value = self.indirect_indexing_adressing_x(bus, &mut ticks);
                self.sbc(value);
                self.config.logger.log(format!("Subtracted {:X} from the acc", value));
            }
            0xF1 => {
                // SBC_INDIRECT_Y
                let value = self.indexing_indirect_adressing_y(bus, &mut ticks);
                self.sbc(value);
                self.config.logger.log(format!("Subtracted {:X} from the acc", value));
            }
            0x85 => {
                // STA_ZERO_PAGE
                let addr = self.get_zp_adress(bus, &mut ticks);
                self.sta(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored Acc at {:X}", addr));
            }
            0x95 => {
                // STA_ZERO_PAGE_X
                let addr = self.get_zp_adress_x(bus, &mut ticks);
                self.sta(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored Acc at {:X}", addr));
            }
            0x8D => {
                // STA_ABSOLUTE
                let addr = self.get_absolute_adress(bus, &mut ticks);
                self.sta(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored Acc at {:X}", addr));
            }
            0x9D => {
                // STA_ABSOLUTE_X
                let addr = self.get_absolute_adress_x(bus, &mut ticks);
                self.sta(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored Acc at {:X}", addr));
            }
            0x99 => {
                // STA_ABSOLUTE_Y
                let addr = self.get_absolute_adress_y(bus, &mut ticks);
                self.sta(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored Acc at {:X}", addr));
            }
            0x81 => {
                // STA_INDIRECT_X
                let addr = self.get_indirect_indexing_adress(bus, &mut ticks);
                self.sta(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored Acc at {:X}", addr));
            }
            0x91 => {
                // STA_INDIRECT_Y
                let addr = self.get_indexing_indirect_adress(bus, &mut ticks);
                self.sta(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored Acc at {:X}", addr));
            }
            0x86 => {
                // STX_ZERO_PAGE
                let addr = self.get_zp_adress(bus, &mut ticks);
                self.stx(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored X at {:X}", addr));
            }
            0x96 => {
                // STX_ZERO_PAGE_Y
                let addr = self.get_zp_adress_y(bus, &mut ticks);
                self.stx(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored X at {:X}", addr));
            }
            0x8E => {
                // STX_ABSOLUTE
                let addr = self.get_absolute_adress(bus, &mut ticks);
                self.stx(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored X at {:X}", addr));
            }
            0x84 => {
                // STY_ZERO_PAGE
                let addr = self.get_zp_adress(bus, &mut ticks);
                self.sty(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored Y at {:X}", addr));
            }
            0x94 => {
                // STY_ZERO_PAGE_X
                let addr = self.get_zp_adress_x(bus, &mut ticks);
                self.sty(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored Y at {:X}", addr));
            }
            0x8C => {
                // STY_ABSOLUTE
                let addr = self.get_absolute_adress(bus, &mut ticks);
                self.sty(bus, addr, &mut ticks);
                self.config.logger.log(format!("Stored Y at {:X}", addr));
            }
            0xAA => {
                // TAX
                self.tax(&mut ticks);
                self.config.logger.log(format!("Tranfered A into X regiser"));
            }
            0xA8 => {
                // TAY
                self.tay(&mut ticks);
                self.config.logger.log(format!("Tranfered A into Y regiser"));
            }
            0xBA => {
                // TSX
                self.tsx(&mut ticks);
                self.config.logger.log(format!("Tranfered SP into X regiser"));
            }
            0x8A => {
                // TXA
                self.txa(&mut ticks);
                self.config.logger.log(format!("Tranfered X into A regiser"));
            }
            0x9A => {
                // TXS
                self.txs(&mut ticks);
                self.config.logger.log(format!("Tranfered X into S regiser"));
            }
            0x98 => {
                // TYA
                self.tya(&mut ticks);
                self.config.logger.log(format!("Tranfered Y into A regiser"));
            }
            0xEA => {
                // NOP
                ticks += 1;
                self.config.logger.log(format!("NOP"));
            }
            _ => {
                self.config.logger.log(format!("Tried to execute unknown command!"));
            }
        }
        ticks
    }
}
