const LINES: usize = 24;
const COLUMNS: usize = 40;

pub struct Screen {
    chars: [u8; COLUMNS * LINES],
    x: usize,
    y: usize,
    cr: u8,
    output: bool,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            chars: [0; COLUMNS * LINES],
            x: 0,
            y: 0,
            cr: 0,
            output: false,
        }
    }

    fn draw_screen(&self) {
        println!("{:?}", self.chars);
        //todo!("Implement screen rendering");
    }

    fn new_line(&self) {
        todo!("Implement new line");
    }

    pub fn write(&mut self, c: u8) {
        match c {
            0x5F => {
                if self.x == 0 {
                    self.y -= 1;
                    self.x = COLUMNS - 1;
                } else {
                    self.x -= 1;
                }

                self.chars[self.y * COLUMNS + self.x] = 0;
            }
            0x7F => (),
            _ => panic!("Unknown character: {}", c),
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
    }

    pub fn write_cr(&mut self, cr: u8) {
        if !self.output && cr >= 0x80 {
            self.output = true;
        } else {
            self.cr = cr;
        }
    }

    pub fn clear(&mut self) {
        self.chars = [0; COLUMNS * LINES];
        self.x = 0;
        self.y = 0;
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
