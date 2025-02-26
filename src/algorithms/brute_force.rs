use crate::city::City;
use crate::route::Route;
use crate::utils::best_route;
use crate::Solver;

pub struct BruteForce;

impl BruteForce {
    pub const CITY_LIMIT: usize = 6;
}

fn heaps(cities: usize, permutation: &mut Vec<City>, permutations: &mut Vec<Vec<City>>) {
    if !permutations.contains(&permutation) {
        permutations.push(permutation.to_vec());
    }

    match cities {
        0 | 1 => {}
        _ => {
            for i in 0..cities {
                heaps(cities - 1, permutation, permutations);

                if cities % 2 == 0 {
                    permutation.swap(i, cities - 1);
                } else {
                    permutation.swap(0, cities - 1);
                }
            }
        }
    }
}

impl<'a> Solver<'a> for BruteForce {
    fn solve(routes: &[Route]) -> Route {
        best_route(routes)
    }

    fn get_routes(cities: &[City]) -> Vec<Route> {
        assert!(cities.len() <= BruteForce::CITY_LIMIT);

        let mut permutations_heaps = vec![];
        let initial_heaps = cities;

        heaps(
            cities.len(),
            &mut initial_heaps.to_vec(),
            &mut permutations_heaps,
        );

        permutations_heaps
            .into_iter()
            .map(|x| Route::new(x))
            .collect()
    }
}

#[cfg(test)]
mod brute_force_tests {
    use super::*;

    const EXPECTED_THREE_FACT: usize = 1 * 2 * 3;
    const EXPECTED_FOUR_FACT: usize = 1 * 2 * 3 * 4;
    const EXPECTED_FIVE_FACT: usize = 1 * 2 * 3 * 4 * 5;
    const EXPECTED_SIX_FACT: usize = 1 * 2 * 3 * 4 * 5 * 6;

    #[test]
    fn find_best() {
        let city_a = City::new(0.0, 0.0, 1.0);
        let city_b = City::new(10.0, 0.0, 1.0);
        let city_c = City::new(5.0, 5.0, 1.0);
        let city_d = City::new(5.0, 9.0, 1.0);
        let city_e = City::new(3.0, 1.0, 1.0);
        let city_q = City::new(3.0, 15.0, 1.0);

        let expected_best_route = Route::new(vec![city_a, city_e, city_b, city_c, city_d, city_q]);
        let expected_order = vec![
            EXPECTED_SIX_FACT,
            EXPECTED_FIVE_FACT,
            EXPECTED_FOUR_FACT,
            EXPECTED_THREE_FACT,
        ];

        let cities = vec![city_a, city_b, city_c, city_d, city_e, city_q];

        for (index, expected) in expected_order.into_iter().enumerate() {
            let routes = BruteForce::get_routes(&cities[..cities.len() - index]);

            assert_eq!(expected, routes.len());
        }

        let routes = BruteForce::get_routes(&cities);
        let best_route = BruteForce::solve(&routes);

        assert_eq!(expected_best_route, best_route);
    }
}
