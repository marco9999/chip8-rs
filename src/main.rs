#![feature(drop_types_in_const)]

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate futures_cpupool;
extern crate sdl2;

extern crate chip8_rs as chip8;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::VideoSubsystem;
use sdl2::render::WindowCanvas;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::render::Texture;
use futures_cpupool::CpuPool;
use chip8::Core;
use chip8::Config;
use chip8::common::constants::cpu::{HORIZONTAL_RES, VERTICAL_RES};

struct SdlContext {
    sdl_context: Option<Sdl>,
    _video_subsystem: Option<VideoSubsystem>,
    window_canvas: Option<WindowCanvas>,
    texture_creator: Option<TextureCreator<WindowContext>>,
    texture: Option<Texture<'static>>,
}

static mut SDL_CONTEXT: SdlContext = SdlContext {
    sdl_context: None,
    _video_subsystem: None,
    window_canvas: None,
    texture_creator: None,
    texture: None,
};

fn main() {
    init_sdl2();
    log4rs::init_file("./workspace/config/log.yml", Default::default()).unwrap();
    if cfg!(build = "debug") {
        info!("Started (debug)");
    } else if cfg!(build = "release") {
        info!("Started (release)")
    }

    let config = Config {
        workspace_path: "./workspace/".to_owned(),
        time_delta_us: 20000.0,
        multithreaded_pool: Some(CpuPool::new_num_cpus()),
        cpu_bias: 1.0, 
        spu_bias: 1.0,
        timer_bias: 1.0,
        video_callback: Some(render),
        audio_callback: None,
    };
    let mut core = Core::new(Some(config));
    core.reset("./workspace/roms/PONG").unwrap();

    unsafe {
        let event_pump = &mut SDL_CONTEXT.sdl_context.as_mut().unwrap().event_pump().unwrap();
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
    }
    
    if cfg!(build = "debug") {
        debug!("Memory dumped to workspace/dumps folder");
        core.debug_dump_all("_exit").unwrap();
    }
}

fn init_sdl2() {
    unsafe {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("chip8-rs", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let window_canvas = window.into_canvas().build().unwrap();
        let texture_creator = window_canvas.texture_creator();

        SDL_CONTEXT = SdlContext {
            sdl_context: Some(sdl_context),
            _video_subsystem: Some(video_subsystem),
            window_canvas: Some(window_canvas),
            texture_creator: Some(texture_creator),
            texture: None,
        };

        let texture = (&mut SDL_CONTEXT).texture_creator.as_mut().unwrap()
            .create_texture_streaming(PixelFormatEnum::RGB24, HORIZONTAL_RES, VERTICAL_RES).unwrap();
        SDL_CONTEXT.texture = Some(texture);
    }
}

fn render(framebuffer: &[bool; HORIZONTAL_RES * VERTICAL_RES]) { 
    unsafe {
        let sdl_context = &mut SDL_CONTEXT;
        sdl_context.window_canvas.as_mut().unwrap().clear();
        sdl_context.texture.as_mut().unwrap().with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..VERTICAL_RES {
                for x in 0..HORIZONTAL_RES {
                    let fb_index = y * HORIZONTAL_RES + x;
                    let tex_offset = (y * pitch) + (x * 3);
                    buffer[tex_offset] = (framebuffer[fb_index] as u8) * 255;
                    buffer[tex_offset + 1] = (framebuffer[fb_index] as u8) * 255;
                    buffer[tex_offset + 2] = (framebuffer[fb_index] as u8) * 255;
                }
            }
        }).unwrap(); 
        sdl_context.window_canvas.as_mut().unwrap().copy(&sdl_context.texture.as_mut().unwrap(), None, None).unwrap();
        sdl_context.window_canvas.as_mut().unwrap().present();
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