pub mod cpu;
pub mod spu;
pub mod timer;

use common::types::storage::memory::word_memory::WordMemory;
use resources::cpu::CPU;
use resources::spu::SPU;
use resources::timer::Timer;

pub struct Resources {
    pub memory: WordMemory,
    pub cpu: CPU,
    pub spu: SPU,
    pub timer: Timer,
}

unsafe impl Sync for Resources { }

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