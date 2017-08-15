//! Base storage types and traits, used by the register and memory types.

use num::traits::*;

pub mod memory;
pub mod register;

/// CHIP8 machine unsigned word (8-bits).
#[allow(non_camel_case_types)]
pub type uword = u8;
/// CHIP8 machine signed word (8-bits).
#[allow(non_camel_case_types)]
pub type iword = i8;
/// CHIP8 machine unsigned dword (16-bits).
#[allow(non_camel_case_types)]
pub type udword = u16;
/// CHIP8 machine signed dword (16-bits).
#[allow(non_camel_case_types)]
pub type idword = i16;
/// CHIP8 machine (unsigned) pointer type (16-bits).
#[allow(non_camel_case_types)]
pub type uptr = u16;


/// Describes a bus access context, used to implement custom behaviour.
/// Where the storage is not accessed through a bus, use 'Raw'
/// (For example, a register attached directly to a CPU.)
#[derive(Debug, Copy, Clone)]
pub enum BusContext {
    Raw,
}

/// Trait for read/writing to storage.
pub trait Storage<T: PrimInt> {
    /// Get a mutable reference to storage of type T.
    fn get_storage(&mut self) -> &mut [T];

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
        let v = self.get_storage()[offset];
        self.mutate_read(ctx, v)
    }

    /// Write a T, applying the write modifier.
    fn write(&mut self, ctx: BusContext, offset: usize, value: T) {
        let m = self.mutate_write(ctx, value);
        self.get_storage()[offset] = m;
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

pub trait WrappingOps {
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_neg(self) -> Self;
}

impl WrappingOps for uword {
    fn wrapping_add(self, rhs: uword) -> Self {
        uword::wrapping_add(self, rhs)
    }
    fn wrapping_sub(self, rhs: uword) -> Self {
        uword::wrapping_sub(self, rhs)
    }
    fn wrapping_mul(self, rhs: uword) -> Self {
        uword::wrapping_mul(self, rhs)
    }
    fn wrapping_neg(self) -> Self {
        uword::wrapping_neg(self)
    }
}

impl WrappingOps for udword {
    fn wrapping_add(self, rhs: udword) -> Self {
        udword::wrapping_add(self, rhs)
    }
    fn wrapping_sub(self, rhs: udword) -> Self {
        udword::wrapping_sub(self, rhs)
    }
    fn wrapping_mul(self, rhs: udword) -> Self {
        udword::wrapping_mul(self, rhs)
    }
    fn wrapping_neg(self) -> Self {
        udword::wrapping_neg(self)
    }
}