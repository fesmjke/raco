use crate::city::City;

pub fn adjacency_matrix(cities: &Vec<City>) -> Vec<Vec<f32>> {
    let mut matrix : Vec<Vec<f32>> = vec![];
    
    for i in 0..cities.len() {
        let mut row :Vec<f32> = vec![];

        for j in 0..cities.len() {
            if i == j {
                row.push(0.0);
                continue;
            }

            row.push(cities[i].position().distance_to(*cities[j].position()));
        }

        matrix.push(row);
    }

    matrix
}

pub fn scale_distances(distances :&mut Vec<Vec<f32>>, slace_factor: f32) {
    for row in distances.iter_mut() {
        for value in row.iter_mut() {
            *value /= slace_factor;
        }
    }
}

pub fn caclulate_distance(route :&Vec<City>) -> f32 {
    let mut length = 0.0;
    let mut current = route[0].clone();

    for city in route.iter() {
        length += current.position().distance_to(*city.position());

        current = city.clone();
    }

    length += current.position().distance_to(*route[0].position());

    length
}

pub fn convert(indexes: &Vec<usize>,cities :&Vec<City>) -> Vec<City> {
    let mut temp = vec![];

    for index in indexes.iter() {
        temp.push(cities[*index].clone());
    }

    temp
}

pub fn choose_best(routes : &Vec<Vec<City>>) -> usize {
    let mut best = 0 as usize;
    let mut length = caclulate_distance(routes.first().expect("first route missing"));

    for (index,route) in routes.iter().enumerate() {
        let delta = caclulate_distance(route);
        
        if length > delta {
            best = index;
            length = delta;
        }
    }

    best
}

#[cfg(test)]
mod utils {
    use super::*;

    #[test]
    fn calc_distance() {
        let city_a = City::new(0.0, 0.0);
        let city_b = City::new(10.0, 0.0);
        let city_c = City::new(5.0, 5.0);

        let route = vec![city_a, city_b, city_c];

        let length = caclulate_distance(&route);

        assert_eq!(24.142136 ,length)
    }
}