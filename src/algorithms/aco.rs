use crate::solution::Solution;
use crate::city::City;
use crate::utils::{adjacency_matrix, scale_distances};

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
        let size = self.table.len();

        for (i, row) in self.table.iter_mut().enumerate() {
            if visited.contains(&i) {
                *row = vec![0.0; size];
                continue;
            }
    
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
}

impl Pheromone {
    fn new() -> Self {
        Self { 
            table: vec![],
            pheromone_strength: 0.3
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

    pub fn update_table(){
        todo!()
    }
}

struct Probability {
    table: Vec<Vec<f32>>
}

impl Probability {
    fn new() -> Self {
        Self { table: vec![] }
    }
    
    fn initialize_table(&mut self, size: usize) {
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
        todo!()
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
        let city_d = City::new(15.0, 8.0);
        let city_e = City::new(8.0, 31.0);

        let mut cities = vec![city_a, city_b, city_c];
        let size = cities.len();

        let solver = Aco::new(cities.len(), 100);

        let mut distances = adjacency_matrix(&cities);
        scale_distances(&mut distances, 100.0);

        let mut pheromone = Pheromone::new();
        let mut trails = Trail::new();
        let mut probability = Probability::new();
        
        probability.initialize_table(size);
        pheromone.init_table(size);
        trails.init_table(&pheromone, &distances);

        probability.update_table(&trails);

        assert_eq!(vec![vec![0.0, 0.58578646, 0.4142136], 
                        vec![0.58578646, 0.0, 0.4142136],
                        vec![0.5, 0.5 , 0.0]], probability.table);

        let visited : Vec<usize> = vec![0];

        trails.reduce_trail(&visited);

        probability.update_table(&trails);

        assert_eq!(vec![vec![0.0, 0.0, 0.0], 
                        vec![0.0, 0.0, 1.0],
                        vec![0.0, 1.0, 0.0]], probability.table);
    }
}