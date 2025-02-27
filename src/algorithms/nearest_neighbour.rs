use crate::city::City;
use crate::route::Route;
use crate::utils::best_route;
use crate::Solver;

pub struct NearestNeighbour;

impl NearestNeighbour {
    pub const CITY_LIMIT: usize = 12;
}

fn select_nearest(current: &City, cities: &[City], visited: &[City]) -> City {
    cities
        .iter()
        .filter(|city| !visited.contains(city))
        .min_by(|city_a, city_b| {
            current
                .distance_between(city_a)
                .partial_cmp(&current.distance_between(city_b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(&current)
        .clone()
}

impl<'a> Solver<'a> for NearestNeighbour {
    fn solve(routes: &[Route]) -> Route {
        best_route(routes)
        // routes.first().unwrap_or(&Route::default()).clone()
    }

    fn get_routes(cities: &[City]) -> Vec<Route> {
        let mut routes = vec![];

        for current in cities.iter() {
            let mut visited = vec![];
            let mut new = *current;

            for _ in cities.iter() {
                visited.push(new);
                let next = select_nearest(&new, cities, &visited);
                new = next;
            }

            routes.push(Route::new(visited));
        }

        routes
    }
}

#[cfg(test)]
mod solutions {
    use super::*;

    #[test]
    fn best_route() {
        let city_a = City::new(0.0, 0.0, 1.0);
        let city_b = City::new(10.0, 0.0, 1.0);
        let city_c = City::new(5.0, 5.0, 1.0);
        let city_d = City::new(5.0, 9.0, 1.0);
        let city_e = City::new(3.0, 1.0, 1.0);
        let city_q = City::new(3.0, 15.0, 1.0);

        let expected_best_route = Route::new(vec![city_e, city_a, city_c, city_d, city_q, city_b]);
        let cities = vec![city_a, city_b, city_c, city_d, city_e, city_q];

        let routes = NearestNeighbour::get_routes(&cities);
        let best_route = NearestNeighbour::solve(&routes);

        assert_eq!(expected_best_route, best_route);
    }
}
