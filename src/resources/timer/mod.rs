use common::types::storage::register::word_register::WordRegister;
use common::types::clock_state::ClockState;

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    pub clock_state: ClockState,
    pub counter: WordRegister,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { 
            clock_state: ClockState::new(),
            counter: WordRegister::new(),
        }
    }
} 