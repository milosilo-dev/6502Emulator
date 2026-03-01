#[cfg(test)]
mod klaus_test {
    use crate::bus::Bus;
    use crate::cpu::cpu::CPU;
    use crate::devices::mem::Mem;

    #[test]
    fn functional_test() {
        let mut cpu = CPU::default();
        let mut bus = Bus::default();
        let mem = Box::new(Mem::default());
        bus.register(0..=0xFFFF, mem);

        let loaded = cpu.load_rom(&mut bus, "test_roms/6502_functional_test.bin", 0x0000);
        println!("Loaded {loaded}");

        cpu.reset(&bus);
        cpu.pc = 0x0400;
        cpu.config.speed = 30.0; // run it faster because it take long
        cpu.run(&mut bus, 0x37CE);
    }
}