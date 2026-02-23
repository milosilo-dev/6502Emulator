#[cfg(test)]
mod tests {
    use crate::cpu::CPU;
    use crate::bus::Bus;
    use crate::devices::mem::Mem;

    fn init() -> (CPU, Bus) {
        let mut cpu = CPU::default();
        let mut bus = Bus::default();
        let mem = Box::new(Mem::default());
        bus.register(0..=0xFFFF, mem);

        bus.write(0xFFFC, 0x00);
        bus.write(0xFFFD, 0x00);
        cpu.reset(&bus);
        (cpu, bus)
    }

    #[test]
    fn lda_im() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x69);
        cpu.execute(&mut bus, 2);
        assert_eq!(cpu.read_acc(), 0x69);
    }

    #[test]
    fn lda_im_zero_flag() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate #0 -> sets Z flag
        bus.write(0x0001, 0x00);
        cpu.execute(&mut bus, 2);
        assert_eq!(cpu.read_acc(), 0x00);
    }

    #[test]
    fn lda_zp() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0042, 0xAB); // value in zero page
        bus.write(0x0000, 0xA5); // LDA zero page
        bus.write(0x0001, 0x42);
        cpu.execute(&mut bus, 3);
        assert_eq!(cpu.read_acc(), 0xAB);
    }

    #[test]
    fn lda_zp_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x05);
        bus.write(0x0002, 0xB5); // LDA zero page, X
        bus.write(0x0003, 0x10);
        bus.write(0x0015, 0x77); // 0x10 + 0x05 = 0x15
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0x77);
    }

    #[test]
    fn lda_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0xBE); // value at absolute address
        bus.write(0x0000, 0xAD); // LDA absolute
        bus.write(0x0001, 0x34);
        bus.write(0x0002, 0x12);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0xBE);
    }

    #[test]
    fn lda_absolute_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x03);
        bus.write(0x0002, 0xBD); // LDA absolute, X
        bus.write(0x0003, 0x00);
        bus.write(0x0004, 0x10);
        bus.write(0x1003, 0xCC); // 0x1000 + 0x03 = 0x1003
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0xCC);
    }

    #[test]
    fn lda_absolute_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0x02);
        bus.write(0x0002, 0xB9); // LDA absolute, Y
        bus.write(0x0003, 0x00);
        bus.write(0x0004, 0x20);
        bus.write(0x2002, 0xDD); // 0x2000 + 0x02 = 0x2002
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0xDD);
    }

    #[test]
    fn ldx_im() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x42);
        cpu.execute(&mut bus, 2);
        assert_eq!(cpu.read_x(), 0x42);
    }

    #[test]
    fn ldx_zp() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0050, 0x11);
        bus.write(0x0000, 0xA6); // LDX zero page
        bus.write(0x0001, 0x50);
        cpu.execute(&mut bus, 3);
        assert_eq!(cpu.read_x(), 0x11);
    }

    #[test]
    fn ldx_zp_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0x04);
        bus.write(0x0002, 0xB6); // LDX zero page, Y
        bus.write(0x0003, 0x10);
        bus.write(0x0014, 0x55); // 0x10 + 0x04 = 0x14
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_x(), 0x55);
    }

    #[test]
    fn ldx_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x3000, 0x99);
        bus.write(0x0000, 0xAE); // LDX absolute
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0x30);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_x(), 0x99);
    }

    #[test]
    fn ldx_absolute_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0x01);
        bus.write(0x0002, 0xBE); // LDX absolute, Y
        bus.write(0x0003, 0x00);
        bus.write(0x0004, 0x40);
        bus.write(0x4001, 0x88); // 0x4000 + 0x01 = 0x4001
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_x(), 0x88);
    }

    #[test]
    fn ldy_im() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0x33);
        cpu.execute(&mut bus, 2);
        assert_eq!(cpu.read_y(), 0x33);
    }

    #[test]
    fn ldy_zp() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0020, 0x7F);
        bus.write(0x0000, 0xA4); // LDY zero page
        bus.write(0x0001, 0x20);
        cpu.execute(&mut bus, 3);
        assert_eq!(cpu.read_y(), 0x7F);
    }

    #[test]
    fn ldy_zp_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x03);
        bus.write(0x0002, 0xB4); // LDY zero page, X
        bus.write(0x0003, 0x10);
        bus.write(0x0013, 0xAA); // 0x10 + 0x03 = 0x13
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_y(), 0xAA);
    }

    #[test]
    fn ldy_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x5000, 0x66);
        bus.write(0x0000, 0xAC); // LDY absolute
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0x50);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_y(), 0x66);
    }

    #[test]
    fn ldy_absolute_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x02);
        bus.write(0x0002, 0xBC); // LDY absolute, X
        bus.write(0x0003, 0x00);
        bus.write(0x0004, 0x60);
        bus.write(0x6002, 0x44); // 0x6000 + 0x02 = 0x6002
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_y(), 0x44);
    }

    #[test]
    fn adc_immediate_no_carry() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x10);
        bus.write(0x0002, 0x69); // ADC immediate
        bus.write(0x0003, 0x05);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0x15); // 0x10 + 0x05 = 0x15
    }

    #[test]
    fn adc_immediate_wraps() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x69); // ADC immediate
        bus.write(0x0003, 0x01);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0x00); // 0xFF + 0x01 wraps to 0x00
    }

    #[test]
    fn adc_zp() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0030, 0x20);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x10);
        bus.write(0x0002, 0x65); // ADC zero page
        bus.write(0x0003, 0x30);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_acc(), 0x30); // 0x10 + 0x20
    }

    #[test]
    fn adc_zp_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0015, 0x07);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x03);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x05);
        bus.write(0x0004, 0x75); // ADC zero page, X
        bus.write(0x0005, 0x10);
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0x0A); // 0x03 + 0x07
    }

    #[test]
    fn adc_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0x11);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x22);
        bus.write(0x0002, 0x6D); // ADC absolute
        bus.write(0x0003, 0x34);
        bus.write(0x0004, 0x12);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0x33); // 0x22 + 0x11
    }

    #[test]
    fn adc_absolute_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1003, 0x01);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x09);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x03);
        bus.write(0x0004, 0x7D); // ADC absolute, X
        bus.write(0x0005, 0x00);
        bus.write(0x0006, 0x10);
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0x0A); // 0x09 + 0x01
    }

    #[test]
    fn adc_absolute_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x2002, 0x08);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x02);
        bus.write(0x0002, 0xA0); // LDY immediate
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0x79); // ADC absolute, Y
        bus.write(0x0005, 0x00);
        bus.write(0x0006, 0x20);
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0x0A); // 0x02 + 0x08
    }

    #[test]
    fn and_immediate() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA
        bus.write(0x0001, 0x99);
        bus.write(0x0002, 0x29); // AND
        bus.write(0x0003, 0x88);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0x88);
    }

    #[test]
    fn and_zero_page() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0042, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x25); // AND zero page
        bus.write(0x0003, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_acc(), 0x0F); // 0xFF & 0x0F
    }

    #[test]
    fn and_zero_page_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0015, 0x3C);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x05);
        bus.write(0x0004, 0x35); // AND zero page, X
        bus.write(0x0005, 0x10); // 0x10 + 0x05 = 0x15
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0x3C); // 0xFF & 0x3C
    }

    #[test]
    fn and_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0xF0);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x2D); // AND absolute
        bus.write(0x0003, 0x34);
        bus.write(0x0004, 0x12);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0xF0); // 0xFF & 0xF0
    }

    #[test]
    fn and_absolute_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1003, 0xAA);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x03);
        bus.write(0x0004, 0x3D); // AND absolute, X
        bus.write(0x0005, 0x00);
        bus.write(0x0006, 0x10); // 0x1000 + 0x03 = 0x1003
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0xAA); // 0xFF & 0xAA
    }

    #[test]
    fn and_absolute_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x2002, 0x55);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xA0); // LDY immediate
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0x39); // AND absolute, Y
        bus.write(0x0005, 0x00);
        bus.write(0x0006, 0x20); // 0x2000 + 0x02 = 0x2002
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0x55); // 0xFF & 0x55
    }

    #[test]
    fn and_indirect_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0015, 0x00); // lo byte of target
        bus.write(0x0016, 0x30); // hi byte of target -> 0x3000
        bus.write(0x3000, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x05);
        bus.write(0x0004, 0x21); // AND indirect, X
        bus.write(0x0005, 0x10); // 0x10 + 0x05 = 0x15
        cpu.execute(&mut bus, 10);
        assert_eq!(cpu.read_acc(), 0x0F); // 0xFF & 0x0F
    }

    #[test]
    fn and_indirect_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0020, 0x00); // lo byte of base
        bus.write(0x0021, 0x30); // hi byte of base -> 0x3000
        bus.write(0x3002, 0x3C);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xA0); // LDY immediate
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0x31); // AND indirect, Y
        bus.write(0x0005, 0x20); // base = 0x3000, + Y(0x02) = 0x3002
        cpu.execute(&mut bus, 9);
        assert_eq!(cpu.read_acc(), 0x3C); // 0xFF & 0x3C
    }

    #[test]
    fn asl_accumulator() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA
        bus.write(0x0001, 0x50);
        bus.write(0x0002, 0x0A); // ASL
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0xA0); // 0x55 << 1
    }

    #[test]
    fn asl_zp() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0042, 0x04);
        bus.write(0x0000, 0x06); // ASL zero page
        bus.write(0x0001, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(bus.read(0x0042), 0x08); // 0x04 << 1, written back to 0x42
    }

    #[test]
    fn asl_zp_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0015, 0x10);
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x05);
        bus.write(0x0002, 0x16); // ASL zero page, X
        bus.write(0x0003, 0x10); // 0x10 + 0x05 = 0x15
        cpu.execute(&mut bus, 8);
        assert_eq!(bus.read(0x0015), 0x20); // 0x10 << 1, written back to 0x15
    }

    #[test]
    fn asl_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0x04);
        bus.write(0x0000, 0x0E); // ASL absolute
        bus.write(0x0001, 0x34);
        bus.write(0x0002, 0x12);
        cpu.execute(&mut bus, 6);
        // ASL shifts memory in place, so read back from the address
        assert_eq!(bus.read(0x1234), 0x08); // 0x04 << 1
    }

    #[test]
    fn asl_absolute_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1003, 0x10);
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x03);
        bus.write(0x0002, 0x1E); // ASL absolute, X
        bus.write(0x0003, 0x00);
        bus.write(0x0004, 0x10); // 0x1000 + 0x03 = 0x1003
        cpu.execute(&mut bus, 9);
        assert_eq!(bus.read(0x1003), 0x20); // 0x10 << 1
    }

    #[test]
    fn jmp_absolute() {
        let (mut cpu, mut bus) = init();
        // Jump to 0x0200, then immediately load A with 0x42
        bus.write(0x0000, 0x4C); // JMP absolute
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0x02);
        bus.write(0x0200, 0xA9); // LDA immediate at jump target
        bus.write(0x0201, 0x42);
        cpu.execute(&mut bus, 5); // 3 ticks for JMP + 2 for LDA
        assert_eq!(cpu.read_acc(), 0x42);
    }

    #[test]
    fn jmp_indirect() {
        let (mut cpu, mut bus) = init();
        // Pointer at 0x0300 points to 0x0200
        bus.write(0x0300, 0x00);
        bus.write(0x0301, 0x02);
        bus.write(0x0000, 0x6C); // JMP indirect
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0x03);
        bus.write(0x0200, 0xA9); // LDA immediate at final target
        bus.write(0x0201, 0x55);
        cpu.execute(&mut bus, 8); // 6 ticks for JMP indirect + 2 for LDA
        assert_eq!(cpu.read_acc(), 0x55);
    }

    #[test]
    fn bcc_branches_when_carry_clear() {
        let (mut cpu, mut bus) = init();
        // Carry is clear by default
        bus.write(0x0000, 0x90); // BCC
        bus.write(0x0001, 0x02); // offset +2 -> 0x0004
        bus.write(0x0004, 0xA9); // LDA immediate
        bus.write(0x0005, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_acc(), 0x42);
    }

    #[test]
    fn bcc_no_branch_when_carry_set() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x69); // ADC immediate - sets carry
        bus.write(0x0003, 0x01);
        bus.write(0x0004, 0x90); // BCC
        bus.write(0x0005, 0x02);
        bus.write(0x0006, 0xA9); // LDA immediate (sequential path)
        bus.write(0x0007, 0x01);
        bus.write(0x0008, 0xA9); // LDA immediate (branch target, should not run)
        bus.write(0x0009, 0xFF);
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0x01);
    }

    #[test]
    fn bcs_branches_when_carry_set() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x69); // ADC immediate - sets carry
        bus.write(0x0003, 0x01);
        bus.write(0x0004, 0xB0); // BCS
        bus.write(0x0005, 0x02); // offset +2 -> 0x0008
        bus.write(0x0008, 0xA9); // LDA immediate (branch target)
        bus.write(0x0009, 0x42);
        cpu.execute(&mut bus, 9);
        assert_eq!(cpu.read_acc(), 0x42);
    }

    #[test]
    fn bcs_no_branch_when_carry_clear() {
        let (mut cpu, mut bus) = init();
        // Carry clear by default
        bus.write(0x0000, 0xB0); // BCS
        bus.write(0x0001, 0x02);
        bus.write(0x0002, 0xA9); // LDA immediate (sequential path)
        bus.write(0x0003, 0x01);
        bus.write(0x0004, 0xA9); // LDA immediate (branch target, should not run)
        bus.write(0x0005, 0xFF);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0x01);
    }

    #[test]
    fn beq_branches_when_zero_set() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate #0 - sets zero flag
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0xF0); // BEQ
        bus.write(0x0003, 0x02); // offset +2 -> 0x0006
        bus.write(0x0006, 0xA9); // LDA immediate (branch target)
        bus.write(0x0007, 0x42);
        cpu.execute(&mut bus, 7);
        assert_eq!(cpu.read_acc(), 0x42);
    }

    #[test]
    fn beq_no_branch_when_zero_clear() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate - clears zero flag
        bus.write(0x0001, 0x01);
        bus.write(0x0002, 0xF0); // BEQ
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0xA9); // LDA immediate (sequential path)
        bus.write(0x0005, 0x02);
        bus.write(0x0006, 0xA9); // LDA immediate (branch target, should not run)
        bus.write(0x0007, 0xFF);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0x02);
    }

    #[test]
    fn bne_branches_when_zero_clear() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate - clears zero flag
        bus.write(0x0001, 0x01);
        bus.write(0x0002, 0xD0); // BNE
        bus.write(0x0003, 0x02); // offset +2 -> 0x0006
        bus.write(0x0006, 0xA9); // LDA immediate (branch target)
        bus.write(0x0007, 0x42);
        cpu.execute(&mut bus, 7);
        assert_eq!(cpu.read_acc(), 0x42);
    }

    #[test]
    fn bne_no_branch_when_zero_set() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate #0 - sets zero flag
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0xD0); // BNE
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0xA9); // LDA immediate (sequential path)
        bus.write(0x0005, 0x05);
        bus.write(0x0006, 0xA9); // LDA immediate (branch target, should not run)
        bus.write(0x0007, 0xFF);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0x05);
    }

    #[test]
    fn bmi_branches_when_negative_set() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate - sets negative flag (bit 7 set)
        bus.write(0x0001, 0x80);
        bus.write(0x0002, 0x30); // BMI
        bus.write(0x0003, 0x02); // offset +2 -> 0x0006
        bus.write(0x0006, 0xA9); // LDA immediate (branch target)
        bus.write(0x0007, 0x42);
        cpu.execute(&mut bus, 7);
        assert_eq!(cpu.read_acc(), 0x42);
    }

    #[test]
    fn bmi_no_branch_when_negative_clear() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate - clears negative flag
        bus.write(0x0001, 0x01);
        bus.write(0x0002, 0x30); // BMI
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0xA9); // LDA immediate (sequential path)
        bus.write(0x0005, 0x03);
        bus.write(0x0006, 0xA9); // LDA immediate (branch target, should not run)
        bus.write(0x0007, 0xFF);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0x03);
    }

    #[test]
    fn bpl_branches_when_negative_clear() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate - clears negative flag
        bus.write(0x0001, 0x01);
        bus.write(0x0002, 0x10); // BPL
        bus.write(0x0003, 0x02); // offset +2 -> 0x0006
        bus.write(0x0006, 0xA9); // LDA immediate (branch target)
        bus.write(0x0007, 0x42);
        cpu.execute(&mut bus, 7);
        assert_eq!(cpu.read_acc(), 0x42);
    }

    #[test]
    fn bpl_no_branch_when_negative_set() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate - sets negative flag
        bus.write(0x0001, 0x80);
        bus.write(0x0002, 0x10); // BPL
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0xA9); // LDA immediate (sequential path)
        bus.write(0x0005, 0x03);
        bus.write(0x0006, 0xA9); // LDA immediate (branch target, should not run)
        bus.write(0x0007, 0xFF);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0x03);
    }

    #[test]
    fn bvc_branches_when_overflow_clear() {
        let (mut cpu, mut bus) = init();
        // Overflow is clear by default
        bus.write(0x0000, 0x50); // BVC
        bus.write(0x0001, 0x02); // offset +2 -> 0x0004
        bus.write(0x0004, 0xA9); // LDA immediate (branch target)
        bus.write(0x0005, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_acc(), 0x42);
    }

    #[test]
    fn bvc_no_branch_when_overflow_set() {
        let (mut cpu, mut bus) = init();
        // 0x50 + 0x50 = 0xA0 - positive + positive = negative, sets overflow
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x50);
        bus.write(0x0002, 0x69); // ADC immediate
        bus.write(0x0003, 0x50);
        bus.write(0x0004, 0x50); // BVC
        bus.write(0x0005, 0x02);
        bus.write(0x0006, 0xA9); // LDA immediate (sequential path)
        bus.write(0x0007, 0x01);
        bus.write(0x0008, 0xA9); // LDA immediate (branch target, should not run)
        bus.write(0x0009, 0xFF);
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0x01);
    }

    #[test]
    fn bvs_branches_when_overflow_set() {
        let (mut cpu, mut bus) = init();
        // 0x50 + 0x50 = 0xA0 - positive + positive = negative, sets overflow
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x50);
        bus.write(0x0002, 0x69); // ADC immediate
        bus.write(0x0003, 0x50);
        bus.write(0x0004, 0x70); // BVS
        bus.write(0x0005, 0x02); // offset +2 -> 0x0008
        bus.write(0x0008, 0xA9); // LDA immediate (branch target)
        bus.write(0x0009, 0x42);
        cpu.execute(&mut bus, 9);
        assert_eq!(cpu.read_acc(), 0x42);
    }

    #[test]
    fn bvs_no_branch_when_overflow_clear() {
        let (mut cpu, mut bus) = init();
        // Overflow is clear by default
        bus.write(0x0000, 0x70); // BVS
        bus.write(0x0001, 0x02);
        bus.write(0x0002, 0xA9); // LDA immediate (sequential path)
        bus.write(0x0003, 0x01);
        bus.write(0x0004, 0xA9); // LDA immediate (branch target, should not run)
        bus.write(0x0005, 0xFF);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0x01);
    }

    #[test]
    fn nop() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xEA); // NOP
        bus.write(0x0001, 0xA9); // LDA immediate
        bus.write(0x0002, 0x01);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0x01);
    }

    #[test]
    fn bit_zp_zero_flag_set() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0042, 0x00); // M = 0x00, A & M = 0 -> Z set
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x24); // BIT zero page
        bus.write(0x0003, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_acc(), 0xFF); // accumulator unchanged
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set
    }

    #[test]
    fn bit_zp_zero_flag_clear() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0042, 0xFF); // M = 0xFF, A & M = 0xFF -> Z clear
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x24); // BIT zero page
        bus.write(0x0003, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_acc(), 0xFF); // accumulator unchanged
        assert_eq!(cpu.read_status() & 0b00000010, 0); // Z clear
    }

    #[test]
    fn bit_zp_negative_flag_from_memory() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0042, 0x80); // M bit 7 set -> N set
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x24); // BIT zero page
        bus.write(0x0003, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_status() & 0b10000000, 0b10000000); // N set
    }

    #[test]
    fn bit_zp_overflow_flag_from_memory() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0042, 0x40); // M bit 6 set -> V set
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x24); // BIT zero page
        bus.write(0x0003, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_status() & 0b01000000, 0b01000000); // V set
    }

    #[test]
    fn bit_zp_overflow_and_negative_clear() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0042, 0x3F); // M bits 6 and 7 clear -> V and N clear
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x24); // BIT zero page
        bus.write(0x0003, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_status() & 0b11000000, 0); // V and N clear
    }

    #[test]
    fn bit_absolute_zero_flag_set() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0x00); // M = 0x00, A & M = 0 -> Z set
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x2C); // BIT absolute
        bus.write(0x0003, 0x34);
        bus.write(0x0004, 0x12);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0xFF); // accumulator unchanged
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set
    }

    #[test]
    fn bit_absolute_negative_and_overflow_from_memory() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0xC0); // M = 0xC0, bits 6 and 7 set -> V and N set
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x2C); // BIT absolute
        bus.write(0x0003, 0x34);
        bus.write(0x0004, 0x12);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_status() & 0b11000000, 0b11000000); // V and N set
    }

    #[test]
    fn brk_jumps_to_irq_vector() {
        let (mut cpu, mut bus) = init();
        // Set IRQ vector at 0xFFFE/0xFFFF to point to 0x0200
        bus.write(0xFFFE, 0x00);
        bus.write(0xFFFF, 0x02);
        bus.write(0x0000, 0x00); // BRK
        // Place an LDA at the IRQ handler address
        bus.write(0x0200, 0xA9);
        bus.write(0x0201, 0x42);
        cpu.execute(&mut bus, 9); // 7 ticks for BRK + 2 for LDA
        assert_eq!(cpu.read_acc(), 0x42);
    }

    #[test]
    fn brk_pushes_pc_to_stack() {
        let (mut cpu, mut bus) = init();
        bus.write(0xFFFE, 0x00);
        bus.write(0xFFFF, 0x02);
        bus.write(0x0000, 0x00); // BRK at 0x0000, so PC+2 = 0x0002
        bus.write(0x0200, 0xEA); // NOP at handler so execute completes
        cpu.execute(&mut bus, 9);
        // Stack grows down from 0x01FF, SP starts at 0xFD
        // BRK pushes hi(PC+2) to 0x01FD, lo(PC+2) to 0x01FC, status to 0x01FB
        assert_eq!(bus.read(0x01FD), 0x00); // hi byte of 0x0002
        assert_eq!(bus.read(0x01FC), 0x02); // lo byte of 0x0002
    }

    #[test]
    fn brk_pushes_status_with_b_flag() {
        let (mut cpu, mut bus) = init();
        bus.write(0xFFFE, 0x00);
        bus.write(0xFFFF, 0x02);
        bus.write(0x0000, 0x00); // BRK
        bus.write(0x0200, 0xEA); // NOP at handler
        cpu.execute(&mut bus, 9);
        // Status pushed to 0x01FB with B flag (bit 4) set
        let pushed_status = bus.read(0x01FB);
        assert_eq!(pushed_status & 0b00010000, 0b00010000); // B flag set
    }

    #[test]
    fn clc_clears_carry() {
        let (mut cpu, mut bus) = init();
        // Set carry first by overflowing ADC
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x69); // ADC immediate - sets carry
        bus.write(0x0003, 0x01);
        bus.write(0x0004, 0x18); // CLC
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_status() & 0b00000001, 0); // C clear
    }

    #[test]
    fn cld_clears_decimal() {
        let (mut cpu, mut bus) = init();
        // Set decimal flag first with SED (0xF8)
        bus.write(0x0000, 0xF8); // SED
        bus.write(0x0001, 0xD8); // CLD
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_status() & 0b00001000, 0); // D clear
    }

    #[test]
    fn cli_clears_interrupt_disable() {
        let (mut cpu, mut bus) = init();
        // Set interrupt disable first with SEI (0x78)
        bus.write(0x0000, 0x78); // SEI
        bus.write(0x0001, 0x58); // CLI
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_status() & 0b00000100, 0); // I clear
    }

    #[test]
    fn clv_clears_overflow() {
        let (mut cpu, mut bus) = init();
        // Set overflow by adding two positive numbers that produce a negative result
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x50);
        bus.write(0x0002, 0x69); // ADC immediate - sets overflow
        bus.write(0x0003, 0x50);
        bus.write(0x0004, 0xB8); // CLV
        cpu.execute(&mut bus, 7);
        assert_eq!(cpu.read_status() & 0b01000000, 0); // V clear
    }

    #[test]
    fn cpx_immediate_equal() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xE0); // CPX immediate
        bus.write(0x0003, 0x42);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set (X >= M)
        assert_eq!(cpu.read_status() & 0b10000000, 0);           // N clear
        assert_eq!(cpu.read_x(), 0x42);                          // X unchanged
    }

    #[test]
    fn cpx_immediate_greater() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x50);
        bus.write(0x0002, 0xE0); // CPX immediate
        bus.write(0x0003, 0x30);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_status() & 0b00000010, 0);           // Z clear
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set (X >= M)
        assert_eq!(cpu.read_status() & 0b10000000, 0);           // N clear
    }

    #[test]
    fn cpx_immediate_less() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x10);
        bus.write(0x0002, 0xE0); // CPX immediate
        bus.write(0x0003, 0x20);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_status() & 0b00000010, 0);           // Z clear
        assert_eq!(cpu.read_status() & 0b00000001, 0);           // C clear (X < M)
        assert_eq!(cpu.read_status() & 0b10000000, 0b10000000); // N set
    }

    #[test]
    fn cpx_zero_page() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0050, 0x42);
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xE4); // CPX zero page
        bus.write(0x0003, 0x50);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set (equal)
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set
    }

    #[test]
    fn cpx_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0x42);
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xEC); // CPX absolute
        bus.write(0x0003, 0x34);
        bus.write(0x0004, 0x12);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set (equal)
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set
    }

    #[test]
    fn cpy_immediate_equal() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xC0); // CPY immediate
        bus.write(0x0003, 0x42);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set (Y >= M)
        assert_eq!(cpu.read_status() & 0b10000000, 0);           // N clear
        assert_eq!(cpu.read_y(), 0x42);                          // Y unchanged
    }

    #[test]
    fn cpy_immediate_greater() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0x50);
        bus.write(0x0002, 0xC0); // CPY immediate
        bus.write(0x0003, 0x30);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_status() & 0b00000010, 0);           // Z clear
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set (Y >= M)
        assert_eq!(cpu.read_status() & 0b10000000, 0);           // N clear
    }

    #[test]
    fn cpy_immediate_less() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0x10);
        bus.write(0x0002, 0xC0); // CPY immediate
        bus.write(0x0003, 0x20);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_status() & 0b00000010, 0);           // Z clear
        assert_eq!(cpu.read_status() & 0b00000001, 0);           // C clear (Y < M)
        assert_eq!(cpu.read_status() & 0b10000000, 0b10000000); // N set
    }

    #[test]
    fn cpy_zero_page() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0050, 0x42);
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xC4); // CPY zero page
        bus.write(0x0003, 0x50);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set (equal)
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set
    }

    #[test]
    fn cpy_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0x42);
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xCC); // CPY absolute
        bus.write(0x0003, 0x34);
        bus.write(0x0004, 0x12);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set (equal)
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set
    }

    #[test]
    fn cmp_immediate_equal() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xC9); // CMP immediate
        bus.write(0x0003, 0x42);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set (A >= M)
        assert_eq!(cpu.read_status() & 0b10000000, 0);           // N clear
        assert_eq!(cpu.read_acc(), 0x42);                        // A unchanged
    }

    #[test]
    fn cmp_immediate_greater() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x50);
        bus.write(0x0002, 0xC9); // CMP immediate
        bus.write(0x0003, 0x30);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_status() & 0b00000010, 0);           // Z clear
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set (A >= M)
        assert_eq!(cpu.read_status() & 0b10000000, 0);           // N clear
    }

    #[test]
    fn cmp_immediate_less() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x10);
        bus.write(0x0002, 0xC9); // CMP immediate
        bus.write(0x0003, 0x20);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_status() & 0b00000010, 0);           // Z clear
        assert_eq!(cpu.read_status() & 0b00000001, 0);           // C clear (A < M)
        assert_eq!(cpu.read_status() & 0b10000000, 0b10000000); // N set
    }

    #[test]
    fn cmp_zero_page() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0050, 0x42);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xC5); // CMP zero page
        bus.write(0x0003, 0x50);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set (equal)
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set
    }

    #[test]
    fn cmp_zero_page_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0015, 0x42);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x05);
        bus.write(0x0004, 0xD5); // CMP zero page, X
        bus.write(0x0005, 0x10); // 0x10 + 0x05 = 0x15
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set (equal)
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set
    }

    #[test]
    fn cmp_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0x42);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xCD); // CMP absolute
        bus.write(0x0003, 0x34);
        bus.write(0x0004, 0x12);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set (equal)
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set
    }

    #[test]
    fn cmp_absolute_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1003, 0x42);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x03);
        bus.write(0x0004, 0xDD); // CMP absolute, X
        bus.write(0x0005, 0x00);
        bus.write(0x0006, 0x10); // 0x1000 + 0x03 = 0x1003
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set (equal)
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set
    }

    #[test]
    fn cmp_absolute_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1002, 0x42);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xA0); // LDY immediate
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0xD9); // CMP absolute, Y
        bus.write(0x0005, 0x00);
        bus.write(0x0006, 0x10); // 0x1000 + 0x02 = 0x1002
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set (equal)
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set
    }

    #[test]
    fn cmp_indirect_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0015, 0x00); // lo byte of target
        bus.write(0x0016, 0x30); // hi byte of target -> 0x3000
        bus.write(0x3000, 0x42);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x05);
        bus.write(0x0004, 0xC1); // CMP indirect, X
        bus.write(0x0005, 0x10); // 0x10 + 0x05 = 0x15
        cpu.execute(&mut bus, 10);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set (equal)
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set
    }

    #[test]
    fn cmp_indirect_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0020, 0x00); // lo byte of base
        bus.write(0x0021, 0x30); // hi byte of base -> 0x3000
        bus.write(0x3002, 0x42);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0xA0); // LDY immediate
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0xD1); // CMP indirect, Y
        bus.write(0x0005, 0x20); // base = 0x3000, + Y(0x02) = 0x3002
        cpu.execute(&mut bus, 9);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set (equal)
        assert_eq!(cpu.read_status() & 0b00000001, 0b00000001); // C set
    }

    #[test]
    fn dec_zp() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xC6);
        bus.write(0x0001, 0x55);
        bus.write(0x0055, 0x01);
        cpu.execute(&mut bus, 5);
        assert_eq!(bus.read(0x0055), 0x00);
    }
    
    #[test]
    fn dec_zp_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2);
        bus.write(0x0001, 0x05);
        bus.write(0x0002, 0xD6);
        bus.write(0x0003, 0x50);
        bus.write(0x0055, 0x01);
        cpu.execute(&mut bus, 5);
        assert_eq!(bus.read(0x0055), 0x00);
    }

    #[test]
    fn dec_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xCE);
        bus.write(0x0001, 0x55);
        bus.write(0x0002, 0x00);
        bus.write(0x0055, 0x01);
        cpu.execute(&mut bus, 5);
        assert_eq!(bus.read(0x0055), 0x00);
    }

    #[test]
    fn dec_absolute_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2);
        bus.write(0x0001, 0x05);
        bus.write(0x0002, 0xDE);
        bus.write(0x0003, 0x50);
        bus.write(0x0004, 0x00);
        bus.write(0x0055, 0x01);
        cpu.execute(&mut bus, 5);
        assert_eq!(bus.read(0x0055), 0x00);
    }

    #[test]
    fn decx() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2);
        bus.write(0x0001, 0x02);
        bus.write(0x0002, 0xCA);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_x(), 0x01);
    }

    #[test]
    fn decy() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA0);
        bus.write(0x0001, 0x02);
        bus.write(0x0002, 0x88);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_y(), 0x01);
    }

    #[test]
    fn eor_immediate() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x49); // EOR immediate
        bus.write(0x0003, 0x0F);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0xF0); // 0xFF ^ 0x0F
    }

    #[test]
    fn eor_immediate_zero_flag() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x42);
        bus.write(0x0002, 0x49); // EOR immediate - same value sets Z
        bus.write(0x0003, 0x42);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0x00);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set
    }

    #[test]
    fn eor_immediate_negative_flag() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x0F);
        bus.write(0x0002, 0x49); // EOR immediate - result has bit 7 set
        bus.write(0x0003, 0xF0);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0xFF); // 0x0F ^ 0xF0
        assert_eq!(cpu.read_status() & 0b10000000, 0b10000000); // N set
    }

    #[test]
    fn eor_zero_page() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0050, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x45); // EOR zero page
        bus.write(0x0003, 0x50);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_acc(), 0xF0); // 0xFF ^ 0x0F
    }

    #[test]
    fn eor_zero_page_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0015, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x05);
        bus.write(0x0004, 0x55); // EOR zero page, X
        bus.write(0x0005, 0x10); // 0x10 + 0x05 = 0x15
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0xF0); // 0xFF ^ 0x0F
    }

    #[test]
    fn eor_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0x4D); // EOR absolute
        bus.write(0x0003, 0x34);
        bus.write(0x0004, 0x12);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0xF0); // 0xFF ^ 0x0F
    }

    #[test]
    fn eor_absolute_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1003, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x03);
        bus.write(0x0004, 0x5D); // EOR absolute, X
        bus.write(0x0005, 0x00);
        bus.write(0x0006, 0x10); // 0x1000 + 0x03 = 0x1003
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0xF0); // 0xFF ^ 0x0F
    }

    #[test]
    fn eor_absolute_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1002, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xA0); // LDY immediate
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0x59); // EOR absolute, Y
        bus.write(0x0005, 0x00);
        bus.write(0x0006, 0x10); // 0x1000 + 0x02 = 0x1002
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0xF0); // 0xFF ^ 0x0F
    }

    #[test]
    fn eor_indirect_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0015, 0x00); // lo byte of target
        bus.write(0x0016, 0x30); // hi byte of target -> 0x3000
        bus.write(0x3000, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x05);
        bus.write(0x0004, 0x41); // EOR indirect, X
        bus.write(0x0005, 0x10); // 0x10 + 0x05 = 0x15
        cpu.execute(&mut bus, 10);
        assert_eq!(cpu.read_acc(), 0xF0); // 0xFF ^ 0x0F
    }

    #[test]
    fn eor_indirect_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0020, 0x00); // lo byte of base
        bus.write(0x0021, 0x30); // hi byte of base -> 0x3000
        bus.write(0x3002, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xA0); // LDY immediate
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0x51); // EOR indirect, Y
        bus.write(0x0005, 0x20); // base = 0x3000, + Y(0x02) = 0x3002
        cpu.execute(&mut bus, 9);
        assert_eq!(cpu.read_acc(), 0xF0); // 0xFF ^ 0x0F
    }
    #[test]
    fn inc_zero_page() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0042, 0x09);
        bus.write(0x0000, 0xE6); // INC zero page
        bus.write(0x0001, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(bus.read(0x0042), 0x0A); // 0x09 + 1
    }

    #[test]
    fn inc_zero_page_wraps() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0042, 0xFF);
        bus.write(0x0000, 0xE6); // INC zero page
        bus.write(0x0001, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(bus.read(0x0042), 0x00); // 0xFF wraps to 0x00
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set
    }

    #[test]
    fn inc_zero_page_negative_flag() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0042, 0x7F);
        bus.write(0x0000, 0xE6); // INC zero page
        bus.write(0x0001, 0x42);
        cpu.execute(&mut bus, 5);
        assert_eq!(bus.read(0x0042), 0x80); // 0x7F + 1 = 0x80, bit 7 set
        assert_eq!(cpu.read_status() & 0b10000000, 0b10000000); // N set
    }

    #[test]
    fn inc_zero_page_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0015, 0x09);
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x05);
        bus.write(0x0002, 0xF6); // INC zero page, X
        bus.write(0x0003, 0x10); // 0x10 + 0x05 = 0x15
        cpu.execute(&mut bus, 6);
        assert_eq!(bus.read(0x0015), 0x0A); // 0x09 + 1
    }

    #[test]
    fn inc_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0x09);
        bus.write(0x0000, 0xEE); // INC absolute
        bus.write(0x0001, 0x34);
        bus.write(0x0002, 0x12);
        cpu.execute(&mut bus, 6);
        assert_eq!(bus.read(0x1234), 0x0A); // 0x09 + 1
    }

    #[test]
    fn inc_absolute_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1003, 0x09);
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x03);
        bus.write(0x0002, 0xFE); // INC absolute, X
        bus.write(0x0003, 0x00);
        bus.write(0x0004, 0x10); // 0x1000 + 0x03 = 0x1003
        cpu.execute(&mut bus, 9);
        assert_eq!(bus.read(0x1003), 0x0A); // 0x09 + 1
    }

    #[test]
    fn inx_basic() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x09);
        bus.write(0x0002, 0xE8); // INX
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_x(), 0x0A); // 0x09 + 1
    }

    #[test]
    fn inx_wraps() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xE8); // INX
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_x(), 0x00); // 0xFF wraps to 0x00
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set
    }

    #[test]
    fn inx_negative_flag() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA2); // LDX immediate
        bus.write(0x0001, 0x7F);
        bus.write(0x0002, 0xE8); // INX
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_x(), 0x80); // 0x7F + 1 = 0x80, bit 7 set
        assert_eq!(cpu.read_status() & 0b10000000, 0b10000000); // N set
    }

    #[test]
    fn iny_basic() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0x09);
        bus.write(0x0002, 0xC8); // INY
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_y(), 0x0A); // 0x09 + 1
    }

    #[test]
    fn iny_wraps() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xC8); // INY
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_y(), 0x00); // 0xFF wraps to 0x00
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set
    }

    #[test]
    fn iny_negative_flag() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA0); // LDY immediate
        bus.write(0x0001, 0x7F);
        bus.write(0x0002, 0xC8); // INY
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_y(), 0x80); // 0x7F + 1 = 0x80, bit 7 set
        assert_eq!(cpu.read_status() & 0b10000000, 0b10000000); // N set
    }

    #[test]
    fn jsr_jumps_to_address() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0x20); // JSR
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0x02); // target = 0x0200
        bus.write(0x0200, 0xA9); // LDA immediate at subroutine
        bus.write(0x0201, 0x42);
        cpu.execute(&mut bus, 8); // 6 ticks for JSR + 2 for LDA
        assert_eq!(cpu.read_acc(), 0x42);
    }

    #[test]
    fn jsr_pushes_return_address_hi() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0x20); // JSR at 0x0000
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0x02); // target = 0x0200
        bus.write(0x0200, 0xEA); // NOP at subroutine
        cpu.execute(&mut bus, 8);
        // JSR pushes PC-1 = 0x0002 (last byte of JSR instruction)
        // hi byte pushed to 0x01FD (SP starts at 0xFD)
        assert_eq!(bus.read(0x01FD), 0x00); // hi byte of 0x0002
    }

    #[test]
    fn jsr_pushes_return_address_lo() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0x20); // JSR at 0x0000
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0x02); // target = 0x0200
        bus.write(0x0200, 0xEA); // NOP at subroutine
        cpu.execute(&mut bus, 8);
        // lo byte pushed to 0x01FC
        assert_eq!(bus.read(0x01FC), 0x02); // lo byte of 0x0002
    }

    #[test]
    fn jsr_decrements_sp_by_two() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0x20); // JSR
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0x02);
        bus.write(0x0200, 0xEA); // NOP at subroutine
        cpu.execute(&mut bus, 8);
        // SP starts at 0xFD, two pushes -> 0xFB
        assert_eq!(cpu.read_sp(), 0xFB);
    }

    #[test]
    fn jsr_nested() {
        let (mut cpu, mut bus) = init();
        // First JSR at 0x0000 -> 0x0200
        bus.write(0x0000, 0x20);
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0x02);
        // Second JSR at 0x0200 -> 0x0300
        bus.write(0x0200, 0x20);
        bus.write(0x0201, 0x00);
        bus.write(0x0202, 0x03);
        // LDA at final subroutine
        bus.write(0x0300, 0xA9);
        bus.write(0x0301, 0x42);
        cpu.execute(&mut bus, 14); // 6 + 6 + 2
        assert_eq!(cpu.read_acc(), 0x42);
        // SP should be decremented by 4 total (two JSRs)
        assert_eq!(cpu.read_sp(), 0xF9);
    }

    #[test]
    fn ora_immediate() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xF0);
        bus.write(0x0002, 0x09); // ORA immediate
        bus.write(0x0003, 0x0F);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0xFF); // 0xF0 | 0x0F
    }

    #[test]
    fn ora_immediate_zero_flag() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0x09); // ORA immediate
        bus.write(0x0003, 0x00);
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0x00);
        assert_eq!(cpu.read_status() & 0b00000010, 0b00000010); // Z set
    }

    #[test]
    fn ora_immediate_negative_flag() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0x00);
        bus.write(0x0002, 0x09); // ORA immediate
        bus.write(0x0003, 0x80); // bit 7 set -> N set
        cpu.execute(&mut bus, 4);
        assert_eq!(cpu.read_acc(), 0x80);
        assert_eq!(cpu.read_status() & 0b10000000, 0b10000000); // N set
    }

    #[test]
    fn ora_zero_page() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0050, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xF0);
        bus.write(0x0002, 0x05); // ORA zero page
        bus.write(0x0003, 0x50);
        cpu.execute(&mut bus, 5);
        assert_eq!(cpu.read_acc(), 0xFF); // 0xF0 | 0x0F
    }

    #[test]
    fn ora_zero_page_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0015, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xF0);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x05);
        bus.write(0x0004, 0x15); // ORA zero page, X
        bus.write(0x0005, 0x10); // 0x10 + 0x05 = 0x15
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0xFF); // 0xF0 | 0x0F
    }

    #[test]
    fn ora_absolute() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1234, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xF0);
        bus.write(0x0002, 0x0D); // ORA absolute
        bus.write(0x0003, 0x34);
        bus.write(0x0004, 0x12);
        cpu.execute(&mut bus, 6);
        assert_eq!(cpu.read_acc(), 0xFF); // 0xF0 | 0x0F
    }

    #[test]
    fn ora_absolute_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1003, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xF0);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x03);
        bus.write(0x0004, 0x1D); // ORA absolute, X
        bus.write(0x0005, 0x00);
        bus.write(0x0006, 0x10); // 0x1000 + 0x03 = 0x1003
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0xFF); // 0xF0 | 0x0F
    }

    #[test]
    fn ora_absolute_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x1002, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xF0);
        bus.write(0x0002, 0xA0); // LDY immediate
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0x19); // ORA absolute, Y
        bus.write(0x0005, 0x00);
        bus.write(0x0006, 0x10); // 0x1000 + 0x02 = 0x1002
        cpu.execute(&mut bus, 8);
        assert_eq!(cpu.read_acc(), 0xFF); // 0xF0 | 0x0F
    }

    #[test]
    fn ora_indirect_x() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0015, 0x00); // lo byte of target
        bus.write(0x0016, 0x30); // hi byte of target -> 0x3000
        bus.write(0x3000, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xF0);
        bus.write(0x0002, 0xA2); // LDX immediate
        bus.write(0x0003, 0x05);
        bus.write(0x0004, 0x01); // ORA indirect, X
        bus.write(0x0005, 0x10); // 0x10 + 0x05 = 0x15
        cpu.execute(&mut bus, 10);
        assert_eq!(cpu.read_acc(), 0xFF); // 0xF0 | 0x0F
    }

    #[test]
    fn ora_indirect_y() {
        let (mut cpu, mut bus) = init();
        bus.write(0x0020, 0x00); // lo byte of base
        bus.write(0x0021, 0x30); // hi byte of base -> 0x3000
        bus.write(0x3002, 0x0F);
        bus.write(0x0000, 0xA9); // LDA immediate
        bus.write(0x0001, 0xF0);
        bus.write(0x0002, 0xA0); // LDY immediate
        bus.write(0x0003, 0x02);
        bus.write(0x0004, 0x11); // ORA indirect, Y
        bus.write(0x0005, 0x20); // base = 0x3000, + Y(0x02) = 0x3002
        cpu.execute(&mut bus, 9);
        assert_eq!(cpu.read_acc(), 0xFF); // 0xF0 | 0x0F
    }
}