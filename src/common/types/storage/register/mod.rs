//! A collection of register traits plus general purpose register types.

use num::traits::*;
use num::One;
use parking_lot::ReentrantMutexGuard;
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
/// These are mostly convenience funtions that could be done through a 
/// separate read and write calls.
pub trait Register<T> : Storage<T>
    where T: PrimInt + One {
    /// Read a value from the register using the given bitfield parameters.
    fn read_bitfield(&self, ctx: BusContext, offset: usize, field: &BitfieldParam) -> T {
        let mut r: T = self.read(ctx, offset);
        r = r.unsigned_shr(field.start);
        r = r & (T::one().unsigned_shl(field.length) - T::one());
        r
    }

    /// Write a value to the register using the given bitfield parameters.
    /// If value is larger than can be stored, it will be made to fit according
    /// to the bitfield parameters.
    fn write_bitfield(&self, ctx: BusContext, offset: usize, field: &BitfieldParam, value: T) {
        let mut value = value & (T::one().unsigned_shl(field.length) - T::one());
        value = value.unsigned_shl(field.start);
        let mut r: T = self.read(ctx, offset);
        let mask = T::one().unsigned_shl(field.length) - T::one();
        let mask = mask.unsigned_shl(field.start);
        r = r & !mask;
        r = r | value;
        self.write(ctx, offset, r);
    }
}

/// Trait to provide synchronous access through multiple controllers.
///
/// Intended to be locked for the appropriate scope within a perhiperal 
/// controller's function. The lock is re-entrant; it will not block a thread 
/// that currently owns the lock. This is most useful for example when entering
/// a scope (analogous to a transaction block) with scope_guard(), reading the
/// register, then writing to the register using write() after doing some
/// calculation.
///
/// Reads are lock free, and will not block (similar to RwLock).
///
/// Use this register sparingly for performance reasons - for registers that are
/// not simulaneously accessed through multiple controllers, there is no need to
/// use this. Use a regular register instead. Even when they are simultaniously
/// used, it might not need to be Sync - consider a DMA count register which is
/// only accessed by a CPU if the DMA control register is set to stop. In this
/// case, the count register will never be updated by the DMA controller at the
/// same time, hence no data contention issues.
pub trait SyncRegister {
    fn scope_guard(&self) -> ReentrantMutexGuard<()>;
}