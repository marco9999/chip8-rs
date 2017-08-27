//! Word memory.
//!
//! Represents a block of memory of arbitrary size.
//! Loads and stores are automatically converted from the Chip8's
//! big endian memory layout into the host's endianness.

use std::vec::Vec;
use std::io::*;
use std::fs::File;
use common::types::storage::*;
use common::types::primative::*;

/// Word memory.
#[derive(Serialize, Deserialize, Debug)]
pub struct WordMemory {
    values: Vec<uword>,
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
            values: vec![0; size],
        }
    }

    pub fn read_file(&mut self, offset: usize, path: &str) -> Result<()> {
        let mut file = File::open(path)?;
        file.read(&mut self.values[offset..])?;
        Ok(())
    }

    pub fn dump_file(&self, path: &str) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.values)?;
        Ok(())
    }
}

impl Storage<uword> for WordMemory {
    fn storage(&mut self, offset: usize) -> &mut uword {
        &mut self.values[offset]
    }

    #[allow(unused_variables)]
    fn mutate_read(&mut self, ctx: BusContext, value: uword) -> uword {
        uword::from_be(value)
    }

    #[allow(unused_variables)]
    fn mutate_write(&mut self, ctx: BusContext, value: uword) -> uword {
        uword::to_be(value)
    }
}

impl Storage<udword> for WordMemory {
    fn storage(&mut self, offset: usize) -> &mut udword {
        unsafe { 
            &mut *(&mut self.values[offset] as *mut u8 as *mut udword)
        }
    }

    #[allow(unused_variables)]
    fn mutate_read(&mut self, ctx: BusContext, value: udword) -> udword {
        udword::from_be(value)
    }

    #[allow(unused_variables)]
    fn mutate_write(&mut self, ctx: BusContext, value: udword) -> udword {
        udword::to_be(value)
    }
}