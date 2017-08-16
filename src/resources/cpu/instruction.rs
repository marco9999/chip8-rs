use common::types::storage::{udword, uptr};
use resources::cpu::instruction_table::lookup;

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    /// Unique instruction index.
    index: Option<usize>,
    /// Raw instruction.
    raw_inst: RawInstruction,
}

impl Instruction {
    /// Construct a new instruction, and performs a lookup to determine type of instruction.
    pub fn new(raw_inst: RawInstruction) -> Instruction {
        let index = lookup(raw_inst);
        Instruction {
            index,
            raw_inst,
        }
    }

    /// Returns the unique instruction index previously looked up.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn raw_instruction(&self) -> RawInstruction {
        self.raw_inst
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RawInstruction {
    /// Raw instruction value.
    value: udword,
}

impl RawInstruction {
    /// Construct a new instruction, and performs a lookup to determine type of instruction.
    pub fn new(value: udword) -> RawInstruction {
        RawInstruction {
            value,
        }
    }

    /// Returns the upper 4-bits of an instruction.
    pub fn high_nibble(&self) -> u8 {
        ((self.value & 0xF000) >> 12) as u8
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