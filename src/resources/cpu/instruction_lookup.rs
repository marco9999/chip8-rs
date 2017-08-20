use resources::cpu::instruction::RawInstruction;

/// CPU instruction lookup, which returns a unique index for use in a function call array.
/// For documentation, see here: https://en.wikipedia.org/wiki/CHIP-8
pub fn lookup(inst: RawInstruction) -> Option<usize> {
    let index: Option<usize> = match inst.high_nibble() {
        0x0 => {
            match inst.immediate() {
                0xE0 => Some(0), // cls
                0xEE => Some(1), // ret
                _ => Some(2), // call_rca1802
            }
        },
        0x1 => {
            Some(3)  // jump
        },
        0x2 => {
            Some(4) // call
        },
        0x3 => {
            Some(5) // sifeqi
        },
        0x4 => {
            Some(6) // sifnei
        },
        0x5 => {
            Some(7) // sifeq
        },
        0x6 => {
            Some(8) // movi
        },
        0x7 => {
            Some(9) // addi
        },
        0x8 => {
            match inst.low_nibble() {
                0x0 => Some(10), // mov
                0x1 => Some(11), // or
                0x2 => Some(12), // and
                0x3 => Some(13), // xor
                0x4 => Some(14), // add
                0x5 => Some(15), // sub
                0x6 => Some(16), // shr1
                0x7 => Some(17), // rsub
                0xE => Some(18), // shl1
                _ => None,
            }
        },
        0x9 => {
            match inst.low_nibble() {
                0x0 => Some(19), // sifne
                _ => None,
            }
        },
        0xA => {
            Some(20) // mov_I
        },
        0xB => {
            Some(21) // call_I
        },
        0xC => {
            Some(22) // rand
        },
        0xD => {
            Some(23) // draw
        },
        0xE => {
            match inst.immediate() {
                0x9E => Some(24), // sifkeq
                0xA1 => Some(25), // sifkne
                _ => None,
            }
        },
        0xF => {
            match inst.immediate() {
                0x07 => Some(26), // timerr
                0x0A => Some(27), // keyr
                0x15 => Some(28), // timerw
                0x18 => Some(29), // soundw
                0x1E => Some(30), // add_I
                0x29 => Some(31), // sprite_I
                0x33 => Some(32), // bcd
                0x55 => Some(33), // save
                0x65 => Some(34), // load
                _ => None,
            }
        },
        _ => None,
    };

    index
}