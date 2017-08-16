use std::sync::Weak;
use common::constants::cpu::*;
use resources::Resources;
use controller::CoreResources;
use resources::cpu::instruction::Instruction;

struct CPU {
    /// Core manager resources.
    core_resources: CoreResources,

    /// Instruction function pointer table.
    instruction_table: [fn(&mut Resources, &RawInstruction), common::constants::cpu::INSTRUCTION_COUNT],
}

impl CPU {
    pub fn new(core: Weak<Core>) -> CPU {
        CPU {
            core_resources: CoreResources::new(core),
            instruction_table: [cls, ret, call_rca1802, jump, call, sifeqi, sifnei, movi, addi, mov, or, and, xor, add, subshr1, rsub, shl1, sifne, mov_I, call_I, rand, draw, sifkeq, sifkne, timerr, keyr, timerw, soundw, add_I, sprite_I, bcd, save, load],
        }
    }

    fn cls(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn ret(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn call_rca1802(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn jump(res: &mut Resources, inst: &RawInstruction) {
        res.cpu.pc.write(BusContext::Raw, 0, inst.address());
    }

    fn call(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn sifeqi(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn sifnei(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn movi(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn addi(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn mov(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn or(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn and(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn xor(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn add(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn subshr1(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn rsub(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn shl1(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn sifne(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn mov_I(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn call_I(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn rand(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn draw(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn sifkeq(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn sifkne(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn timerr(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn keyr(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn timerw(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn soundw(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn add_I(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn sprite_I(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn bcd(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn save(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn load(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }
}