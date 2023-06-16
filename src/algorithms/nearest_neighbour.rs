use crate::solution::Solution;
use crate::city::City;

pub struct NearestNeighbour;

impl NearestNeighbour {
    pub fn new() -> Self {
        Self {

        }
    }

    fn calculate_distance(&self, from: &City, to: &City) -> f32 {
        from.position().distance_to(*to.position())
    }

    fn select_nearest(&self, from : &City, left: &Vec<City>) -> usize {
        let mut distance = self.calculate_distance(from, &left[0]);
        let mut lowest = 0 as usize;

        for (index,city) in left.iter().enumerate() {
            let temp = self.calculate_distance(from, city);

            if temp < distance {    
                distance = temp;
                lowest = index;
            }
        }

        lowest
    }
}

impl Solution for NearestNeighbour {
    fn solve(&self, cities : &Vec<City>) -> Vec<Vec<City>> {
        let mut routes : Vec<Vec<City>> = vec![];

        let root_route = cities[1..].to_vec().clone();

        for (index,city) in root_route.iter().enumerate() {
            let mut temp = root_route.clone();
            let mut route : Vec<City> = vec![];
            let mut lowest = 0 as usize;
            let mut start = city.clone();

            route.push(temp[index].clone());

            temp.remove(index);

            while temp.len() > 0 {
                lowest = self.select_nearest(&start, &temp);

                route.push(temp[lowest].clone());

                start = temp[lowest].clone();

                temp.remove(lowest);
            }

            routes.push(route);
        }

        routes
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

        assert_eq!(2, answer.len());

        cities.push(city_d);

        answer = nn.solve(&cities);

        assert_eq!(3, answer.len());

        cities.push(city_e);

        answer = nn.solve(&cities);

        assert_eq!(4, answer.len());
    }
}