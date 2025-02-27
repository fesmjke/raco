use city::City;
use route::Route;

pub trait Rendereable {
    fn render(&self);
}

pub trait Solver<'a> {
    /// solve - return best route
    // TODO replace Route<'a> with Option<Route<'a>>
    // due to some algorithms might have poor performance (Brute-Force)
    fn solve(routes: &[Route]) -> Route;
    fn get_routes(cities: &[City]) -> Vec<Route>;
}

pub mod algorithms {
    pub enum Algorithms {
        BruteForce,
        NearestNeighbour,
    }

    pub use brute_force::BruteForce;
    pub use nearest_neighbour::NearestNeighbour;

    mod brute_force;
    mod nearest_neighbour;
}

pub mod city;
pub mod route;
pub mod tsp;
pub mod utils;
