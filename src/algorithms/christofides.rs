use crate::solution::Solution;
use crate::city::City;
use crate::utils::adjacency_matrix;

pub struct Christofides;

type Pair = (usize, usize);

fn prims(cities: &Vec<City>, root: usize) -> Vec<Pair> {
    let matrix = adjacency_matrix(cities);
    let mut edges : Vec<Pair> = vec![];

    let mut visited = vec![root];

    let mut lowest = f32::MAX;

    let mut temp_col = root;
    let mut temp_row = root;

    while visited.len() != cities.len() {
        for row in 0..matrix.len() {

            if visited.contains(&row) {
                continue;
            }

            for visit in visited.clone().into_iter() {

                for col in 0..matrix[row].len() {
                
                    if col != visit {
                        continue;
                    }
                    
                    if lowest > matrix[row][visit] && matrix[row][visit] != 0.0 {
                        lowest = matrix[row][visit];
                        temp_col = row;
                        temp_row = visit;
                    }
                }
            }
        }

        lowest = f32::MAX;
        edges.push((temp_row, temp_col));
        visited.push(temp_col);
    }
    
    edges
}

impl Christofides {    
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Solution for Christofides {
    fn solve(&self, cities : &Vec<City>) -> Vec<Vec<City>> {
        todo!();
    }
}

#[cfg(test)]
mod solutions {
    use super::*;

    #[test]
    fn christofides() {

    }
}