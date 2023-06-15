use raylib::prelude::RaylibDraw;

pub enum DrawState {
    Drawing,
    Finished
}

pub trait Drawable {
    fn draw<T: RaylibDraw>(&self, d : &mut T) {}
    fn draw_mut<T: RaylibDraw>(&mut self, d : &mut T) -> DrawState { DrawState::Finished }
}