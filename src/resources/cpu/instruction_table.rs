use common::constants::cpu::INSTRUCTION_COUNT;
use super::instruction::Instruction;

/// CPU instruction lookup, which returns a unique index for use in a function call array.
/// For documentation, see here: https://en.wikipedia.org/wiki/CHIP-8
pub fn lookup(inst: &Instruction) -> usize {
    let index: usize = match inst.high_nibble() {
        0x0 => {
            match inst.constant() {
                0xE0 => 0,
                0xEE => 1,
                _ => 2,
            }
        }
        0x1 => {
            3
        }
        0x2 => {
            4
        }
        0x3 => {
            5
        }
        0x4 => {
            6
        }
        0x5 => {
            7
        }
        0x6 => {
            8
        }
        0x7 => {
            9
        }
        0x8 => {
            match inst.low_nibble() {
                0x0 => 10,
                0x1 => 11,
                0x2 => 12,
                0x3 => 13,
                0x4 => 14,
                0x5 => 15,
                0x6 => 16,
                0x7 => 17,
                0xE => 18,
                _ => panic!("Couldn't find the proper instruction!"),
            }
        }
        0x9 => {
            match inst.low_nibble() {
                0x0 => 19,
                _ => panic!("Couldn't find the proper instruction!"),
            }
        }
        0xA => {

        }
        0xB => {

        }
        0xC => {

        }
        0xD => {

        }
        0xE => {

        }
        0xF => {

        }
        _ => panic!("Couldn't find the proper instruction!"),
    };

    assert!(index <= INSTRUCTION_COUNT);
    index
}