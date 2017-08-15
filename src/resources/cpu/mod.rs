use common::types::storage::*;
use common::types::storage::register::*;

mod instruction_table;
mod instruction;

pub use self::instruction_table::lookup;
pub use self::instruction::Instruction;

#[derive(Serialize, Deserialize, Debug)]
pub struct CPU {
    pc: udword,
    gpr: [WordRegister; 16],
    i: DwordRegister,
    stack: Vec<udword>,
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