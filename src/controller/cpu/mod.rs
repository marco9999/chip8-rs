use std::sync::Weak;
use common::constants::cpu::*;
use resources::Resources;
use resources::cpu::instruction::Instruction;
use controller::{CoreResources, Event};

struct CPU {
    /// Core manager.
    core: Weak<Core>,

    /// Instruction function pointer table.
    instruction_table: [fn(&mut Resources, &RawInstruction), common::constants::cpu::INSTRUCTION_COUNT],
}

impl Controller for CPU {
    fn step(&self, event: &Event) -> isize {
        match event.source {
            Tick => {
                // Aquire resources.
                let res = core.upgrade().unwrap().resources();

                // Grab current instruction value at PC.
                let pc = res.CPU.PC.read(BusContext::Raw, 0);
                let inst_value: udword = res.memory.read(BusContext::Raw, pc);

                // Update PC.
                res.CPU.PC += INSTRUCTION_SIZE;

                // Get instruction details.
                let inst = Instruction::new(udword::from_be(inst_value));

                // Perform instruction.
                (self.instruction_table[inst.index.unwrap()])(&res, &inst.raw_inst);
                
                // Finished one cycle.
                1
            },
        }
    }
}

impl CPU {
    pub fn new(core: Weak<Core>) -> CPU {
        CPU {
            core,
            instruction_table: [cls, ret, call_rca1802, jump, call, sifeqi, sifnei, movi, addi, mov, or, and, xor, add, sub, shr1, rsub, shl1, sifne, mov_I, call_I, rand, draw, sifkeq, sifkne, timerr, keyr, timerw, soundw, add_I, sprite_I, bcd, save, load],
        }
    }

    fn cls(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: cls");
    }

    fn ret(res: &mut Resources, inst: &RawInstruction) {
        let ret_pc = res.CPU.stack.pop().unwrap();
        res.CPU.PC.write(BusContext::Raw, 0, ret_pc);
    }

    fn call_rca1802(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: call_rca1802");
    }

    fn jump(res: &mut Resources, inst: &RawInstruction) {
        res.cpu.pc.write(BusContext::Raw, 0, inst.address());
    }

    fn call(res: &mut Resources, inst: &RawInstruction) {
        let pc = res.CPU.PC.read(BusContext::Raw, 0);
        res.CPU.stack.push(pc);
        res.CPU.PC.write(BusContext::Raw, 0, inst.address());
    }

    fn sifeqi(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        if value == inst.immediate() {
            res.CPU.PC += 2;
        }
    }

    fn sifnei(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        if value != inst.immediate() {
            res.CPU.PC += 2;
        }
    }

    fn movi(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, inst.immediate());
    }

    fn addi(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        res.cpu.gpr[x_index] += inst.immediate();
    }

    fn mov(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, value);
    }

    fn or(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        res.cpu.gpr[x_index] |= res.cpu.gpr[y_index].read(BusContext::Raw, 0);
    }

    fn and(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        res.cpu.gpr[x_index] &= res.cpu.gpr[y_index].read(BusContext::Raw, 0);
    }

    fn xor(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        res.cpu.gpr[x_index] ^= res.cpu.gpr[y_index].read(BusContext::Raw, 0);
    }

    fn add(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: add");
    }

    fn sub(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: sub");
    }

    fn shr1(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: shr1");
    }

    fn rsub(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: rsub");
    }

    fn shl1(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: shl1");
    }

    fn sifne(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: sifne");
    }

    fn mov_I(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: mov_I");
    }

    fn call_I(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: call_I");
    }

    fn rand(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: rand");
    }

    fn draw(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: draw");
    }

    fn sifkeq(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: sifkeq");
    }

    fn sifkne(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: sifkne");
    }

    fn timerr(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: timerr");
    }

    fn keyr(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: keyr");
    }

    fn timerw(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: timerw");
    }

    fn soundw(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: soundw");
    }

    fn add_I(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: add_I");
    }

    fn sprite_I(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: sprite_I");
    }

    fn bcd(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: bcd");
    }

    fn save(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: save");
    }

    fn load(res: &mut Resources, inst: &RawInstruction) {
        unimplemented!("CPU: load");
    }
}