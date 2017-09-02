//! Base storage types and traits, used by the register and memory types.

use num::traits::*;

pub mod memory;
pub mod register;

/// Describes a bus access context, used to implement custom behaviour.
/// Where the storage is not accessed through a bus, use 'Raw'
/// (For example, a register attached directly to a CPU.)
#[derive(Debug, Copy, Clone)]
pub enum BusContext {
    Raw,
}

/// Trait for read/writing to storage.
pub trait Storage<T: PrimInt> {
    /// Get a mutable reference to a single unit of storage of type T.
    fn storage(&self, offset: usize) -> &mut T;

    /// Read a T. Override with functionality if needed.
    #[allow(unused_variables)]
    fn read(&self, ctx: BusContext, offset: usize) -> T {
        *self.storage(offset)
    }

    /// Write a T. Override with functionality if needed.
    #[allow(unused_variables)]
    fn write(&self, ctx: BusContext, offset: usize, value: T) {
        *self.storage(offset) = value;
    }

    /// Read a slice of T's.
    #[allow(unused_variables)]
    fn read_slice(&self, ctx: BusContext, offset: usize, values: &mut [T]) {
        for index in 0..values.len() {
            values[offset] = self.read(ctx, offset + index)
        }
    }

    /// Write a slice of T's.
    #[allow(unused_variables)]
    fn write_slice(&self, ctx: BusContext, offset: usize, values: &[T]) {
        for index in 0..values.len() {
            self.write(ctx, offset + index, values[index])
        }
    }
}