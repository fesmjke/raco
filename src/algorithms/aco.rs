use crate::solution::Solution;
use crate::city::City;
use crate::utils::{adjacency_matrix, scale_distances, caclulate_distance, convert};

use rand::Rng;

pub struct Aco {
    ants: usize,
    iterations: usize,
}

struct Trail {
    table: Vec<Vec<f32>>,
    alpha: f32,
    beta: f32,
}

impl Trail {
    fn new() -> Self {
        Self {
            table : vec![],
            alpha: 1.0, 
            beta: 1.0 
        }
    }

    fn init_table(&mut self, pheromone : &Pheromone, distances: &Vec<Vec<f32>>) {
        self.table = vec![vec![0.0;distances.len()]; distances.len()];
        
        for (i, row) in self.table.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                if i == j {
                    continue;
                }
                
                *value = pheromone.table[i][j].powf(self.alpha) * distances[i][j].powf(self.beta);
            }
        }
    }

    fn reduce_trail(&mut self, visited: &Vec<usize>) {
        for (i, row) in self.table.iter_mut().enumerate() {    
            for (j, value) in row.iter_mut().enumerate() {
                if visited.contains(&j) {
                    *value = 0.0;
                    continue;
                }
    
                if i == j {
                    continue;
                }
            }
        }
    }
}

struct Pheromone {
    table: Vec<Vec<f32>>,
    pheromone_strength: f32,
    Q : f32,
    evaporation: f32
}

impl Pheromone {
    fn new() -> Self {
        Self { 
            table: vec![],
            pheromone_strength: 0.3,
            Q : 4.0,
            evaporation: 0.5,
        }
    }

    fn init_table(&mut self, size: usize) {
        for i in 0..size {
            let mut row = vec![];
    
            for j in 0..size {
                if i == j {
                    row.push(0.0);
                    continue;
                }
    
                row.push(self.pheromone_strength);
            }
    
            self.table.push(row);
        }
    }

    pub fn update_table(&mut self, route: &Vec<usize>, length: f32){
        let delta = self.Q / length;
        let mut current = route[0];

        let mut initial = true;

        for index in route.iter() {
            if initial {
                initial = false;
                continue;
            }
            
            self.table[current][*index] = ((1.0 - self.evaporation) * self.table[current][*index]) + delta;

            current = *index;
        }

        self.table[current][route[0]] = ((1.0 - self.evaporation) * self.table[current][route[0]]) + delta;
    }
}

struct Probability {
    table: Vec<Vec<f32>>
}

impl Probability {
    fn new() -> Self {
        Self { table: vec![] }
    }
    
    fn init_table(&mut self, size: usize) {
        for _ in 0..size {
            let mut row = vec![];
    
            for _ in 0..size {
                row.push(0.0);
            }
    
            self.table.push(row);
        }
    }

    fn update_table(&mut self, trail: &Trail) {
        for (i,row) in self.table.iter_mut().enumerate() {
            let row_sum: f32 = trail.table[i].iter().map(|&v| v as f32).sum();
    
            for (j,value) in row.iter_mut().enumerate() {
                if i == j {
                    continue;
                }
    
                if row_sum == 0.0 {
                    *value = 0.0;
                } else {
                    *value = trail.table[i][j] / row_sum;
                }
            }
        }
    }
}

impl Aco {
    pub fn new(ants: usize, iterations: usize) -> Self {    
        Self {
            ants,
            iterations,
        }
    }
}

impl Solution for Aco {
    fn solve(&self, cities : &Vec<City>) -> Vec<Vec<City>> {
        let mut routes = vec![];

        let mut rng = rand::thread_rng();

        let mut distances = adjacency_matrix(&cities);
        scale_distances(&mut distances, 100.0);
        
        let size = cities.len();
        
        let mut pheromone = Pheromone::new();
        let mut trails = Trail::new();
        let mut probability = Probability::new();
        
        probability.init_table(size);
        pheromone.init_table(size);
        trails.init_table(&pheromone, &distances);

        probability.update_table(&trails);

        for _ in 0..self.iterations {
            let mut visited_routes: Vec<Vec<usize>> = vec![];

            for ant in 0..self.ants {
                let mut visited : Vec<usize> = vec![]; 

                visited.push(ant);

                while visited.len() != self.ants {
                    let rd = rng.gen_range(0.0..1.0);

                    let mut sum = 0.0;

                    let last_visited = visited.last().unwrap();

                    for (i,pr) in probability.table[*last_visited].iter().enumerate() {
                        sum += pr;

                        if rd < sum {
                            trails.reduce_trail(&visited);
                            probability.update_table(&trails);
                            visited.push(i);
                            break;
                        }
                    }
                }

                visited_routes.push(visited);

                trails.init_table(&pheromone, &distances); // restore trails table after reduce trails
                probability.update_table(&trails); // restore probability table
            }
            
            // update pheromone table and routes
            for visited in visited_routes.iter() {
                let route = convert(visited, cities);

                let length = caclulate_distance(&route);

                pheromone.update_table(visited, length);

                routes.push(route);
            }
        }

        let mut temp : Vec<Vec<City>> = vec![];
        
        for route in routes.iter() {
            if route.starts_with(&[cities[0].clone()]) {
                temp.push(route.clone());
            }
        } 

        temp
    }
}

#[cfg(test)]
mod solutions {
    use super::*;

    #[test]
    fn initialize() {
        let city_a = City::new(0.0, 0.0);
        let city_b = City::new(10.0, 0.0);
        let city_c = City::new(5.0, 5.0);

        let cities = vec![city_a, city_b, city_c];
        let size = cities.len();

        let mut distances = adjacency_matrix(&cities);
        scale_distances(&mut distances, 100.0);

        let mut pheromone = Pheromone::new();
        let mut trails = Trail::new();
        let mut probability = Probability::new();
        
        probability.init_table(size);
        pheromone.init_table(size);
        trails.init_table(&pheromone, &distances);

        probability.update_table(&trails);

        assert_eq!(vec![vec![0.0, 0.58578646, 0.4142136], 
                        vec![0.58578646, 0.0, 0.4142136],
                        vec![0.5, 0.5 , 0.0]], probability.table);

        let mut visited : Vec<usize> = vec![0,2];

        trails.reduce_trail(&visited);

        probability.update_table(&trails);

        assert_eq!(vec![vec![0.0, 0.0, 0.0], 
                        vec![0.0, 0.0, 1.0],
                        vec![0.0, 1.0, 0.0]], probability.table);

        
        visited.push(1);

        pheromone.update_table(&visited, 24.14);

        assert_eq!(vec![vec![0.0, 0.3, 0.3157001], 
                        vec![0.3157001, 0.0, 0.3],
                        vec![0.3, 0.3157001, 0.0]], pheromone.table);
    }

    #[test]
    fn simple_aco() {
        let city_a = City::new(0.0, 0.0);
        let city_b = City::new(10.0, 0.0);
        let city_c = City::new(5.0, 5.0);

        let cities = vec![city_a, city_b, city_c];

        let solver = Aco::new(cities.len(), 100);

        let routes = solver.solve(&cities);

        assert_eq!(3, routes.len());
    }
}