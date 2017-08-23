pub mod cpu;
pub mod spu;
pub mod timer;

use common::types::storage::memory::word_memory::WordMemory;
use resources::cpu::Cpu;
use resources::spu::Spu;
use resources::timer::Timer;

pub struct Resources {
    pub memory: WordMemory,
    pub cpu: Cpu,
    pub spu: Spu,
    pub timer: Timer,
}

unsafe impl Sync for Resources { }

impl Resources {
    pub fn new() -> Resources {
        Resources {
            memory: WordMemory::new(0x1000),
            cpu: Cpu::new(),
            spu: Spu::new(),
            timer: Timer::new(),
        }
    }
}