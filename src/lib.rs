use route::Route;

pub trait Rendereable {
    fn render(&self);
}

pub trait Solver {
    fn solve(&self, routes: &Vec<Route>) -> Route;
}

pub mod algorithms;
pub mod city;
pub mod preset;
pub mod route;
pub mod utils;
