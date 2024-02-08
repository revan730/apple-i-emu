use std::sync::{Arc, Mutex};

use mos_6502::cpu::Cpu;

use crate::memory::init_mem;

mod keyboard;
mod memory;
mod screen;

fn main() {
    let screen = Arc::new(Mutex::new(screen::Screen::default()));
    let keyboard = Arc::new(Mutex::new(keyboard::Keyboard::default()));
    let memory = init_mem(screen.clone(), keyboard.clone());

    println!("{:?}", memory);
    let mut cpu = Cpu::new(memory);

    // TODO: Sync

    cpu.reset();
    /*cpu.step();
    println!("Cpu state: {:?}", cpu);

    cpu.step();
    println!("Cpu state: {:?}", cpu);

    // LDY
    cpu.step();
    println!("Cpu state: {:?}", cpu);

    // STY DSP
    cpu.step();
    println!("Cpu state: {:?}", cpu);*/

    loop {
        cpu.step();
        println!("Cpu state: {:?}", cpu);
    }
}
