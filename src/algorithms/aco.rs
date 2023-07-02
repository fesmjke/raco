use crate::solution::Solution;
use crate::city::City;

pub struct Aco;

impl Aco {
    pub fn new() -> Self {
        Self {

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
    fn simple_aco() {

    }
}