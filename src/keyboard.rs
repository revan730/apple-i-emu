pub struct Keyboard {
    cr: u8,
    kbd: u8,
    kbd_interrupts: bool,
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Keyboard {
    fn new() -> Keyboard {
        Keyboard {
            cr: 0,
            kbd: 0,
            kbd_interrupts: false,
        }
    }

    pub fn write_kbd(&mut self, kbd: u8) {
        self.kbd = kbd;
    }

    pub fn read_kbd(&mut self) -> u8 {
        self.kbd
    }

    pub fn write_cr(&mut self, cr: u8) {
        if !self.kbd_interrupts && cr >= 0x80 {
            self.kbd_interrupts = true;
        } else {
            self.cr = cr;
        }
    }

    pub fn read_cr(&mut self) -> u8 {
        if self.kbd_interrupts && self.cr >= 0x80 {
            self.cr = 0;

            0xA7
        } else {
            self.cr
        }
    }
}
