use common::types::storage::register::word_register::WordSyncRegister;
use common::types::clock_state::ClockState;

#[derive(Debug)]
pub struct Timer {
    pub clock_state: ClockState,
    pub counter: WordSyncRegister,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { 
            clock_state: ClockState::new(),
            counter: WordSyncRegister::new(),
        }
    }
} 