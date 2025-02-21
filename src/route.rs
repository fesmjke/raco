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
    fn render(&self) {}
}
