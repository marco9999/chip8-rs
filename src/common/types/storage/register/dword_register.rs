use std::mem;
use common::types::storage::*;
use common::types::storage::register::*;

/// Dword register.
#[derive(Serialize, Deserialize, Debug)]
pub struct DwordRegister {
    /// Holds the current value of the register.
    value: udword,
}

impl DwordRegister {
    /// Create a new dword register, with zeroed initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use chip8_emu_rs::common::types::storage::*;
    /// use chip8_emu_rs::common::types::storage::register::DwordRegister;
    /// let mut r = DwordRegister::new();
    /// let v: udword = r.read(BusContext::Raw, 0);
    /// assert_eq!(v, 0);
    /// ```
    pub fn new() -> DwordRegister {
        DwordRegister { value: 0 }
    }
}

impl Storage<uword> for DwordRegister {
    fn storage(&mut self) -> &mut [uword] {
        unsafe { mem::transmute::<&mut udword, &mut [uword; 2]>(&mut self.value) }
    }
}

impl Storage<udword> for DwordRegister {
    fn storage(&mut self) -> &mut [udword] {
        unsafe { mem::transmute::<&mut udword, &mut [udword; 1]>(&mut self.value) }
    }
}

impl Register<uword> for DwordRegister {
}

impl Register<udword> for DwordRegister {
}

impl From<udword> for DwordRegister {
    /// Create a new word register, with specified initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// use chip8_emu_rs::common::types::storage::*;
    /// use chip8_emu_rs::common::types::storage::register::DwordRegister;
    /// let mut r = DwordRegister::new_init(3);
    /// let v: udword = r.read(BusContext::Raw, 0);
    /// assert_eq!(v, 3);
    /// ```
    fn from(value: udword) -> DwordRegister {
        DwordRegister { value }
    }
}