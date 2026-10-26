use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    ttf::{Font, Sdl2TtfContext},
    video::{Window, WindowContext},
    Sdl,
};

use crate::{
    drawable::Drawable,
    screen::{COLUMNS, LINES},
};

lazy_static! {
    static ref TTF_CONTEXT: Sdl2TtfContext = sdl2::ttf::init().unwrap();
}

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 600;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub struct SdlRenderer {
    canvas: Canvas<Window>,
    font: Font<'static, 'static>,
    texture_creator: TextureCreator<WindowContext>,
}

impl SdlRenderer {
    pub fn new(sdl_context: Sdl) -> SdlRenderer {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Apple I", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .present_vsync()
            .build()
            .unwrap();
        canvas
            .set_logical_size(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .unwrap();

        let font = TTF_CONTEXT
            .load_font("roms/VT323-Regular.ttf", 128)
            .unwrap();

        let texture_creator = canvas.texture_creator();

        SdlRenderer {
            canvas,
            font,
            texture_creator,
        }
    }
}

impl Drawable for SdlRenderer {
    /*fn draw_old(&mut self, chars: &[u8]) {
        // TODO: Render one surface per line
        let mut utf_string_chars = Vec::new();
        for (ind, c) in chars.iter().enumerate() {
            if *c != 0 {
                utf_string_chars.push(*c);
                if ind > 0 && ind % 40 == 0 {
                    utf_string_chars.push(b'\n');
                }
            }
        }

        let utf_string = String::from_utf8(utf_string_chars).unwrap();
        println!("utf string: {}", utf_string);

        if utf_string.len() > 0 {
            let texture_creator = &self.texture_creator;
            let surface = self
                .font
                .render(&utf_string)
                .blended(Color::RGBA(0, 255, 0, 255))
                .unwrap();

            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();

            let line_width = utf_string.len() * 8;

            let target = Rect::new(0, 0, line_width as u32, 32);

            self.canvas.copy(&texture, None, Some(target)).unwrap();
        }
    }*/

    fn draw(&mut self, chars: &[u8]) {
        for i in 0..LINES {
            let mut utf_string_chars = Vec::new();
            for c in chars.iter().skip(COLUMNS * i).take(COLUMNS) {
                if *c == 0xD {
                    utf_string_chars.push(b'\\');
                } else if *c != 0 {
                    utf_string_chars.push(*c);
                } else {
                    utf_string_chars.push(b' ');
                }
            }

            let utf_string = String::from_utf8(utf_string_chars).unwrap();
            println!("utf string: {}", utf_string);

            let texture_creator = &self.texture_creator;
            let surface = self
                .font
                .render(&utf_string)
                .blended(Color::RGBA(0, 255, 0, 255))
                .unwrap();

            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();

            let line_width = utf_string.len() * 8;

            let target = Rect::new(0, 32 * i as i32, line_width as u32, 32);

            self.canvas.copy(&texture, None, Some(target)).unwrap();
        }
    }

    fn cls(&mut self) {
        todo!()
    }

    fn render(&mut self) {
        self.canvas.present();
    }
}
