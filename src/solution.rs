use crate::city::City;

pub trait Solution {
    fn solve(&self,cities : &Vec<City>) -> Vec<Vec<City>>;
}