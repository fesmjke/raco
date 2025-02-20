use crate::city::City;
use crate::drawable::Drawable;
use crate::utils::route_length;

pub struct Route<'a> {
    pub(crate) cities: &'a Vec<City>,
}

impl<'a> Route<'a> {
    pub fn new(cities: &'a Vec<City>) -> Self {
        Self { cities: &cities }
    }

    pub fn path_length(&self) -> f32 {
        route_length(&self)
    }

    pub fn replace(&mut self, cities: &'a Vec<City>) {
        self.cities = &cities;
    }
}

impl<'a> Drawable for Route<'a> {
    fn draw(&self) {}
}
