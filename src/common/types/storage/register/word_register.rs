use std::mem;
use std::cell::UnsafeCell;
use parking_lot::ReentrantMutex;
use parking_lot::ReentrantMutexGuard;
use common::types::primative::*;
use common::types::storage::*;
use common::types::storage::register::*;

/// Word register.
#[derive(Debug)]
pub struct WordRegister {
    /// Holds the current value of the register.
    value: UnsafeCell<uword>,
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
        WordRegister { value: UnsafeCell::new(0) }
    }
}

impl Storage<uword> for WordRegister {
    fn storage(&self, offset: usize) -> &mut uword {
        unsafe { 
            &mut mem::transmute::<&mut uword, &mut [uword; 1]>(&mut *self.value.get())[offset] 
        }
    }
}

impl Register<uword> for WordRegister {}

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
        WordRegister { value: UnsafeCell::new(value) }
    }
}

/// Synchronised write word register.
/// See WordRegister for general usage documentation and the
/// SyncRegister trait for how it differs.
#[derive(Debug)]
pub struct WordSyncRegister {
    value: UnsafeCell<uword>,
    scope_mutex: ReentrantMutex<()>,
}

impl WordSyncRegister {
    pub fn new() -> WordSyncRegister {
        WordSyncRegister { 
            value: UnsafeCell::new(0),
            scope_mutex: ReentrantMutex::new(()),
        }
    }
}

impl Storage<uword> for WordSyncRegister {
    fn storage(&self, offset: usize) -> &mut uword {
        unsafe { 
            &mut mem::transmute::<&mut uword, &mut [uword; 1]>(&mut *self.value.get())[offset] 
        }
    }

    #[allow(unused_variables)]
    fn write(&self, ctx: BusContext, offset: usize, value: uword) {
        let _write_guard = self.scope_guard();
        *self.storage(offset) = value;
    }
}

impl Register<uword> for WordSyncRegister {}

impl SyncRegister for WordSyncRegister {
    fn scope_guard(&self) -> ReentrantMutexGuard<()> {
        self.scope_mutex.lock()
    }
}