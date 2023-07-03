use crate::solution::Solution;
use crate::city::City;
use crate::utils::adjacency_matrix;

pub struct Aco {
    ants: usize,
    iterations: usize,
    alpha: f32,
    beta: f32,
    pheromone_default: f32,
}

fn initialize_pheromone_table(size: usize, pheromone_default: f32) -> Vec<Vec<f32>> {
    let mut pheromone_table = vec![]; 

    for i in 0..size {
        let mut row = vec![];

        for j in 0..size {
            if i == j {
                row.push(0.0);
                continue;
            }

            row.push(pheromone_default);
        }

        pheromone_table.push(row);
    }

    pheromone_table
}

fn update_probability_table(probability_table: &mut Vec<Vec<f32>>, trail: &Vec<Vec<f32>>) {
    for (i,row) in probability_table.iter_mut().enumerate() {
        let row_sum: f32 = trail[i].iter().map(|&v| v as f32).sum();

        for (j,value) in row.iter_mut().enumerate() {
            if i == j {
                continue;
            }

            *value = trail[i][j] / row_sum;
        }
    }
}

fn calc_trail(pheromone_table: &Vec<Vec<f32>>, distances: &Vec<Vec<f32>>, alpha: f32, beta: f32) -> Vec<Vec<f32>> {
    let mut trail : Vec<Vec<f32>> = vec![vec![0.0;distances.len()]; distances.len()];

    for (i, row) in trail.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            if i == j {
                continue;
            }
            
            *value = pheromone_table[i][j].powf(alpha) * distances[i][j].powf(beta);
        }
    }

    trail
}

fn scale_distances(distances: &mut Vec<Vec<f32>>, slace_factor: f32) {
    for row in distances.iter_mut() {
        for value in row.iter_mut() {
            *value /= slace_factor;

            let temp : f32 = format!("{:.2}", value).parse().unwrap();

            *value = temp;
        }
    }
}

impl Aco {
    pub fn new(ants: usize, iterations: usize) -> Self {    
        Self {
            ants,
            iterations,
            alpha : 1.0,
            beta : 1.0,
            pheromone_default: 0.3,
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

        let solver = Aco::new(cities.len(), 100);

        let mut probability_table : Vec<Vec<f32>> = vec![vec![0.0;cities.len()];cities.len()];
        let mut pheromone_table = initialize_pheromone_table(cities.len(), 0.3);
        let mut distances = adjacency_matrix(&cities);
        let mut trails = calc_trail(&pheromone_table, &distances, solver.alpha, solver.beta);

        scale_distances(&mut distances, 100.0);

        assert_eq!(vec![vec![0.0, 0.1, 0.07],
                        vec![0.1, 0.0, 0.07],
                        vec![0.07, 0.07, 0.0]], distances);

        update_probability_table(&mut probability_table, &trails);
            
        // p = (trail[row][col]) / sum(trail[row])

        assert_eq!(vec![vec![0.0, 0.5857864, 0.41421357], 
                        vec![0.5857864, 0.0, 0.41421357],
                        vec![0.5, 0.5 , 0.0]], probability_table);
    }
}