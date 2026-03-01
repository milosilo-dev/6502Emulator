use crate::platform::logging::{Logger, NoLog, Stdout};

pub struct CpuConfig {
    pub emulate_indirect_jmp_bug: bool,
    pub logger: Box<dyn Logger>,
    pub speed: f64,
}

impl CpuConfig{
    pub fn default() -> Self{
        Self {
            emulate_indirect_jmp_bug: false,
            logger: Box::new(Stdout{}),
            speed: 1.0,
        }
    }
}