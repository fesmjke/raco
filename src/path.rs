use raylib::prelude::{RaylibDraw,Color};
use crate::drawable::{Drawable, DrawState};
use crate::city::City;

pub struct Path {
    route : Vec<City>,
    root : City,
    draw_index: usize
}

impl Path {
    pub fn new(cities: Vec<City>, from : City) -> Self {
        Self {
            route : cities,
            root : from,
            draw_index : 0
        }
    }

    pub fn total_distance(&self) -> f32 {
        let mut prev = self.root.clone();
        let mut total_distance = 0.0;
        
        for city in self.route.iter() {
            total_distance += prev.position().distance_to(*city.position());

            prev = city.clone();
        }

        total_distance += prev.position().distance_to(*self.root.position());

        total_distance
    }

    fn next(&mut self,) -> DrawState {
        if self.draw_index == self.route.len() {
            
            return DrawState::Finished;
        } else {
            self.draw_index += 1;
            return DrawState::Drawing;
        }
    }
}

impl Drawable for Path {
    fn draw<T: RaylibDraw>(&self, d : &mut T) {
        let mut prev_city = self.root.clone();

        self.root.draw(d);

        d.draw_text(format!("0").as_str(), self.root.position().x as i32, self.root.position().y as i32, 20, Color::RED);

        for (index,city) in self.route.iter().enumerate() {
            city.draw(d);
            d.draw_text(format!("{}", index + 1).as_str(), city.position().x as i32, city.position().y as i32, 20, Color::RED);
        }

        for city in self.route.iter() {
            d.draw_line_v(prev_city.position(), city.position(), Color::BLACK);
            prev_city = city.clone();
        }
        
        d.draw_line_v(prev_city.position(), self.root.position(), Color::BLACK);
    }

    fn draw_mut<T: RaylibDraw>(&mut self, d : &mut T) -> DrawState {
        let mut prev_city = self.root.clone();

        self.root.draw(d);

        for city in self.route.iter() {
            city.draw(d);
        }

        for (index,city) in self.route.iter().enumerate() {
            if self.draw_index > index {
                d.draw_line_v(prev_city.position(), city.position(), Color::BLACK);
                prev_city = city.clone();
            }
        }
        
        if self.draw_index == self.route.len() {
             d.draw_line_v(prev_city.position(), self.root.position(), Color::BLACK);
         }

        return self.next();
    }
}