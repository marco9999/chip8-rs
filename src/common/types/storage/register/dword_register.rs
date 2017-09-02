use std::mem;
use std::cell::UnsafeCell;
use common::types::primative::*;
use common::types::storage::*;
use common::types::storage::register::*;

/// Dword register.
#[derive(Debug)]
pub struct DwordRegister {
    /// Holds the current value of the register.
    value: UnsafeCell<udword>,
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
        DwordRegister { value: UnsafeCell::new(0) }
    }
}

impl Storage<uword> for DwordRegister {
    fn storage(&self, offset: usize) -> &mut uword {
        unsafe { 
            &mut mem::transmute::<&mut udword, &mut [uword; 2]>(&mut *self.value.get())[offset] 
        }
    }
}

impl Storage<udword> for DwordRegister {
    fn storage(&self, offset: usize) -> &mut udword {
        unsafe { 
            &mut mem::transmute::<&mut udword, &mut [udword; 1]>(&mut *self.value.get())[offset]
        }
    }
}

impl Register<udword> for DwordRegister {}

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
        DwordRegister { value: UnsafeCell::new(value) }
    }
}