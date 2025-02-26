use crate::{city::City, route::Route};

/// Creates a adjacency matrix between cities
pub fn adjacency_matrix(cities: &Vec<City>) -> Vec<Vec<f32>> {
    cities
        .iter()
        .map(|current_city| {
            cities
                .iter()
                .map(|next_city| current_city.distance_between(next_city))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

/// Scale distance matrix by scale factor
pub fn scale_distances(distances: &mut Vec<Vec<f32>>, scale_factor: f32) {
    distances.iter_mut().for_each(|row| {
        row.iter_mut()
            .for_each(|distance| *distance *= scale_factor)
    });
}

/// Caclulate total route length between cities
pub fn route_length(route: &Route) -> f32 {
    route
        .cities
        .iter()
        .zip(route.cities.iter().cycle().skip(1))
        .take(route.cities.len())
        .fold(0.0, |acc, (city_a, city_b)| {
            acc + city_a.distance_between(city_b)
        })
}

// TODO WHAT THE HELL???
pub fn convert(indexes: &Vec<usize>, cities: &Vec<City>) -> Vec<City> {
    let mut temp = vec![];

    for index in indexes.iter() {
        temp.push(cities[*index].clone());
    }

    temp
}

/// Return best route index
pub fn best_route_index(routes: &[Route]) -> usize {
    assert!(!routes.is_empty());

    routes
        .iter()
        .enumerate()
        .min_by(|(_, current_route), (_, next_route)| {
            route_length(current_route)
                .partial_cmp(&route_length(next_route))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(index, _)| index)
        .unwrap_or(0)
}

/// Return worst route index
pub fn worst_route_index(routes: &[Route]) -> usize {
    assert!(!routes.is_empty());

    routes
        .iter()
        .enumerate()
        .max_by(|(_, current_route), (_, next_route)| {
            route_length(current_route)
                .partial_cmp(&route_length(next_route))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(index, _)| index)
        .unwrap_or(0)
}

/// Return a best route from generated routes
pub fn best_route(routes: &[Route]) -> Route {
    let index = best_route_index(routes);

    routes[index].clone()
}

/// Return a worst route from generated routes
pub fn worst_route(routes: &[Route]) -> Route {
    let index = worst_route_index(routes);

    routes[index].clone()
}

#[cfg(test)]
mod utils_tests {
    use super::{adjacency_matrix, *};

    #[test]
    fn adjacency_matrix_test() {
        let city_a = City::new(0.0, 0.0, 1.0);
        let city_b = City::new(10.0, 0.0, 1.0);
        let city_c = City::new(0.0, 10.0, 1.0);

        let cities = vec![city_a, city_b, city_c];

        let matrix = adjacency_matrix(&cities);

        let expected_matrix = vec![
            vec![0.0, 10.0, 10.0],
            vec![10.0, 0.0, 14.142136],
            vec![10.0, 14.142136, 0.0],
        ];

        assert_eq!(expected_matrix, matrix);
    }

    #[test]
    fn scale_distances_test() {
        let mut matrix = vec![
            vec![0.0, 10.0, 10.0],
            vec![10.0, 0.0, 1.0],
            vec![10.0, 1.0, 0.0],
        ];

        scale_distances(&mut matrix, 0.1);

        let expected_matrix = vec![
            vec![0.0, 1.0, 1.0],
            vec![1.0, 0.0, 0.1],
            vec![1.0, 0.1, 0.0],
        ];

        assert_eq!(expected_matrix, matrix);
    }

    #[test]
    fn total_route_distance_test() {
        let city_a = City::new(0.0, 0.0, 1.0);
        let city_b = City::new(10.0, 0.0, 1.0);
        let city_c = City::new(0.0, 10.0, 1.0);

        let cities = vec![city_a, city_b, city_c];
        let route = Route::new(cities);

        let route_length = route_length(&route);
        let exptected_length = 34.1421356237;

        assert_eq!(exptected_length, route_length);
    }

    #[test]
    fn best_route_index_test() {
        let city_a = City::new(0.0, 0.0, 1.0);
        let city_b = City::new(10.0, 0.0, 1.0);
        let city_c = City::new(0.0, 10.0, 1.0);

        let cities = vec![city_a, city_b, city_c];
        let route_1 = Route::new(cities);

        let city_a = City::new(0.0, 0.0, 1.0);
        let city_b = City::new(9.0, 0.0, 1.0);
        let city_c = City::new(0.0, 10.0, 1.0);

        let cities = vec![city_a, city_b, city_c];
        let route_2 = Route::new(cities);

        let routes = vec![route_1, route_2];

        let best_route_index = best_route_index(&routes);
        let expected_best_index = 1;

        assert_eq!(expected_best_index, best_route_index);
    }

    #[test]
    fn worst_route_index_test() {
        let city_a = City::new(0.0, 0.0, 1.0);
        let city_b = City::new(10.0, 0.0, 1.0);
        let city_c = City::new(0.0, 10.0, 1.0);

        let cities = vec![city_a, city_b, city_c];
        let route_1 = Route::new(cities);

        let city_a = City::new(0.0, 0.0, 1.0);
        let city_b = City::new(9.0, 0.0, 1.0);
        let city_c = City::new(0.0, 10.0, 1.0);

        let cities = vec![city_a, city_b, city_c];
        let route_2 = Route::new(cities);

        let routes = vec![route_1, route_2];

        let best_worst_index = worst_route_index(&routes);
        let expected_worst_index = 0;

        assert_eq!(expected_worst_index, best_worst_index);
    }
}
