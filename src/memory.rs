use std::{
    fs, io,
    sync::{Arc, Mutex},
};

use mos_6502::memory_bus::{MemoryBus, MemoryRegion};

use crate::{
    keyboard::{self, Keyboard},
    screen::Screen,
};

const ZERO_PAGE_START: usize = 0x00;
pub const ZERO_PAGE_END: usize = 0xFF;
pub const ZERO_PAGE_SIZE: usize = ZERO_PAGE_END - ZERO_PAGE_START + 1;

const DATA_STACK_START: usize = 0x100;
const DATA_STACK_END: usize = 0x1FF;
const DATA_STACK_SIZE: usize = DATA_STACK_END - DATA_STACK_START + 1;

const RAM_IO_ROM_START: usize = 0x200;
const RAM_IO_ROM_END: usize = 0xFFF9;
const RAM_IO_ROM_SIZE: usize = RAM_IO_ROM_END - RAM_IO_ROM_START + 1;

const NMI_START: usize = 0xFFFA;
const NMI_END: usize = 0xFFFB;

const RESET_START: usize = 0xFFFC;
const RESET_END: usize = 0xFFFD;

const IRQ_START: usize = 0xFFFE;
const IRQ_END: usize = 0xFFFF;

const VECTOR_SIZE: usize = 2;

const RAM_LOW_START: usize = ZERO_PAGE_START;
const RAM_LOW_END: usize = 0xFFF;
const RAM_LOW_SIZE: usize = RAM_LOW_END - RAM_LOW_START + 1;

const RAM_HIGH_START: usize = 0xE000;
const RAM_HIGH_END: usize = 0xEFFF;

const ROM_START: usize = 0xFF00;
const ROM_END: usize = MEM_SPACE_END;

const MEM_SPACE_END: usize = 0xFFFF;

const KBD: usize = 0xD010;
const KBDCR: usize = 0xD011;
const DSP: usize = 0xD012;
const DSPCR: usize = 0xD013;

static mut RAM_STORAGE: [u8; 8192] = [0; 8192];
static mut ROM_STORAGE: [u8; 256] = [0; 256];

pub fn init_mem(screen: Arc<Mutex<Screen>>, keyboard: Arc<Mutex<Keyboard>>) -> MemoryBus {
    let mut mem = MemoryBus::new();

    let ram_region_low = MemoryRegion {
        start: RAM_LOW_START,
        end: RAM_LOW_END,
        read_handler: Box::new(|address| unsafe { RAM_STORAGE[address] }),
        write_handler: Box::new(|address, value| unsafe {
            RAM_STORAGE[address] = value;
        }),
    };

    mem.add_region(ram_region_low);

    let ram_region_high = MemoryRegion {
        start: RAM_HIGH_START,
        end: RAM_HIGH_END,
        read_handler: Box::new(|address| unsafe { RAM_STORAGE[RAM_LOW_SIZE + address] }),
        write_handler: Box::new(|address, value| unsafe {
            RAM_STORAGE[RAM_LOW_SIZE + address] = value;
        }),
    };

    mem.add_region(ram_region_high);

    let rom_region = MemoryRegion {
        start: ROM_START,
        end: ROM_END,
        read_handler: Box::new(|address| unsafe { ROM_STORAGE[address] }),
        write_handler: Box::new(|address, value| {
            panic!("Attempt to write {value:#X} to address {address:#X} of ROM")
        }), // TODO: Maybe log this instead of panicking
    };

    mem.add_region(rom_region);

    let kbd_read_handler = (|| {
        let _keyboard = keyboard.clone();

        Box::new(move |address: usize| match address {
            0x0 => _keyboard.lock().unwrap().read_kbd(),
            0x1 => _keyboard.lock().unwrap().read_cr(),
            _ => todo!(),
        })
    })();

    let kbd_region = MemoryRegion {
        start: KBD,
        end: KBDCR,
        read_handler: kbd_read_handler,
        write_handler: Box::new(move |address, value| match address {
            0x0 => todo!(),
            0x1 => keyboard.lock().unwrap().write_cr(value),
            _ => todo!(),
        }),
    };

    mem.add_region(kbd_region);

    let dsp_region = MemoryRegion {
        start: DSP,
        end: DSPCR,
        read_handler: Box::new(|_| 0x0), // It won't be read on real hardware but the way
        // i implemented it, fetch_operand always reads from memory
        write_handler: Box::new(move |address, value| match address {
            0x0 => screen.lock().unwrap().write(value),
            0x1 => screen.lock().unwrap().write_cr(value),
            _ => todo!(),
        }),
    };

    mem.add_region(dsp_region);

    unsafe {
        load_wozmon();
        load_basic();
    }

    mem
}

fn load_file(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}

unsafe fn load_wozmon() {
    let wozmon_builtin = include_bytes!("../roms/wozmon.rom");
    let wozmon = wozmon_builtin.to_vec();
    ROM_STORAGE[..wozmon.len()].copy_from_slice(&wozmon);

    println!(
        "Reset vec {:#X} {:#X}",
        ROM_STORAGE[0xFC], ROM_STORAGE[0xFD]
    );
}

unsafe fn load_basic() {
    let basic_builtin = include_bytes!("../roms/basic.rom");
    let basic = basic_builtin.to_vec();
    RAM_STORAGE[RAM_LOW_SIZE..(basic.len() + RAM_LOW_SIZE)].copy_from_slice(&basic);
}
