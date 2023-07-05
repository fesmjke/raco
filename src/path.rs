use raylib::prelude::{RaylibDraw,Color};
use crate::drawable::Drawable;
use crate::city::City;
use crate::utils::caclulate_distance;

pub struct Path {
    route : Vec<City>,
}

impl Path {
    pub fn new(cities: &Vec<City>) -> Self {
        Self {
            route : cities.to_vec() 
        }
    }

    pub fn total_distance(&self) -> f32 {
        caclulate_distance(&self.route)
    }

    pub fn replace(&mut self, route: &Vec<City>) {
        self.route = route.to_vec();
    }
}

impl Drawable for Path {
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