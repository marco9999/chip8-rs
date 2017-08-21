use std::sync::Weak;
use std::sync::RwLock;
use rand;
use rand::Rng;
use Core;
use common::constants::cpu::*;
use common::types::primative::*;
use common::types::storage::*;
use common::types::storage::register::*;
use resources::Resources;
use resources::input::*;
use resources::cpu::instruction::Instruction;
use resources::cpu::instruction::RawInstruction;
use controller::Event;
use controller::Controller;

struct CPU {
    /// Core manager.
    core: Weak<Core>,
}

impl Controller for CPU {
    fn step(&self, event: &Event) -> isize {
        let mut consumed;

        match event.source {
            ref _Tick => {
                // Aquire resources.
                let core = self.core
                    .upgrade()
                    .unwrap();

                let locked_res = core
                    .resources();

                let res: Resources = locked_res
                    .write()
                    .unwrap();

                // Grab current instruction value at PC.
                let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
                let inst_value: udword = res.memory.read(BusContext::Raw, pc as usize);

                // Update PC.
                res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);

                // Get instruction details.
                let inst = Instruction::new(udword::from_be(inst_value));

                // Perform instruction.
                (INSTRUCTION_TABLE[inst.index().unwrap()])(&mut res, &inst.raw());
                
                // Finished one cycle.
                consumed = 1;
            },
        }

        consumed
    }
}

impl CPU {
    pub fn new(core: Weak<Core>) -> CPU {
        CPU {
            core,
        }
    }

    fn cls(res: &mut Resources, _inst: &RawInstruction) {
        for row in res.gpu.framebuffer.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = false;
            }
        }
    }

    fn ret(res: &mut Resources, _inst: &RawInstruction) {
        let ret_pc = res.cpu.stack.pop().unwrap();
        res.cpu.pc.write(BusContext::Raw, 0, ret_pc);
    }

    fn call_rca1802(_res: &mut Resources, _inst: &RawInstruction) {
        unimplemented!("CPU: call_rca1802");
    }

    fn jump(res: &mut Resources, inst: &RawInstruction) {
        res.cpu.pc.write(BusContext::Raw, 0, inst.address());
    }

    fn call(res: &mut Resources, inst: &RawInstruction) {
        let pc = res.cpu.pc.read(BusContext::Raw, 0);
        res.cpu.stack.push(pc);
        res.cpu.pc.write(BusContext::Raw, 0, inst.address());
    }

    fn sifeqi(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        if value == inst.immediate() {
            let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn sifnei(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        if value != inst.immediate() {
            let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn sifeq(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        if x_value == y_value {
            let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn movi(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, inst.immediate());
    }

    fn addi(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, value + inst.immediate());
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
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, x_value | y_value);
    }

    fn and(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, x_value & y_value);
    }

    fn xor(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, x_value ^ y_value);
    }

    fn add(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        let (result, of) = x_value.overflowing_add(y_value);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, result);
        res.cpu.gpr[0xF].write(BusContext::Raw, 0, of as uword);
    }

    fn sub(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        let (result, of) = x_value.overflowing_sub(y_value);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, result);
        res.cpu.gpr[0xF].write(BusContext::Raw, 0, of as uword);
    }

    fn shr1(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, value >> 1);
        res.cpu.gpr[0xF].write(BusContext::Raw, 0, value & 1);
    }

    fn rsub(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        let (result, of) = y_value.overflowing_sub(x_value);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, result);
        res.cpu.gpr[0xF].write(BusContext::Raw, 0, of as uword);
    }

    fn shl1(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, value << 1);
        res.cpu.gpr[0xF].write(BusContext::Raw, 0, value & 0x80);
    }

    fn sifne(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        if x_value != y_value {
            let pc:uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn movi_I(res: &mut Resources, inst: &RawInstruction) {
        let addr = inst.address();
        res.cpu.i.write(BusContext::Raw, 0, addr);
    }

    fn jumpr(res: &mut Resources, inst: &RawInstruction) {
        let base = res.cpu.gpr[0x0].read(BusContext::Raw, 0);
        res.cpu.pc.write(BusContext::Raw, 0, base as uptr + inst.address());
    }

    fn rand(res: &mut Resources, inst: &RawInstruction) {
        let num: u8 = rand::thread_rng().gen_range(0, 256);
        let x_index = inst.x_register();
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, num & inst.immediate());
    }

    fn draw(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_coord = res.cpu.gpr[x_index].read(BusContext::Raw, 0) as usize;
        let y_coord = res.cpu.gpr[y_index].read(BusContext::Raw, 0) as usize;
        let height = inst.low_nibble();

        res.cpu.gpr[0xF].write(BusContext::Raw, 0, 0);

        for line in 0..(height + 1) {
            let y_coord = y_coord + (line as usize);
            let addr: uptr = res.cpu.i.read(BusContext::Raw, 0);
            let row_value: uword = res.memory.read(BusContext::Raw, (addr as usize) + (line as usize));
            
            for bit in 0..(8 + 1) {
                let x_coord = x_coord + (bit as usize);
                let old_value: bool = res.gpu.framebuffer[y_coord][x_coord];
                let new_value: bool = (row_value & (1 << bit)) > 0;

                res.gpu.framebuffer[y_coord][x_coord] = new_value ^ old_value;

                if old_value == true && new_value == true {
                    res.cpu.gpr[0xF].write(BusContext::Raw, 0, 1);
                }
            }
        }
    }

    fn sifkeq(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let key = res.cpu.gpr[x_index].read(BusContext::Raw, 0) as usize;
        let key_value = res.input.keys.read_bitfield(BusContext::Raw, 0, KEYS[key]);

        // TODO: remove later...
        println!("sifkeq rotating result {} -> {}...", key_value, key_value ^ 1);
        res.input.keys.write_bitfield(BusContext::Raw, 0, KEYS[key], key_value ^ 1);

        if key_value == 1 {
            let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn sifkne(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let key = res.cpu.gpr[x_index].read(BusContext::Raw, 0) as usize;
        let key_value = res.input.keys.read_bitfield(BusContext::Raw, 0, KEYS[key]);

        // TODO: remove later...
        println!("sifkeq rotating result {} -> {}...", key_value, key_value ^ 1);
        res.input.keys.write_bitfield(BusContext::Raw, 0, KEYS[key], key_value ^ 1);

        if key_value == 0 {
            let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn timerr(res: &mut Resources, inst: &RawInstruction) {
        let timer_value = res.timer.counter.read(BusContext::Raw, 0);
        let x_index = inst.x_register();
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, timer_value);
    }

    fn keyr(res: &mut Resources, inst: &RawInstruction) {
        // TODO: remove later...
        let key: u8 = rand::thread_rng().gen_range(0, 16);
        res.input.keys.write_bitfield(BusContext::Raw, 0, KEYS[key as usize], 1);

        let x_index = inst.x_register();
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, key);
    }

    fn timerw(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        res.timer.counter.write(BusContext::Raw, 0, value);
    }

    fn soundw(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        res.spu.counter.write(BusContext::Raw, 0, value);
    }

    fn add_I(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let i_value: udword = res.cpu.i.read(BusContext::Raw, 0);
        res.cpu.i.write(BusContext::Raw, 0, i_value + (value as udword));
    }

    fn sprite_I(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let addr = (SPRITE_SIZE * value as usize) as uptr;
        res.cpu.i.write(BusContext::Raw, 0, addr as udword);
    }

    fn bcd(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value: uword = res.cpu.gpr[x_index].read(BusContext::Raw, 0);

        let hundreds = value / 100;
        let tens = (value % 100) / 10;
        let ones = (value % 10) / 1;

        let addr: uptr = res.cpu.i.read(BusContext::Raw, 0);
        res.memory.write(BusContext::Raw, addr as usize, hundreds);
        res.memory.write(BusContext::Raw, (addr + 1) as usize, tens);
        res.memory.write(BusContext::Raw, (addr + 2) as usize, ones);
    }

    fn save(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        for idx in 0..(x_index + 1) {
            let value = res.cpu.gpr[idx].read(BusContext::Raw, 0);
            let addr: uptr = res.cpu.i.read(BusContext::Raw, 0);
            res.cpu.i.write(BusContext::Raw, 0, (addr as udword) + 1);
            res.memory.write(BusContext::Raw, addr as usize, value);
        }
    }

    fn load(res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        for idx in 0..(x_index + 1) {
            let addr: uptr = res.cpu.i.read(BusContext::Raw, 0);
            res.cpu.i.write(BusContext::Raw, 0, (addr as udword) + 1);
            let value = res.memory.read(BusContext::Raw, addr as usize);
            res.cpu.gpr[idx].write(BusContext::Raw, 0, value);
        }
    }
}

/// CPU instruction function pointer table.
static INSTRUCTION_TABLE: [fn(&mut Resources, &RawInstruction); INSTRUCTION_COUNT] = [
    CPU::cls, 
    CPU::ret, 
    CPU::call_rca1802, 
    CPU::jump, 
    CPU::call, 
    CPU::sifeqi,
    CPU::sifnei,
    CPU::sifeq,
    CPU::movi, 
    CPU::addi, 
    CPU::mov, 
    CPU::or, 
    CPU::and, 
    CPU::xor, 
    CPU::add, 
    CPU::sub, 
    CPU::shr1, 
    CPU::rsub, 
    CPU::shl1, 
    CPU::sifne, 
    CPU::movi_I, 
    CPU::jumpr, 
    CPU::rand, 
    CPU::draw, 
    CPU::sifkeq, 
    CPU::sifkne, 
    CPU::timerr, 
    CPU::keyr, 
    CPU::timerw, 
    CPU::soundw, 
    CPU::add_I, 
    CPU::sprite_I,
    CPU::bcd, 
    CPU::save, 
    CPU::load,
];