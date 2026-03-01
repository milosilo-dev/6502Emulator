use emulate6502::bus::Bus;
use emulate6502::cpu::cpu::CPU;
use emulate6502::devices::mem::Mem;

fn main() {
    let mut cpu = CPU::default();
    let mut bus = Bus::default();
    let mem = Box::new(Mem::default());
    bus.register(0..=0xFFFF, mem);

    let loaded = cpu.load_rom(&mut bus, "test_roms/6502_functional_test.bin", 0x0000);
    println!("Loaded {loaded}");

    cpu.reset(&bus);
    cpu.pc = 0x0400;
    cpu.run(&mut bus);
}
