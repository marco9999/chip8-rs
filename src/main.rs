extern crate chip8_emu_rs as core;

use core::common::types::storage::*;
use core::common::types::storage::memory::*;
use core::common::types::storage::register::*;

fn main() {
    let mut word_memory = WordMemory::new(5);
    let mut word_register = WordRegister::from(0x80);
    let mut word_register1 = WordRegister::from(0x20);
    let mut dword_register = DwordRegister::from(0x8000);

    let word_memory_value = word_memory.read(BusContext::Raw, 1);
    let word_register_value: uword = word_register.read(BusContext::Raw, 0);
    let dword_register_value: udword = dword_register.read(BusContext::Raw, 0);
    
    let r: uword = dword_register.read(BusContext::Raw, 0);
    dword_register.write(BusContext::Raw, 0, (r + 2) as udword);
    let dword_register_value: udword = dword_register.read(BusContext::Raw, 0);

    let mut reg: WordRegister = WordRegister::from(2);

    let mut ref_word_reg: &mut Register<uword> = &mut word_register;
    *ref_word_reg += 5;

    println!("{} {}, {}", word_memory_value, word_register_value as iword, dword_register_value as idword);
}
