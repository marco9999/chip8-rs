//! Word memory.
//!
//! Represents a block of memory of arbitrary size.
//! Loads and stores are automatically converted from the Chip8's
//! big endian memory layout into the host's endianness.

use std::vec::Vec;
use std::io::*;
use std::fs::File;
use std::cell::UnsafeCell;
use common::types::storage::*;
use common::types::primative::*;

/// Word memory.
#[derive(Debug)]
pub struct WordMemory {
    values: UnsafeCell<Vec<uword>>,
}

impl WordMemory {
    /// Create new zeroed word memory of the specified size.
    ///
    /// # Examples
    ///
    /// ```
    /// use chip8_emu_rs::common::types::storage::*;
    /// use chip8_emu_rs::common::types::storage::memory::WordMemory;
    /// let mut m = WordMemory::new(1024);
    /// let v: uword = m.read(BusContext::Raw, 0);
    /// assert_eq!(v, 0);
    /// ```
    pub fn new(size: usize) -> WordMemory {
        WordMemory {
            values: UnsafeCell::new(vec![0; size]),
        }
    }

    pub fn read_file(&self, offset: usize, path: &str) -> Result<()> {
        unsafe {
            let mut file = File::open(path)?;
            file.read(&mut (*self.values.get())[offset..])?;
            Ok(())
        }
    }

    pub fn dump_file(&self, path: &str) -> Result<()> {
        unsafe {
            let mut file = File::create(path)?;
            file.write_all(&*self.values.get())?;
            Ok(())
        }
    }
}

impl Storage<uword> for WordMemory {
    fn storage(&self, offset: usize) -> &mut uword {
        unsafe { &mut (*self.values.get())[offset] }
    }

    #[allow(unused_variables)]
    fn read(&self, ctx: BusContext, offset: usize) -> uword {
         uword::from_be(*self.storage(offset))
    }

    #[allow(unused_variables)]
    fn write(&self, ctx: BusContext, offset: usize, value: uword) {
        *self.storage(offset) = uword::to_be(value);
    }
}

impl Storage<udword> for WordMemory {
    fn storage(&self, offset: usize) -> &mut udword {
        unsafe { 
            &mut *(&mut (*self.values.get())[offset] as *mut u8 as *mut udword)
        }
    }

    #[allow(unused_variables)]
    fn read(&self, ctx: BusContext, offset: usize) -> udword {
         udword::from_be(*self.storage(offset))
    }

    #[allow(unused_variables)]
    fn write(&self, ctx: BusContext, offset: usize, value: udword) {
        *self.storage(offset) = udword::to_be(value);
    }
}