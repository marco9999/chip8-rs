//! Basic primative system types.

/// CHIP8 machine unsigned word (8-bits).
#[allow(non_camel_case_types)]
pub type uword = u8;
/// CHIP8 machine signed word (8-bits).
#[allow(non_camel_case_types)]
pub type iword = i8;
/// CHIP8 machine unsigned dword (16-bits).
#[allow(non_camel_case_types)]
pub type udword = u16;
/// CHIP8 machine signed dword (16-bits).
#[allow(non_camel_case_types)]
pub type idword = i16;
/// CHIP8 machine (unsigned) pointer type (16-bits).
#[allow(non_camel_case_types)]
pub type uptr = u16;