use common::types::storage::{udword, uptr};

#[derive(Debug)]
pub struct Instruction {
    value: udword,
}

impl Instruction {
    pub fn new(value: udword) -> Instruction {
        Instruction {
            value,
        }
    }

    /// Returns the upper 4-bits of an instruction.
    pub fn high_nibble(&self) -> u8 {
        ((self.value & 0xF000) >> 24) as u8
    }

    /// Returns the lower 4-bits of an instruction.
    pub fn low_nibble(&self) -> u8 {
        (self.value & 0xF) as u8
    }

    /// Returns the lower 12-bits of an instruction.
    pub fn address(&self) -> uptr {
        (self.value & 0xFFF) as uptr
    }

    /// Returns the 'x' register parameter (bits 8-11).
    pub fn x_register(&self) -> u8 {
        ((self.value & 0xF00) >> 8) as u8
    }

    /// Returns the 'y' register parameter (bits 4-7).
    pub fn y_register(&self) -> u8 {
        ((self.value & 0xF0) >> 4) as u8
    }

    // Returns the constant parameter (bits 0-7).
    pub fn constant(&self) -> u8 {
        (self.value & 0xFF) as u8
    }
}