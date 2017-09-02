pub mod register;

use common::types::clock_state::ClockState;
use resources::spu::register::CountRegister;

#[derive(Debug)]
pub struct Spu {
    pub clock_state: ClockState,
    pub counter: CountRegister,
}

impl Spu {
    pub fn new() -> Spu {
        Spu { 
            clock_state: ClockState::new(),
            counter: CountRegister::new(),
        }
    }
}