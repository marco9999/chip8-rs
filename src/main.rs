extern crate chip8_rs as core;

use core::Core;

fn main() {
    let mut core = Core::new();
    core.reset("./workspace/roms/PONG").unwrap();

    loop {
        core.run().unwrap();
    }
}
