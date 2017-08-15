//! Constants used throughout the emulator.

pub mod cpu {
    pub const INSTRUCTION_COUNT: usize = 35;
    pub const CLOCK_SPEED: f64 = 500.0;
}

pub mod spu {
    pub const CLOCK_SPEED: f64 = 60.0;    
}

pub mod timer {
    pub const CLOCK_SPEED: f64 = 60.0;
}