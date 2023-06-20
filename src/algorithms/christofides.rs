use crate::solution::Solution;
use crate::city::City;

pub struct Christofides;

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