use raylib::prelude::RaylibDraw;

pub trait Drawable {
    fn draw<T: RaylibDraw>(&self, d : &mut T) {}
}