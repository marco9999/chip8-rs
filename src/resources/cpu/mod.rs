use common::types::storage::uptr;
use common::types::storage::register::word_register::WordRegister;
use common::types::storage::register::dword_register::DwordRegister;

pub mod instruction_table;
pub mod instruction;

#[derive(Serialize, Deserialize, Debug)]
pub struct CPU {
    pc: uptr,
    gpr: [WordRegister; 16],
    i: DwordRegister,
    stack: Vec<uptr>,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: 0x200,
            gpr: [WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), 
                  WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new() ],
            i: DwordRegister::new(),
            stack: Vec::new(),
        }
    }
}