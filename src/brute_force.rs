use crate::solution::Solution;
use crate::city::City;

struct BruteForce;

impl BruteForce {
    pub fn new() -> Self {
        Self {

        }
    }

    fn permutations(&self, cities: &Vec<City>) -> Vec<Vec<City>> {
        todo!()
    }
}

impl Solution for BruteForce {
    fn solve(&self, cities : &Vec<City>) -> Vec<Vec<City>> {
        // Adjacency matrix
        let mut matrix : Vec<Vec<f32>> = vec![];
        
        for i in 0..cities.len() {
            
            let mut temp : Vec<f32> = vec![];

            for j in 0..cities.len() {

                let distance = cities[i].position.distance_to(cities[j].position) as f32;

                temp.push(distance);
            }

            matrix.push(temp);
        }

        let mut root_route: Vec<City> = vec![];

        for city in cities.iter() {
            root_route.push(city.clone());
        }

        let mut routes: Vec<Vec<City>> = vec![];

        routes = self.permutations(&root_route);

        return routes;
    }
}