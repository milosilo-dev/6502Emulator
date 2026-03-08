use std::ops::RangeInclusive;

pub enum TickReturn{
    SHUTDOWN,
    IRQ,
    NONE
}

pub trait Device {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
    fn tick(&mut self) -> TickReturn;
}

pub struct Bus {
    devices: Vec<(RangeInclusive<u16>, Box<dyn Device>)>,
}

impl Bus {
    pub fn default() -> Self {
        Self { devices: vec![] }
    }

    pub fn register(&mut self, range: RangeInclusive<u16>, device: Box<dyn Device>) {
        self.devices.push((range, device));
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        for (range, device) in &mut self.devices {
            if range.contains(&addr) {
                let offset = addr - *range.start();
                return device.read(offset);
            }
        }

        0
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        for (range, device) in &mut self.devices {
            if range.contains(&addr) {
                let offset = addr - *range.start();
                device.write(offset, value);
                return;
            }
        }
    }

    pub fn tick(&mut self) -> TickReturn{
        let mut ret = TickReturn::NONE;
        for (_, device) in &mut self.devices {
            match device.tick() {
                TickReturn::IRQ => {
                    ret = TickReturn::IRQ;
                }
                TickReturn::SHUTDOWN => {
                    return TickReturn::SHUTDOWN;
                }
                TickReturn::NONE => {}
            }
        }
        ret
    }
}
