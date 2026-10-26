pub trait Drawable {
    fn draw(&mut self, chars: &[u8]);
    fn cls(&mut self);
    fn render(&mut self);
}
