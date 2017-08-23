use common::constants::cpu::*;
use common::types::primative::*;
use common::types::clock_state::ClockState;
use common::types::storage::register::word_register::WordRegister;
use common::types::storage::register::dword_register::DwordRegister;
use common::types::storage::register::BitfieldParam;

pub mod instruction_lookup;
pub mod instruction;

pub static KEY_0: BitfieldParam = BitfieldParam::new(0x0, 1);
pub static KEY_1: BitfieldParam = BitfieldParam::new(0x1, 1);
pub static KEY_2: BitfieldParam = BitfieldParam::new(0x2, 1);
pub static KEY_3: BitfieldParam = BitfieldParam::new(0x3, 1);
pub static KEY_4: BitfieldParam = BitfieldParam::new(0x4, 1);
pub static KEY_5: BitfieldParam = BitfieldParam::new(0x5, 1);
pub static KEY_6: BitfieldParam = BitfieldParam::new(0x6, 1);
pub static KEY_7: BitfieldParam = BitfieldParam::new(0x7, 1);
pub static KEY_8: BitfieldParam = BitfieldParam::new(0x8, 1);
pub static KEY_9: BitfieldParam = BitfieldParam::new(0x9, 1);
pub static KEY_A: BitfieldParam = BitfieldParam::new(0xA, 1);
pub static KEY_B: BitfieldParam = BitfieldParam::new(0xB, 1);
pub static KEY_C: BitfieldParam = BitfieldParam::new(0xC, 1);
pub static KEY_D: BitfieldParam = BitfieldParam::new(0xD, 1);
pub static KEY_E: BitfieldParam = BitfieldParam::new(0xE, 1);
pub static KEY_F: BitfieldParam = BitfieldParam::new(0xF, 1);
pub static KEYS: [&'static BitfieldParam; KEYS_COUNT] = [&KEY_0, &KEY_1, &KEY_2, &KEY_3, &KEY_4, &KEY_5, &KEY_6, &KEY_7, &KEY_8, &KEY_9, &KEY_A, &KEY_B, &KEY_C, &KEY_D, &KEY_E, &KEY_F];

pub struct CPU {
    pub clock_state: ClockState,
    pub pc: DwordRegister,
    pub gpr: [WordRegister; 16],
    pub i: DwordRegister,
    pub stack: Vec<uptr>,
    pub keys: DwordRegister,
    pub framebuffer: [[bool; HORIZONTAL_RES]; VERTICAL_RES],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            clock_state: ClockState::new(),
            pc: DwordRegister::from(0x200),
            gpr: [WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), 
                  WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new(), WordRegister::new() ],
            i: DwordRegister::new(),
            stack: Vec::new(),
            keys: DwordRegister::new(),
            framebuffer: [[false; HORIZONTAL_RES]; VERTICAL_RES],
        }
    }
}