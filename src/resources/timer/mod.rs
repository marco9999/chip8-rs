use common::types::storage::register::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    counter: WordRegister,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { 
            counter: WordRegister::new() 
        }
    }
}