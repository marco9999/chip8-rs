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
use resources::Resources;
use controller::Controller;
use controller::cpu::Cpu;
use controller::spu::Spu;
use controller::timer::Timer;

pub struct Core {
    resources: Box<UnsafeCell<Resources>>,
    controllers: Vec<Box<Controller>>,
}

impl Core {
    fn new() -> Core {
        Core {
            resources: Box::new(UnsafeCell::new(Resources::new())),
            controllers: Vec::new(),
        }
    }

    fn init(&mut self) {
        self.controllers.push(Box::new(Cpu::new(&self)));
        // self.controllers.push(Box::new(Spu::new(&self)));
        // self.controllers.push(Box::new(Timer::new(&self)));
    }

    fn resources(&self) -> &mut Resources {
        unsafe {
            &mut *self.resources.get()
        }
    }

    fn run(&self) {

    }
}