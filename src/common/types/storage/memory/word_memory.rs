use std::vec::Vec;
use std::mem;
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
}

impl Storage<uword> for WordMemory {
    fn storage(&mut self) -> &mut [uword] {
        self.values.as_mut_slice()
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
    fn storage(&mut self) -> &mut [udword] {
        unsafe { mem::transmute::<&mut [uword], &mut [udword]>(self.values.as_mut_slice()) }
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