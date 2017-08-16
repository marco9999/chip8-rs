pub mod cpu;
pub mod spu;
pub mod timer;

use common::types::storage::memory::word_memory::WordMemory;
use resources::cpu::CPU;
use resources::spu::SPU;
use resources::timer::Timer;

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