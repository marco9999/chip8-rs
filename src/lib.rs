#![feature(const_fn)]

extern crate num;
extern crate rand;
#[macro_use] 
extern crate serde_derive;
extern crate bincode;

pub mod common;
pub mod resources;
pub mod controller;

use std::cell::UnsafeCell;
use common::types::storage::*;
use resources::Resources;
use controller::Controller;
use controller::cpu::Cpu;
use controller::spu::Spu;
use controller::timer::Timer;

pub struct Config {
    pub workspace_path: String,
    pub time_delta_us: f64,
    pub cpu_bias: f64,
    pub spu_bias: f64,
    pub timer_bias: f64,
}

pub struct Core {
    resources: Box<UnsafeCell<Resources>>,
    controllers: Vec<Box<Controller>>,
    pub config: Config,
}

impl Core {
    pub fn new() -> Core {
        Core {
            resources: Box::new(UnsafeCell::new(Resources::new())),
            controllers: Vec::new(),
            config: Config {
                workspace_path: "./workspace/".to_owned(),
                time_delta_us: 20000.0,
                cpu_bias: 1.0, 
                spu_bias: 1.0,
                timer_bias: 1.0,
            }
        }
    }

    pub fn new_config(config: Config) -> Core {
        Core {
            resources: Box::new(UnsafeCell::new(Resources::new())),
            controllers: Vec::new(),
            config: config,
        }
    }

    pub fn reset(&mut self, rom_path: &str) -> Result<(), String> {
        let self_ptr =  self as *const Core;

        unsafe {
            self.controllers.push(Box::new(Cpu::new(&*self_ptr)));
            self.controllers.push(Box::new(Spu::new(&*self_ptr)));
            self.controllers.push(Box::new(Timer::new(&*self_ptr)));
        }

        self.load_font_set();

        if let Err(_) = self.resources().memory.read_file(0x200, rom_path) {
            return Err("Something went wrong loading rom file.".to_owned());
        }

        Ok(())
    }

    pub fn run(&self) -> Result<(), String> {
        static mut TIME_US: f64 = 0.0;

        unsafe {
            TIME_US += self.config.time_delta_us;
            println!("Emulated time elapsed (s) = {:.6}", TIME_US / 1e6);
        }

        for ref cont in self.controllers.iter() {
            cont.gen_tick_event(self.config.time_delta_us);
        }

        for ref cont in self.controllers.iter() {
            cont.run();
        }

        Ok(())
    }

    pub fn debug_dump_all(&self, postfix_tag: &str) -> Result<(), String> {
        if let Err(_) = self.resources().memory.dump_file(&self.workspace_path(&format!("dumps/memory{}.bin", postfix_tag))) {
            return Err("Something went wrong writing the memory dump file.".to_owned());
        }

        Ok(())
    }

    fn workspace_path(&self, rel_path: &str) -> String {
        self.config.workspace_path.clone() + rel_path
    }

    fn resources(&self) -> &mut Resources {
        unsafe {
            &mut *self.resources.get()
        }
    }

    fn load_font_set(&self) {
        let char_0: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0];
        let char_1: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
        let char_2: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
        let char_3: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
        let char_4: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
        let char_5: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
        let char_6: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
        let char_7: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
        let char_8: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
        let char_9: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
        let char_a: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
        let char_b: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
        let char_c: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
        let char_d: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
        let char_e: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
        let char_f: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];

        self.resources().memory.write_slice(BusContext::Raw, 0x0, &char_0);
        self.resources().memory.write_slice(BusContext::Raw, 0x5, &char_1);
        self.resources().memory.write_slice(BusContext::Raw, 0xA, &char_2);
        self.resources().memory.write_slice(BusContext::Raw, 0xF, &char_3);
        self.resources().memory.write_slice(BusContext::Raw, 0x14, &char_4);
        self.resources().memory.write_slice(BusContext::Raw, 0x19, &char_5);
        self.resources().memory.write_slice(BusContext::Raw, 0x1E, &char_6);
        self.resources().memory.write_slice(BusContext::Raw, 0x23, &char_7);
        self.resources().memory.write_slice(BusContext::Raw, 0x28, &char_8);
        self.resources().memory.write_slice(BusContext::Raw, 0x2D, &char_9);
        self.resources().memory.write_slice(BusContext::Raw, 0x32, &char_a);
        self.resources().memory.write_slice(BusContext::Raw, 0x37, &char_b);
        self.resources().memory.write_slice(BusContext::Raw, 0x3C, &char_c);
        self.resources().memory.write_slice(BusContext::Raw, 0x41, &char_d);
        self.resources().memory.write_slice(BusContext::Raw, 0x46, &char_e);
        self.resources().memory.write_slice(BusContext::Raw, 0x4B, &char_f);
    }
}
