//! Constants used throughout the emulator.

pub mod cpu {
    use std::mem;
    use common::types::primative::*;

    pub const INSTRUCTION_SIZE: usize = mem::size_of::<udword>();
    pub const INSTRUCTION_COUNT: usize = 35;
    pub const CLOCK_SPEED: f64 = 500.0;
    pub const SPRITE_SIZE: usize = 5;
}

pub mod spu {
    pub const CLOCK_SPEED: f64 = 60.0;    
}

pub mod timer {
    pub const CLOCK_SPEED: f64 = 60.0;
}

pub mod input {
    pub const KEYS_COUNT: usize = 16;
}

pub mod gpu {
    pub const HORIZONTAL_RES: usize = 64;
    pub const VERTICAL_RES: usize = 32;
}