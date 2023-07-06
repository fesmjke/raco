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
            beta: 4.0 
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
            pheromone_strength: 1.0,
            Q : 4.0,
            evaporation: 0.3,
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

        for index in route.iter() {
            if *index == current {
                continue;
            }
            
            self.table[current][*index] = ((1.0 - self.evaporation) * self.table[current][*index]) + delta;
            self.table[*index][current] = ((1.0 - self.evaporation) * self.table[*index][current]) + delta;
            current = *index;
        }

        self.table[current][route[0]] = ((1.0 - self.evaporation) * self.table[current][route[0]]) + delta;
        self.table[route[0]][current] = ((1.0 - self.evaporation) * self.table[route[0]][current]) + delta;
    }

    pub fn best_route(&self, start: usize) -> Vec<usize> {
        let mut route = vec![];

        route.push(start);

        while route.len() != self.table.len() {
            let last = route.last().unwrap();
            let mut lowest = f32::MIN;
            let mut current = 0 as usize;
            for (index, value) in self.table[*last].iter().enumerate() {
                if route.contains(&index) {
                    continue;
                }

                if lowest < *value {
                    current = index;
                    lowest = *value;
                }
            }

            route.push(current);
        }

        route
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

            // update pheromone table
            for visited in visited_routes.iter() {
                let route = convert(visited, cities);

                let length = caclulate_distance(&route) / 100.0;

                pheromone.update_table(visited, length);

                routes.push(route);
            }

            // update trails and probability tables after pheromone update
            trails.init_table(&pheromone, &distances);
            probability.update_table(&trails);

            visited_routes.clear();
        }
        
        for ant in 0..self.ants {
            routes.push(convert(&pheromone.best_route(ant), cities));
        }
        
        routes
    }
}

#[cfg(test)]
mod solutions {
    use super::*;

    #[test]
    fn simple_aco() {
        let city_a = City::new(0.0, 0.0);
        let city_b = City::new(10.0, 0.0);
        let city_c = City::new(5.0, 5.0);
        let city_d = City::new(4.0, 8.0);
        let city_e = City::new(8.0, 3.0);


        let cities = vec![city_a, city_b, city_c, city_d, city_e];

        let solver = Aco::new(cities.len(), 100);

        let routes = solver.solve(&cities);

        assert_eq!(cities.len() * 100 + cities.len(), routes.len());
    }
}