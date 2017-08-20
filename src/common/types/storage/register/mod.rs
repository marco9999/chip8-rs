//! A collection of register traits plus general purpose register types.

use num::traits::*;
use num::One;
use common::types::storage::*;

pub mod word_register;
pub mod dword_register;

/// Stores information about a bitfield.
#[derive(Debug, Copy, Clone)]
pub struct BitfieldParam {
    /// Bitfield start position.
    start: u32,

    /// Bitfield length.
    length: u32,
}

impl BitfieldParam {
    /// Create a new bitfield parameter, which can be used with registers.
    pub const fn new(start: u32, length: u32) -> BitfieldParam {
        BitfieldParam { start, length }
    }
}

/// Trait for various register functionality.
pub trait Register<T> : Storage<T>
    where T: PrimInt + One {
    /// Read a value from the register using the given bitfield parameters.
    fn read_bitfield(&mut self, ctx: BusContext, offset: usize, field: &BitfieldParam) -> T {
        let mut r: T = self.read(ctx, offset);
        r = r.unsigned_shr(field.start);
        r = r & (T::one().unsigned_shl(field.length) - T::one());
        r
    }

    /// Write a value to the register using the given bitfield parameters.
    /// If value is larger than can be stored, it will be made to fit according to the bitfield parameters.
    fn write_bitfield(&mut self, ctx: BusContext, offset: usize, field: &BitfieldParam, value: T) {
        let mut value = value & (T::one().unsigned_shl(field.length) - T::one());
        value = value.unsigned_shl(field.start);
        let mut r: T = self.read(ctx, offset);
        r = r & !((T::one().unsigned_shl(field.length) - T::one()).unsigned_shr(field.start));
        r = r & value;
        self.write(ctx, offset, r);
    }
}