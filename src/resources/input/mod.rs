use common::constants::input::KEYS_COUNT;
use common::types::storage::register::BitfieldParam;
use common::types::storage::register::dword_register::DwordRegister;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub keys: DwordRegister,
}

impl Input {
    pub fn new() -> Input {
        Input {
            keys: DwordRegister::new()
        }
    }
}