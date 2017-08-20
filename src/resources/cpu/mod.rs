use common::types::primative::*;
use common::types::storage::register::word_register::WordRegister;
use common::types::storage::register::dword_register::DwordRegister;

pub mod instruction_lookup;
pub mod instruction;

#[derive(Serialize, Deserialize, Debug)]
pub struct CPU {
    pub pc: DwordRegister,
    pub gpr: [WordRegister; 16],
    pub i: DwordRegister,
    pub stack: Vec<uptr>,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: DwordRegister::from(0x200),
            gpr: [WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), 
                  WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new() ],
            i: DwordRegister::new(),
            stack: Vec::new(),
        }
    }
}