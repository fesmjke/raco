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

pub fn solve<'a, S>(routes: &'a Vec<Route>)
where
    S: Solver<'a>,
{
    S::solve(routes);
}

pub mod algorithms {
    pub use brute_force::BruteForce;
    pub mod brute_force;
    // pub mod nearest_neighbour;
}

pub mod city;
pub mod preset;
pub mod route;
pub mod utils;
