use common::types::storage::register::word_register::WordRegister;
use common::types::clock_state::ClockState;

#[derive(Serialize, Deserialize, Debug)]
pub struct Spu {
    pub clock_state: ClockState,
    pub counter: WordRegister,
}

impl Spu {
    pub fn new() -> Spu {
        Spu { 
            clock_state: ClockState::new(),
            counter: WordRegister::new(),
        }
    }
}