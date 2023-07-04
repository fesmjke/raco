use raylib::prelude::{RaylibDraw,Color,Vector2};
use crate::drawable::Drawable;

#[derive(Clone)]
pub struct City {
    position : Vector2,
    size: f32, 
}

impl City {
    pub fn new(x : f32, y: f32) -> Self {
        Self {
            position : Vector2 { x, y },
            size: 5.0
        }
    }

    pub fn position(&self) -> &Vector2 {
        &self.position
    }
}

impl PartialEq for City {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Drawable for City {
    fn draw<T: RaylibDraw>(&self, d : &mut T) {
        d.draw_circle_v(self.position, self.size, Color::BLACK);
    }
}