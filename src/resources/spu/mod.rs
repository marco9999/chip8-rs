use common::types::storage::register::word_register::WordRegister;

#[derive(Serialize, Deserialize, Debug)]
pub struct SPU {
    counter: WordRegister,
}

impl SPU {
    pub fn new() -> SPU {
        SPU { 
            counter: WordRegister::new() 
        }
    }
}