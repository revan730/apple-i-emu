use crate::drawable::Drawable;

pub const LINES: usize = 24;
pub const COLUMNS: usize = 40;

pub struct Screen {
    chars: [u8; COLUMNS * LINES],
    x: usize,
    y: usize,
    cr: u8,
    dsp: u8,
    output: bool,
    renderer: Box<dyn Drawable>,
}

impl Screen {
    pub fn new(renderer: Box<dyn Drawable>) -> Screen {
        Screen {
            chars: [0; COLUMNS * LINES],
            x: 0,
            y: 0,
            cr: 0,
            dsp: 0,
            output: false,
            renderer,
        }
    }

    fn draw_screen(&mut self) {
        self.renderer.draw(&self.chars);
        self.renderer.render();
    }

    fn new_line(&self) {
        println!("{:?}", self.chars);
        todo!("Implement new line");
    }

    pub fn write(&mut self, c: u8) {
        let char = if c >= 0x80 { c - 0x80 } else { c };

        match char {
            0x5F => {
                if self.x == 0 {
                    self.y -= 1;
                    self.x = COLUMNS - 1;
                } else {
                    self.x -= 1;
                }

                self.chars[self.y * COLUMNS + self.x] = 0;
            }
            0x0A | 0x0D => {
                self.x = 0;
                self.y += 1;
            }
            0x00 | 0x7F => (),
            _ => {
                self.chars[self.y * COLUMNS + self.x] = char;
                self.x += 1;
            }
        }

        if self.x == COLUMNS {
            self.x = 0;
            self.y += 1;
        }

        if self.y == LINES {
            self.y = 0;
            self.new_line();
        }

        self.draw_screen();
        self.dsp = char;
    }

    pub fn write_cr(&mut self, cr: u8) {
        if !self.output && cr >= 0x80 {
            self.output = true;
        } else {
            self.cr = cr;
        }
    }

    pub fn read(&self) -> u8 {
        self.dsp
    }

    pub fn read_cr(&self) -> u8 {
        self.cr
    }

    pub fn clear(&mut self) {
        self.chars = [0; COLUMNS * LINES];
        self.dsp = 0;
        self.cr = 0;
        self.x = 0;
        self.y = 0;
    }
}
