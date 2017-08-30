#[macro_use]
extern crate log;
extern crate log4rs;
extern crate futures_cpupool;
extern crate sdl2;

extern crate chip8_rs as chip8;

use std::cell::RefCell;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use futures_cpupool::CpuPool;
use chip8::Core;
use chip8::Config;

fn main() {
    log4rs::init_file("./workspace/config/log.yml", Default::default()).unwrap();
    if cfg!(build = "debug") {
        info!("Started (debug)");
    } else if cfg!(build = "release") {
        info!("Started (release)")
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("chip8-rs", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = RefCell::new(window.into_canvas().build().unwrap());
    let texture_creator = *canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, 64, 32).unwrap();
    
    let render = |framebuffer: &[bool; 64 * 32]| { 
        *canvas.clear();
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..32 {
                for x in 0..64 {
                    let index = y * 32 + x;
                    let offset = y*pitch + x*3;
                    buffer[offset] = (framebuffer[index] as u8) * 255;
                    buffer[offset + 1] = (framebuffer[index] as u8) * 255;
                    buffer[offset + 2] = (framebuffer[index] as u8) * 255;
                }
            }
        }).unwrap(); 
        *canvas.copy(&texture, None, Some(Rect::new(0, 0, 64, 32))).unwrap();
        *canvas.present();
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    let config = Config {
        workspace_path: "./workspace/".to_owned(),
        time_delta_us: 20000.0,
        multithreaded_pool: Some(CpuPool::new_num_cpus()),
        cpu_bias: 1.0, 
        spu_bias: 1.0,
        timer_bias: 1.0,
        video_callback: Some(Box::new(render)),
        audio_callback: None,
    };
    let mut core = Core::new(Some(config));
    core.reset("./workspace/roms/PONG").unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyUp {..} | Event::KeyDown {..} => {
                    if let Err(e) = send_key_event(&core, event) {
                        error!("Encountered error (exiting): {}", e);
                    }
                },
                _ => {}
            }
        }

        if let Err(e) = core.run() {
            error!("Encountered error (exiting): {}", e);
            break 'running;
        }
    }
    
    if cfg!(build = "debug") {
        debug!("Memory dumped to workspace/dumps folder");
        core.debug_dump_all("_exit").unwrap();
    }
}

fn send_key_event(core: &Core, event: Event) -> Result<(), String> {
    match event {
        Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
            core.set_key(0x1, true)
        },
        Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {
            core.set_key(0x1, false)
        },
        Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
            core.set_key(0x2, true)
        },
        Event::KeyUp { keycode: Some(Keycode::Num2), .. } => {
            core.set_key(0x2, false)
        },
        Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
            core.set_key(0x3, true)
        },
        Event::KeyUp { keycode: Some(Keycode::Num3), .. } => {
            core.set_key(0x3, false)
        },
        Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
            core.set_key(0xC, true)
        },
        Event::KeyUp { keycode: Some(Keycode::Num4), .. } => {
            core.set_key(0xC, false)
        },
        Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
            core.set_key(0x4, true)
        },
        Event::KeyUp { keycode: Some(Keycode::Q), .. } => {
            core.set_key(0x4, false)
        },
        Event::KeyDown { keycode: Some(Keycode::W), .. } => {
            core.set_key(0x5, true)
        },
        Event::KeyUp { keycode: Some(Keycode::W), .. } => {
            core.set_key(0x5, false)
        },
        Event::KeyDown { keycode: Some(Keycode::E), .. } => {
            core.set_key(0x6, true)
        },
        Event::KeyUp { keycode: Some(Keycode::E), .. } => {
            core.set_key(0x6, false)
        },
        Event::KeyDown { keycode: Some(Keycode::R), .. } => {
            core.set_key(0xD, true)
        },
        Event::KeyUp { keycode: Some(Keycode::R), .. } => {
            core.set_key(0xD, false)
        },
        Event::KeyDown { keycode: Some(Keycode::A), .. } => {
            core.set_key(0x7, true)
        },
        Event::KeyUp { keycode: Some(Keycode::A), .. } => {
            core.set_key(0x7, false)
        },
        Event::KeyDown { keycode: Some(Keycode::S), .. } => {
            core.set_key(0x8, true)
        },
        Event::KeyUp { keycode: Some(Keycode::S), .. } => {
            core.set_key(0x8, false)
        },
        Event::KeyDown { keycode: Some(Keycode::D), .. } => {
            core.set_key(0x9, true)
        },
        Event::KeyUp { keycode: Some(Keycode::D), .. } => {
            core.set_key(0x9, false)
        },
        Event::KeyDown { keycode: Some(Keycode::F), .. } => {
            core.set_key(0xE, true)
        },
        Event::KeyUp { keycode: Some(Keycode::F), .. } => {
            core.set_key(0xE, false)
        },
        Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
            core.set_key(0xA, true)
        },
        Event::KeyUp { keycode: Some(Keycode::Z), .. } => {
            core.set_key(0xA, false)
        },
        Event::KeyDown { keycode: Some(Keycode::X), .. } => {
            core.set_key(0x0, true)
        },
        Event::KeyUp { keycode: Some(Keycode::X), .. } => {
            core.set_key(0x0, false)
        },
        Event::KeyDown { keycode: Some(Keycode::C), .. } => {
            core.set_key(0xB, true)
        },
        Event::KeyUp { keycode: Some(Keycode::C), .. } => {
            core.set_key(0xB, false)
        },
        Event::KeyDown { keycode: Some(Keycode::V), .. } => {
            core.set_key(0xF, true)
        },
        Event::KeyUp { keycode: Some(Keycode::V), .. } => {
            core.set_key(0xF, false)
        },
        _ => { Ok(()) }
    }
}