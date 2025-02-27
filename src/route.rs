use macroquad::color::BLACK;
use macroquad::shapes::draw_line;

use crate::city::City;
use crate::utils::route_length;
use crate::Rendereable;

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub(crate) cities: Vec<City>,
    length: f32,
}

impl Route {
    pub fn new(cities: Vec<City>) -> Self {
        let mut route = Self { cities, length: 0. };

        route.length = route_length(&route);

        route
    }

    pub fn route_length(&self) -> f32 {
        self.length
    }

    pub fn replace(&mut self, cities: Vec<City>) {
        self.cities = cities;
    }
}

impl Default for Route {
    fn default() -> Self {
        Self {
            cities: vec![],
            length: 0.,
        }
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
