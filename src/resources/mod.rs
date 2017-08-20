pub mod cpu;
pub mod spu;
pub mod timer;
pub mod input;
pub mod gpu;

use common::types::storage::memory::word_memory::WordMemory;
use resources::cpu::CPU;
use resources::spu::SPU;
use resources::timer::Timer;
use resources::input::Input;
use resources::gpu::GPU;

pub struct Resources {
    pub memory: WordMemory,
    pub cpu: CPU,
    pub spu: SPU,
    pub timer: Timer,
    pub input: Input,
    pub gpu: GPU,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            memory: WordMemory::new(0x1000),
            cpu: CPU::new(),
            spu: SPU::new(),
            timer: Timer::new(),
            input: Input::new(),
            gpu: GPU::new(),
        }
    }
}