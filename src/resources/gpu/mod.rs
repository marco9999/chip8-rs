use common::constants::gpu::*;

pub struct GPU {
    pub framebuffer: [[bool; HORIZONTAL_RES]; VERTICAL_RES],
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            framebuffer: [[false; HORIZONTAL_RES]; VERTICAL_RES],
        }
    }
}