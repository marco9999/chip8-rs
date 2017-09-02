use std::mem;
use std::cell::UnsafeCell;
use parking_lot::ReentrantMutex;
use parking_lot::ReentrantMutexGuard;
use common::types::primative::*;
use common::types::storage::*;
use common::types::storage::register::*;

/// Spu count register.
/// Holds a flag to indicate if the register changed from a zero to non-zero
/// value or vice versa, used to determine if the Spu controller should send a
/// sound event to the Core.
#[derive(Debug)]
pub struct CountRegister {
    value: UnsafeCell<uword>,
    scope_mutex: UnsafeCell<ReentrantMutex<()>>,
    sound_flag: UnsafeCell<bool>,
}

impl CountRegister {
    pub fn new() -> CountRegister {
        CountRegister { 
            value: UnsafeCell::new(0),
            scope_mutex: UnsafeCell::new(ReentrantMutex::new(())),
            sound_flag: UnsafeCell::new(false),
        }
    }

    /// Returns the sound flag. Always reset to false after.
    pub fn is_sound_flagged(&self) -> bool {
        unsafe { 
           let temp = *self.sound_flag.get();
           *self.sound_flag.get() = false;
           temp
        }
    }
}

impl Storage<uword> for CountRegister {
    fn storage(&self, offset: usize) -> &mut uword {
        unsafe { 
            &mut mem::transmute::<&mut uword, &mut [uword; 1]>(&mut *self.value.get())[offset] 
        }
    }

    #[allow(unused_variables)]
    fn write(&self, ctx: BusContext, offset: usize, value: uword) {
        let _write_guard = self.scope_guard();
        
        // Update sound flag.
        let old_value = *self.storage(offset);
        if (old_value == 0 && value > 0) || (old_value > 0 && value == 0) {
            unsafe { *self.sound_flag.get() = true; }
        }

        *self.storage(offset) = value;
    }
}

impl Register<uword> for CountRegister {}

impl SyncRegister for CountRegister {
    fn scope_guard(&self) -> ReentrantMutexGuard<()> {
        unsafe { (*self.scope_mutex.get()).lock() }
    }
}