use std::vec::Vec;
use common::types::storage::*;

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
    fn get_storage(&mut self) -> &mut [uword] {
        self.values.as_mut_slice()
    }
}
