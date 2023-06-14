use crate::solution::Solution;
use crate::city::City;

pub struct BruteForce;

impl BruteForce {
    pub fn new() -> Self {
        Self {

        }
    }

    fn permutations(&self, cities: &Vec<City>) -> Vec<Vec<City>> {
        let mut permutations : Vec<Vec<City>> = vec![];

        let mut stack : Vec<usize> = vec![];
        let mut root_route = cities[1..].to_vec();
        let n = root_route.len();

        for _ in 0..n {
            stack.push(0);
        }

        permutations.push(root_route.clone());

        let mut i: usize = 0;

        while i < n {
            if stack[i] < i {
                if i % 2 == 0 {
                    root_route.swap(0, i);
                } else {
                    root_route.swap(stack[i], i);
                }

                permutations.push(root_route.clone());

                stack[i] += 1;

                i = 0;
            } else {
                stack[i] = 0;
                i += 1;
            }
        }

        return permutations;
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

#[cfg(test)]
mod solutions {
    use super::*;

    #[test]
    fn brute_force() {
        
        let city_a = City::new(0.0, 0.0);
        let city_b = City::new(10.0, 0.0);
        let city_c = City::new(5.0, 5.0);
        let city_d = City::new(15.0, 8.0);
        let city_e = City::new(8.0, 31.0);

        let mut cities = vec![city_a, city_b, city_c];

        let brute_force = BruteForce::new();

        let mut answer = brute_force.solve(&cities);

        assert_eq!(2, answer.len());

        cities.push(city_d);

        answer = brute_force.solve(&cities);

        assert_eq!(6, answer.len());

        cities.push(city_e);

        answer = brute_force.solve(&cities);

        assert_eq!(24, answer.len());
    }
}