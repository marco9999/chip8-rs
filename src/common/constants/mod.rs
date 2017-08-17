//! Constants used throughout the emulator.

pub mod cpu {
    use std::mem;
    use common::types::storage::udword;

    pub const INSTRUCTION_SIZE: usize = mem::size_of::<udword>();
    pub const INSTRUCTION_COUNT: usize = 35;
    pub const CLOCK_SPEED: f64 = 500.0;
}

pub mod spu {
    pub const CLOCK_SPEED: f64 = 60.0;    
}

pub mod timer {
    pub const CLOCK_SPEED: f64 = 60.0;
}