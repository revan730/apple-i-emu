#[macro_use]
extern crate lazy_static;

extern crate sdl2;

use memory::init_test_program_mem;
use sdl2::event::Event;

use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use mos_6502::cpu::Cpu;

use crate::{drawable::Drawable, memory::init_mem};

mod drawable;
mod keyboard;
mod memory;
mod screen;
mod sdl_renderer;

fn main() {
   //run_test_program();

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let sdl_renderer = sdl_renderer::SdlRenderer::new(sdl_context);

    let screen = Arc::new(Mutex::new(screen::Screen::new(Box::new(sdl_renderer))));
    let keyboard = Arc::new(Mutex::new(keyboard::Keyboard::default()));
    let memory = init_mem(screen.clone(), keyboard.clone());

    println!("{:?}", memory);
    let mut cpu = Cpu::new(memory);

    // TODO: Sync
    // TODO: For some reason when printing memory contents using wozmon, it doesn't stop at provided address and prints endlessly. seems like one of the opcodes is not implemented correctly

    cpu.reset();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    sdl2::keyboard::Keycode::A
                    | sdl2::keyboard::Keycode::B
                    | sdl2::keyboard::Keycode::C
                    | sdl2::keyboard::Keycode::E
                    | sdl2::keyboard::Keycode::F
                    | sdl2::keyboard::Keycode::R
                    | sdl2::keyboard::Keycode::Num0
                    | sdl2::keyboard::Keycode::Period => {
                        keyboard.lock().unwrap().handle_key(keycode.cast_unsigned() as u8);
                    }
                    sdl2::keyboard::Keycode::Backspace => {
                        keyboard.lock().unwrap().handle_key(0x5F);
                    }
                    sdl2::keyboard::Keycode::Return => {
                        keyboard.lock().unwrap().handle_key(b'\r');
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        cpu.step();
        //println!("Cpu state: {:?}", cpu);
        //sleep(Duration::from_millis(20));
    }
}

fn run_test_program() {
    let memory = init_test_program_mem();
    let mut cpu = Cpu::new(memory);

    cpu.reset();
    cpu.set_pc(0x400);
    loop {
        cpu.step();
        println!("Cpu state: {:?}", cpu);
        //sleep(Duration::from_millis(5));
    }
}
