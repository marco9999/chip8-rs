#![feature(const_fn)]

extern crate num;
extern crate rand;
#[macro_use] 
extern crate serde_derive;
extern crate bincode;

pub mod common;
pub mod resources;
pub mod controller;

use std::sync::RwLock;
use resources::Resources;

struct Core {
    resources: RwLock<Resources>,
}

impl Core {
    fn resources(&self) -> &RwLock<Resources> {
        &self.resources
    }

    fn run(&self) {

    }
}