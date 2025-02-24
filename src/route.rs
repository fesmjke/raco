use macroquad::color::BLACK;
use macroquad::shapes::draw_line;

use crate::city::City;
use crate::utils::route_length;
use crate::Rendereable;

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub(crate) cities: Vec<City>,
}

impl Route {
    pub fn new(cities: Vec<City>) -> Self {
        Self { cities }
    }

    pub fn path_length(&self) -> f32 {
        route_length(&self)
    }

    pub fn replace(&mut self, cities: Vec<City>) {
        self.cities = cities;
    }
}

impl Rendereable for Route {
    fn render(&self) {
        self.cities
            .iter()
            .zip(self.cities.iter().cycle().skip(1))
            .take(self.cities.len())
            .for_each(|(xf, xt)| {
                // render city
                xf.render();
                // render between
                draw_line(
                    xf.position().x,
                    xf.position().y,
                    xt.position().x,
                    xt.position().y,
                    1.0,
                    BLACK,
                );
            });
    }
}
