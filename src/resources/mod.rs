use common::types::storage::memory::*;

mod cpu;
mod spu;
mod timer;

pub use resources::cpu::*;
pub use resources::spu::*;
pub use resources::timer::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Resources {
    memory: WordMemory,
    cpu: CPU,
    spu: SPU,
    timer: Timer,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            memory: WordMemory::new(0x1000),
            cpu: CPU::new(),
            spu: SPU::new(),
            timer: Timer::new(),
        }
    }
}