use raylib::prelude::{RaylibDraw,Color};
use crate::drawable::Drawable;
use crate::city::City;
use crate::utils::caclulate_distance;

pub struct Path<'a> {
    route : &'a Vec<City>,
}

impl<'a> Path<'a> {
    pub fn new(cities: &'a Vec<City>) -> Self {
        Self {
            route : cities 
        }
    }

    pub fn total_distance(&self) -> f32 {
        caclulate_distance(self.route)
    }

    pub fn replace(&mut self, route: &'a Vec<City>) {
        self.route = route;
    }
}

impl<'a> Drawable for Path<'a> {
    fn draw<T: RaylibDraw>(&self, d : &mut T) {
        let root = self.route.first().expect("missing root city in a route");

        root.draw(d);

        for (index,city) in self.route.iter().enumerate() {
            city.draw(d);
            d.draw_text(format!("{}", index).as_str(), city.position().x as i32, city.position().y as i32, 20, Color::RED);
        }

        let mut prev = root;

        for city in self.route.iter() {
            d.draw_line_v(prev.position(), city.position(), Color::BLACK);
            prev = city;
        }
        
        d.draw_line_v(prev.position(), root.position(), Color::BLACK);
    }
}