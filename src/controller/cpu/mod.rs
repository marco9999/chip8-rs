use rand;
use rand::Rng;
use std::sync::mpsc::*;
use Core;
use CoreEvent;
use common::constants::cpu::*;
use common::types::primative::*;
use common::types::storage::*;
use common::types::storage::register::*;
use resources::Resources;
use resources::cpu::*;
use resources::cpu::instruction::*;
use controller::*;

pub struct Cpu<'a> {
    /// Core manager.
    core: &'a Core,

    /// ControllerEvent queue channel receiver.
    event_queue_rx: Receiver<ControllerEvent>,

    /// ControllerEvent queue channel sender.
    event_queue_tx: SyncSender<ControllerEvent>,

    /// Instruction function pointer table.
    instruction_table: [fn(&Cpu<'a>, &mut Resources, &RawInstruction); INSTRUCTION_COUNT],
}

unsafe impl<'a> Sync for Cpu<'a> {}

impl<'a> Controller for Cpu<'a> {
    fn step(&self, event: ControllerEvent) -> Result<(), String> {
        match event {
            ControllerEvent::Tick(mut amount) => { 
                while amount > 0 {
                    // Aquire resources.
                    let res = self.core().resources()?;
                    
                    // If we are halted, don't do anything.
                    if res.cpu.halted {
                        break;
                    }

                    // Grab current instruction value at PC.
                    let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
                    let inst_value: udword = res.memory.read(BusContext::Raw, pc as usize);

                    if cfg!(build = "debug") {
                        debug!("Cpu: pc = 0x{:04X}, inst_value = {:04X}", pc, inst_value);
                    }                    

                    // Update PC.
                    res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);

                    // Get instruction details.
                    let inst = Instruction::new(inst_value);
                    let inst_index = inst.index().ok_or(format!("Cpu encountered unknown instruction 0x{:X}", inst_value))?;

                    // Perform instruction.
                    (self.instruction_table[inst_index])(self, res, &inst.raw());
                    
                    // Finished one cycle.
                    amount -= 1;
                }
            },
            ControllerEvent::Input(key, pressed) => {
                // Aquire resources and set key.
                let res = self.core().resources()?;
                res.cpu.keys.write_bitfield(BusContext::Raw, 0, &KEYS[key], pressed as udword);

                // Wake up Cpu in case it was halted from before (see instruction 'keyr'), if button was pressed.
                if pressed && res.cpu.halted {
                    res.cpu.halted = false;
                    res.cpu.halted_wake_key = Some(key as uword);
                }
            }
        }

        Ok(())
    }

    fn event_iter(&self) -> TryIter<ControllerEvent> {
        self.event_queue_rx.try_iter()
    }

    fn send_event(&self, event: ControllerEvent) {
        self.event_queue_tx.send(event).unwrap();
    }

    fn gen_tick_event(&self, time_delta_us: f64) -> Result<(), String> {
        let clock_state = &mut self.core().resources()?.cpu.clock_state;
        let bias = self.core().config().cpu_bias;
        clock_state.produce(time_delta_us, bias * CLOCK_SPEED);
        let ticks = clock_state.consume_whole();
        self.event_queue_tx.send(ControllerEvent::Tick(ticks as isize)).unwrap();
        Ok(())
    }
}

impl<'a> Cpu<'a> {
    pub fn new(core: &Core) -> Cpu {
        let (event_queue_tx, event_queue_rx) = sync_channel::<ControllerEvent>(128);
        Cpu {
            core,
            event_queue_tx,
            event_queue_rx,
            instruction_table: [
                Cpu::cls, 
                Cpu::ret, 
                Cpu::call_rca1802, 
                Cpu::jump, 
                Cpu::call, 
                Cpu::sifeqi,
                Cpu::sifnei,
                Cpu::sifeq,
                Cpu::movi, 
                Cpu::addi, 
                Cpu::mov, 
                Cpu::or, 
                Cpu::and, 
                Cpu::xor, 
                Cpu::add, 
                Cpu::sub, 
                Cpu::shr1, 
                Cpu::rsub, 
                Cpu::shl1, 
                Cpu::sifne, 
                Cpu::movi_i, 
                Cpu::jumpr, 
                Cpu::rand, 
                Cpu::draw, 
                Cpu::sifkeq, 
                Cpu::sifkne, 
                Cpu::timerr, 
                Cpu::keyr, 
                Cpu::timerw, 
                Cpu::soundw, 
                Cpu::add_i, 
                Cpu::sprite_i,
                Cpu::bcd, 
                Cpu::save, 
                Cpu::load,
            ],
        }
    }

    fn core(&self) -> &Core {
        self.core
    }

    fn cls(&self, res: &mut Resources, _inst: &RawInstruction) {
        for pixel in res.cpu.framebuffer.iter_mut() {
            *pixel = false;
        }
    }

    fn ret(&self, res: &mut Resources, _inst: &RawInstruction) {
        let ret_pc = res.cpu.stack.pop().unwrap();
        res.cpu.pc.write(BusContext::Raw, 0, ret_pc);
    }

    fn call_rca1802(&self, _res: &mut Resources, _inst: &RawInstruction) {
        // Does nothing...
    }

    fn jump(&self, res: &mut Resources, inst: &RawInstruction) {
        res.cpu.pc.write(BusContext::Raw, 0, inst.address());
    }

    fn call(&self, res: &mut Resources, inst: &RawInstruction) {
        let pc = res.cpu.pc.read(BusContext::Raw, 0);
        res.cpu.stack.push(pc);
        res.cpu.pc.write(BusContext::Raw, 0, inst.address());
    }

    fn sifeqi(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        if value == inst.immediate() {
            let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn sifnei(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        if value != inst.immediate() {
            let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn sifeq(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        if x_value == y_value {
            let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn movi(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, inst.immediate());
    }

    fn addi(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let (result, _of) = value.overflowing_add(inst.immediate());
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, result);
    }

    fn mov(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, value);
    }

    fn or(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, x_value | y_value);
    }

    fn and(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, x_value & y_value);
    }

    fn xor(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, x_value ^ y_value);
    }

    fn add(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        let (result, of) = x_value.overflowing_add(y_value);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, result);
        res.cpu.gpr[0xF].write(BusContext::Raw, 0, of as uword);
    }

    fn sub(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        let (result, of) = x_value.overflowing_sub(y_value);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, result);
        res.cpu.gpr[0xF].write(BusContext::Raw, 0, of as uword);
    }

    fn shr1(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, value >> 1);
        res.cpu.gpr[0xF].write(BusContext::Raw, 0, value & 1);
    }

    fn rsub(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        let (result, of) = y_value.overflowing_sub(x_value);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, result);
        res.cpu.gpr[0xF].write(BusContext::Raw, 0, of as uword);
    }

    fn shl1(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, value << 1);
        res.cpu.gpr[0xF].write(BusContext::Raw, 0, value & 0x80);
    }

    fn sifne(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let y_value = res.cpu.gpr[y_index].read(BusContext::Raw, 0);
        if x_value != y_value {
            let pc:uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn movi_i(&self, res: &mut Resources, inst: &RawInstruction) {
        let addr = inst.address();
        res.cpu.i.write(BusContext::Raw, 0, addr);
    }

    fn jumpr(&self, res: &mut Resources, inst: &RawInstruction) {
        let base = res.cpu.gpr[0x0].read(BusContext::Raw, 0);
        res.cpu.pc.write(BusContext::Raw, 0, base as uptr + inst.address());
    }

    fn rand(&self, res: &mut Resources, inst: &RawInstruction) {
        let num: u8 = rand::thread_rng().gen();
        let x_index = inst.x_register();
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, num & inst.immediate());
    }

    fn draw(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let y_index = inst.y_register();
        let x_coord = res.cpu.gpr[x_index].read(BusContext::Raw, 0) as usize;
        let y_coord = res.cpu.gpr[y_index].read(BusContext::Raw, 0) as usize;
        let height = inst.low_nibble();

        res.cpu.gpr[0xF].write(BusContext::Raw, 0, 0);

        for line in 0..height {
            let y_coord = y_coord + (line as usize);
            let addr: uptr = res.cpu.i.read(BusContext::Raw, 0);
            let row_value: uword = res.memory.read(BusContext::Raw, (addr as usize) + (line as usize));
            
            for bit in 0..8 {
                let x_coord = x_coord + (bit as usize);
                let px_index = (y_coord * HORIZONTAL_RES) + x_coord;
                let old_value: bool = res.cpu.framebuffer[px_index];
                let new_value: bool = (row_value & (0x80 >> bit)) > 0;

                res.cpu.framebuffer[px_index] = new_value ^ old_value;

                if old_value == true && new_value == true {
                    res.cpu.gpr[0xF].write(BusContext::Raw, 0, 1);
                }
            }
        }

        self.core().send_event(CoreEvent::Video);
    }

    fn sifkeq(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let key = res.cpu.gpr[x_index].read(BusContext::Raw, 0) as usize;
        let key_value = res.cpu.keys.read_bitfield(BusContext::Raw, 0, KEYS[key]);

        if key_value == 1 {
            let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn sifkne(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let key = res.cpu.gpr[x_index].read(BusContext::Raw, 0) as usize;
        let key_value = res.cpu.keys.read_bitfield(BusContext::Raw, 0, KEYS[key]);

        if key_value == 0 {
            let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
            res.cpu.pc.write(BusContext::Raw, 0, pc + INSTRUCTION_SIZE as uptr);
        }
    }

    fn timerr(&self, res: &mut Resources, inst: &RawInstruction) {
        let timer_value = res.timer.counter.read(BusContext::Raw, 0);
        let x_index = inst.x_register();
        res.cpu.gpr[x_index].write(BusContext::Raw, 0, timer_value);
    }

    fn keyr(&self, res: &mut Resources, inst: &RawInstruction) {
        match res.cpu.halted_wake_key {
            Some(key) => {
                let x_index = inst.x_register();
                res.cpu.gpr[x_index].write(BusContext::Raw, 0, key);
                res.cpu.halted_wake_key = None;
            },
            None => {
                let pc: uptr = res.cpu.pc.read(BusContext::Raw, 0);
                res.cpu.pc.write(BusContext::Raw, 0, pc - INSTRUCTION_SIZE as uptr);
                res.cpu.halted = true;
            }
        }
    }

    fn timerw(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        res.timer.counter.write(BusContext::Raw, 0, value);
    }

    fn soundw(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        res.spu.counter.write(BusContext::Raw, 0, value);
    }

    fn add_i(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let i_value: udword = res.cpu.i.read(BusContext::Raw, 0);
        res.cpu.i.write(BusContext::Raw, 0, i_value + (value as udword));
    }

    fn sprite_i(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        let value = res.cpu.gpr[x_index].read(BusContext::Raw, 0);
        let addr = (SPRITE_SIZE * value as usize) as uptr;
        res.cpu.i.write(BusContext::Raw, 0, addr as udword);
    }

    fn bcd(&self, res: &mut Resources, inst: &RawInstruction) {
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

    fn save(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        for idx in 0..(x_index + 1) {
            let value = res.cpu.gpr[idx].read(BusContext::Raw, 0);
            let addr: uptr = res.cpu.i.read(BusContext::Raw, 0);
            res.cpu.i.write(BusContext::Raw, 0, (addr as udword) + 1);
            res.memory.write(BusContext::Raw, addr as usize, value);
        }
    }

    fn load(&self, res: &mut Resources, inst: &RawInstruction) {
        let x_index = inst.x_register();
        for idx in 0..(x_index + 1) {
            let addr: uptr = res.cpu.i.read(BusContext::Raw, 0);
            res.cpu.i.write(BusContext::Raw, 0, (addr as udword) + 1);
            let value = res.memory.read(BusContext::Raw, addr as usize);
            res.cpu.gpr[idx].write(BusContext::Raw, 0, value);
        }
    }
}