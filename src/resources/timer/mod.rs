use common::types::storage::register::word_register::WordRegister;

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