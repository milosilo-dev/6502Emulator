use emulate6502::devices::bbcmicro::bbc_micro::BBCMicro;

fn main() {
    let mut system = BBCMicro::new();

    while system.tick() {}
}
