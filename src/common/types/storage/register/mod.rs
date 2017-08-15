//! A collection of register traits plus general purpose register types.

use std::ops::*;
use num::traits::*;
use num::One;
use common::types::storage::*;

mod word_register;
mod dword_register;

pub use self::word_register::WordRegister;
pub use self::dword_register::DwordRegister;

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
    const fn new(start: u32, length: u32) -> BitfieldParam {
        BitfieldParam { start, length }
    }
}

/// Trait for various register functionality.
pub trait Register<T> : Storage<T>
    where T: PrimInt + One + WrappingOps {

    /// Read a value from the register using the given bitfield parameters.
    fn read_bitfield(&mut self, ctx: BusContext, offset: usize, field: BitfieldParam) -> T {
        let mut r: T = self.read(ctx, offset);
        r = r.unsigned_shr(field.start);
        r = r & (T::one().unsigned_shl(field.length) - T::one());
        r
    }

    /// Write a value to the register using the given bitfield parameters.
    /// If value is larger than can be stored, it will be made to fit according to the bitfield parameters.
    fn write_bitfield(&mut self, ctx: BusContext, offset: usize, field: BitfieldParam, value: T) {
        let mut value = value & (T::one().unsigned_shl(field.length) - T::one());
        value = value.unsigned_shl(field.start);
        let mut r: T = self.read(ctx, offset);
        r = r & !((T::one().unsigned_shl(field.length) - T::one()).unsigned_shr(field.start));
        r = r & value;
        self.write(ctx, offset, r);
    }

    /// ORs the register with the given value.
    fn or(&mut self, ctx: BusContext, offset: usize, value: T) {
        let mut r: T = self.read(ctx, offset);
        r = r | value;
        self.write(ctx, offset, r);
    }

    /// ANDs the register with the given value.
    fn and(&mut self, ctx: BusContext, offset: usize, value: T) {
        let mut r: T = self.read(ctx, offset);
        r = r & value;
        self.write(ctx, offset, r);
    }

    /// XORs the register with the given value.
    fn xor(&mut self, ctx: BusContext, offset: usize, value: T) {
        let mut r: T = self.read(ctx, offset);
        r = r ^ value;
        self.write(ctx, offset, r);
    }

    /// Adds the register with the given value (overflowing).
    fn add(&mut self, ctx: BusContext, offset: usize, value: T) {
        let mut r: T = self.read(ctx, offset);
        r = r.wrapping_add(value);
        self.write(ctx, offset, r);
    }

    /// Subs the register with the given value (overflowing).
    fn sub(&mut self, ctx: BusContext, offset: usize, value: T) {
        let mut r: T = self.read(ctx, offset);
        r = r.wrapping_sub(value);
        self.write(ctx, offset, r);
    }

    /// Multiplys the register with the given value.
    fn mul(&mut self, ctx: BusContext, offset: usize, value: T) {
        let mut r: T = self.read(ctx, offset);
        r = r.wrapping_mul(value);
        self.write(ctx, offset, r);
    }

    /// Divides the register with the given value.
    fn div(&mut self, ctx: BusContext, offset: usize, value: T) {
        let mut r: T = self.read(ctx, offset);
        r = r / value;
        self.write(ctx, offset, r);
    }

    /// Divides the register with the given value and stores the remainder.
    fn rem(&mut self, ctx: BusContext, offset: usize, value: T) {
        let mut r: T = self.read(ctx, offset);
        r = r % value;
        self.write(ctx, offset, r);
    }

    fn shl(&mut self, ctx: BusContext, offset: usize, amount: u32) {
        let mut r: T = self.read(ctx, offset);
        r = r.unsigned_shl(amount);
        self.write(ctx, offset, r);
    }

    fn shr_unsigned(&mut self, ctx: BusContext, offset: usize, amount: u32) {
        let mut r: T = self.read(ctx, offset);
        r = r.unsigned_shr(amount);
        self.write(ctx, offset, r);
    }

    fn shr_signed(&mut self, ctx: BusContext, offset: usize, amount: u32) {
        let mut r: T = self.read(ctx, offset);
        r = r.signed_shr(amount);
        self.write(ctx, offset, r);
    }

    fn not(&mut self, ctx: BusContext, offset: usize) {
        let mut r: T = self.read(ctx, offset);
        r = !r;
        self.write(ctx, offset, r);
    }
}

impl<T> AddAssign<T> for Register<T> 
    where T: PrimInt + One + WrappingOps {
    fn add_assign(&mut self, rhs: T) {
        self.add(BusContext::Raw, 0, rhs);
    }
}

impl<T> BitAndAssign<T> for Register<T> 
    where T: PrimInt + One + WrappingOps {
    fn bitand_assign(&mut self, rhs: T) {
        self.and(BusContext::Raw, 0, rhs);
    }
}

impl<T> BitOrAssign<T> for Register<T> 
    where T: PrimInt + One + WrappingOps {
    fn bitor_assign(&mut self, rhs: T) {
        self.or(BusContext::Raw, 0, rhs);
    }
}

impl<T> BitXorAssign<T> for Register<T> 
    where T: PrimInt + One + WrappingOps {
    fn bitxor_assign(&mut self, rhs: T) {
        self.xor(BusContext::Raw, 0, rhs);
    }
}

impl<T> DivAssign<T> for Register<T> 
    where T: PrimInt + One + WrappingOps {
    fn div_assign(&mut self, rhs: T) {
        self.div(BusContext::Raw, 0, rhs);
    }
}

impl<T> MulAssign<T> for Register<T> 
    where T: PrimInt + One + WrappingOps {
    fn mul_assign(&mut self, rhs: T) {
        self.mul(BusContext::Raw, 0, rhs);
    }
}

impl<T> RemAssign<T> for Register<T> 
    where T: PrimInt + One + WrappingOps {
    fn rem_assign(&mut self, rhs: T) {
        self.rem(BusContext::Raw, 0, rhs);
    }
}

impl<T> ShlAssign<u32> for Register<T>
    where T: PrimInt + One + WrappingOps {
    fn shl_assign(&mut self, rhs: u32) {
        Register::<T>::shl(self, BusContext::Raw, 0, rhs);
    }
}

impl<T> ShrAssign<u32> for Register<T>
    where T: PrimInt + One + WrappingOps {
    fn shr_assign(&mut self, rhs: u32) {
        Register::<T>::shr_unsigned(self, BusContext::Raw, 0, rhs);
    }
}

impl<T> SubAssign<T> for Register<T> 
    where T: PrimInt + One + WrappingOps {
    fn sub_assign(&mut self, rhs: T) {
        self.sub(BusContext::Raw, 0, rhs);
    }
}