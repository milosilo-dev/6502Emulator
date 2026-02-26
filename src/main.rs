use emulate6502::cpu::cpu::CPU;
use emulate6502::bus::Bus;
use emulate6502::devices::mem::Mem;

fn main() {
    let mut cpu = CPU::default();
    let mut bus = Bus::default();
    let mem = Box::new(Mem::default());
    bus.register(0..=0xFFFF, mem);

    bus.write(0xFFFC, 0x00);
    bus.write(0xFFFD, 0x00);

    bus.write(0x0000, 0x69);
    bus.write(0x0001, 0x45);
    bus.write(0x0002, 0x69);
    bus.write(0x0003, 0x45);

    cpu.reset(&bus);
    cpu.execute(&mut bus, 10);
}
