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
    fn storage(&mut self, offset: usize) -> &mut T;

    /// Read mutator, applied on every read. 
    /// By default does not modify the read.
    #[allow(unused_variables)]
    fn mutate_read(&mut self, ctx: BusContext, value: T) -> T {
        value
    }

    /// Write mutator, applied on every write.
    /// By default does not modify the write.
    #[allow(unused_variables)]
    fn mutate_write(&mut self, ctx: BusContext, value: T) -> T {
        value
    }

    /// Read a T, applying the read modifier.
    fn read(&mut self, ctx: BusContext, offset: usize) -> T {
        let v = *self.storage(offset);
        self.mutate_read(ctx, v)
    }

    /// Write a T, applying the write modifier.
    fn write(&mut self, ctx: BusContext, offset: usize, value: T) {
        let m = self.mutate_write(ctx, value);
        *self.storage(offset) = m;
    }

    /// Read a slice of T's.
    #[allow(unused_variables)]
    fn read_slice(&mut self, ctx: BusContext, offset: usize, values: &mut [T]) {
        for index in 0..values.len() {
            values[offset] = self.read(ctx, offset + index)
        }
    }

    /// Write a slice of T's.
    #[allow(unused_variables)]
    fn write_slice(&mut self, ctx: BusContext, offset: usize, values: &[T]) {
        for index in 0..values.len() {
            self.write(ctx, offset + index, values[index])
        }
    }
}