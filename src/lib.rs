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

pub struct Core {
    resources: UnsafeCell<Resources>,
}

impl Core {
    fn resources(&self) -> &mut Resources {
        unsafe {
            &mut *self.resources.get()
        }
    }

    fn run(&self) {

    }
}