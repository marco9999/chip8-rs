use std::mem;
use common::types::storage::*;
use common::types::storage::register::*;

/// Word register.
#[derive(Serialize, Deserialize, Debug)]
pub struct WordRegister {
    /// Holds the current value of the register.
    value: uword,
}

impl WordRegister {
    /// Create a new word register, with zeroed initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use chip8_emu_rs::common::types::storage::*;
    /// use chip8_emu_rs::common::types::storage::register::WordRegister;
    /// let mut r = WordRegister::new();
    /// let v: uword = r.read(BusContext::Raw, 0);
    /// assert_eq!(v, 0);
    /// ```
    pub fn new() -> WordRegister {
        WordRegister { value: 0 }
    }
}

impl Storage<uword> for WordRegister {
    fn get_storage(&mut self) -> &mut [uword] {
        unsafe { mem::transmute::<&mut uword, &mut [uword; 1]>(&mut self.value) }
    }
}

impl Register<uword> for WordRegister {
}

impl From<uword> for WordRegister {
    /// Create a new word register, with specified initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use chip8_emu_rs::common::types::storage::*;
    /// use chip8_emu_rs::common::types::storage::register::WordRegister;
    /// let mut r = WordRegister::from(3);
    /// let v: uword = r.read(BusContext::Raw, 0);
    /// assert_eq!(v, 3);
    /// ```
    fn from(value: uword) -> WordRegister {
        WordRegister { value }
    }
}