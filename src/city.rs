use crate::drawable::Drawable;
use macroquad::color::BLACK;
use macroquad::math::Vec2;
use macroquad::shapes::draw_circle;

#[derive(Copy, Clone)]
pub struct City {
    position: Vec2,
    size: f32,
}

impl City {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            position: Vec2 { x, y },
            size,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn distance_between(&self, city: &City) -> f32 {
        self.position.distance(city.position())
    }
}

impl PartialEq for City {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Drawable for City {
    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.size, BLACK);
    }
}
