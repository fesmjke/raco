use crate::city::City;

pub trait Solution {
    fn solve(cities : &Vec<City>) -> Vec<Vec<City>>;
}