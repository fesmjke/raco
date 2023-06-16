use crate::solution::Solution;
use crate::city::City;

pub struct NearestNeighbour;

impl NearestNeighbour {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Solution for NearestNeighbour {
    fn solve(&self, cities : &Vec<City>) -> Vec<Vec<City>> {
        todo!()
    }
}

#[cfg(test)]
mod solutions {
    use super::*;

    #[test]
    fn nn() {
        let city_a = City::new(0.0, 0.0);
        let city_b = City::new(10.0, 0.0);
        let city_c = City::new(5.0, 5.0);
        let city_d = City::new(15.0, 8.0);
        let city_e = City::new(8.0, 31.0);

        let mut cities = vec![city_a, city_b, city_c];

        let nn = NearestNeighbour::new();

        let mut answer = nn.solve(&cities);
    }
}