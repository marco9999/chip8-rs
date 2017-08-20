#![feature(const_fn)]

extern crate num;
extern crate rand;
#[macro_use] 
extern crate serde_derive;
extern crate bincode;

pub mod common;
pub mod resources;
pub mod controller;

struct Core {

}