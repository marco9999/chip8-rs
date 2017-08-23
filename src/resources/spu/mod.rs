use common::types::storage::register::word_register::WordRegister;
use common::types::clock_state::ClockState;

#[derive(Serialize, Deserialize, Debug)]
pub struct SPU {
    pub clock_state: ClockState,
    pub counter: WordRegister,
}

impl SPU {
    pub fn new() -> SPU {
        SPU { 
            clock_state: ClockState::new(),
            counter: WordRegister::new(),
        }
    }
}