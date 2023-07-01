use crate::solution::Solution;
use crate::city::City;
use crate::utils::adjacency_matrix;
use std::collections::HashMap;

pub struct Christofides;

type Pair = (usize, usize);

fn prims(cities: &Vec<City>, root: usize) -> Vec<Pair> {
    let matrix = adjacency_matrix(cities);
    let mut edges : Vec<Pair> = vec![];

    let mut visited = vec![root];

    let mut lowest = f32::MAX;

    let mut temp_col = root;
    let mut temp_row = root;

    while visited.len() != cities.len() {
        for row in 0..matrix.len() {

            if visited.contains(&row) {
                continue;
            }

            for visit in visited.clone().into_iter() {

                for col in 0..matrix[row].len() {
                
                    if col != visit {
                        continue;
                    }
                    
                    if lowest > matrix[row][visit] && matrix[row][visit] != 0.0 {
                        lowest = matrix[row][visit];
                        temp_col = row;
                        temp_row = visit;
                    }
                }
            }
        }

        lowest = f32::MAX;
        edges.push((temp_row, temp_col));
        visited.push(temp_col);
    }
    
    edges
}

fn degree(edges: &Vec<Pair>) -> HashMap<usize, usize> {
    let mut degree = HashMap::new();

    for pair in edges.iter() {
        match degree.get(&pair.0) {
            Some(n) => {degree.insert(pair.0, n + 1);},
            None => {degree.insert(pair.0, 1);},
        }
        
        match degree.get(&pair.1) {
            Some(n) => {degree.insert(pair.1, n + 1);},
            None => {degree.insert(pair.1, 1);},
        }

    }

    degree
}

fn convert(indexes: &HashMap<usize, usize>, cities: &Vec<City>) -> Vec<City> {
    let mut temp = vec![];

    for (k,_) in indexes.iter() {
        temp.push(cities[*k].clone());
    }

    temp
}

fn minimum_weight_matching(odd: &Vec<City>) -> Vec<Pair> {
    let mut pairs = vec![];

    let matrix = adjacency_matrix(odd);
    
    let mut lowest = f32::MAX;

    let mut temp_col = 0 as usize;
    let mut temp_row = 0 as usize;

    let mut skip_row : Vec<usize> = vec![];
    let mut skip_col : Vec<usize> = vec![];

    for row in 0..matrix.len() {

        if skip_row.contains(&row) {
            continue;
        }

         for col in 0..matrix[row].len() {
            
            if skip_col.contains(&col) {
                continue;
            }
                
            if lowest > matrix[row][col] && matrix[row][col] != 0.0 {
                lowest = matrix[row][col];
                temp_row = row;
                temp_col = col;
            }
        }

        skip_row.push(temp_row);
        skip_row.push(temp_col);
        skip_col.push(temp_row);
        skip_col.push(temp_col);

        pairs.push((temp_col, temp_row));
        lowest = f32::MAX;
    }

    pairs
}

fn unite(mspt: &Vec<Pair>,matching: &Vec<Pair>) -> Vec<Pair> {
    let mut temp = vec![];

    temp.extend(mspt);
    temp.extend(matching);

    temp
}

fn update_mwm_indexes(mwm: &mut Vec<Pair>, odd_degree: &HashMap<usize, usize>) {
    let shift_index = odd_degree.keys().min().unwrap();
    
    for pair in mwm.iter_mut() {
        pair.0 += shift_index;
        pair.1 += shift_index;
    }
}

fn euler_tour(united: &Vec<Pair>, cities: &Vec<City>) -> Vec<City> {
    let mut tour : Vec<City> = vec![];
    let mut visited : Vec<usize> = vec![];
    
    for pair in united.iter() {

        if !visited.contains(&pair.0) {
            tour.push(cities[pair.0].clone());
            visited.push(pair.0);
        }

        if !visited.contains(&pair.1) {
            tour.push(cities[pair.1].clone());
            visited.push(pair.1);
        } 

    }

    tour
}

impl Christofides {    
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Solution for Christofides {
    fn solve(&self, cities : &Vec<City>) -> Vec<Vec<City>> {
        let mut routes = vec![];

        for (i, _) in cities.iter().enumerate() {
            let mspt = prims(&cities, i);

            let mut degree = degree(&mspt);
    
            for (k, v) in degree.clone().iter() {
                if v % 2 == 0 {
                    degree.remove(k);
                }
            }
    
            let odd_cities = convert(&degree, &cities);
    
            let mut mwm = minimum_weight_matching(&odd_cities);
    
            update_mwm_indexes(&mut mwm, &degree);
    
            let united = unite(&mspt, &mwm);
    
            let route = euler_tour(&united, &cities);
    
            routes.push(route);
        }

        routes
    }
}

#[cfg(test)]
mod solutions {
    use super::*;

    #[test]
    fn christofides() {

    }

    #[test]
    fn simple_prims() {
        let city_a = City::new(0.0, 0.0);
        let city_b = City::new(10.0, 0.0);
        let city_c = City::new(5.0, 5.0);
        let city_d = City::new(15.0, 8.0);
        let city_e = City::new(8.0, 31.0);

        let mut cities = vec![city_a, city_b, city_c];

        let mut answer = prims(&cities, 0);

        assert_eq!(vec![(0,2), (2,1)], answer);

        cities.push(city_d);

        answer = prims(&cities, 0);

        assert_eq!(vec![(0,2), (2,1), (1,3)], answer);

        answer = prims(&cities, 3);

        assert_eq!(vec![(3,1), (1,2), (2,0)], answer);

        cities.push(city_e);

        answer = prims(&cities, 0);

        assert_eq!(vec![(0,2), (2,1), (1,3), (3,4)],answer);
    }

    #[test]
    fn moderate_prims() {
        let city_a = City::new(5.0, 5.0);
        let city_b = City::new(4.0, 6.0);
        let city_c = City::new(6.0, 6.0);
        let city_d = City::new(4.0, 6.0);
        let city_e = City::new(6.0, 4.0);

        let cities = vec![city_a, city_b, city_c, city_d, city_e];

        let answer = prims(&cities, 0);

        assert_eq!(vec![(0,1), (0,2), (0,3), (0,4)], answer);
    }

    #[test]
    fn simple_degree() {
        let edges : Vec<Pair> = vec![(0,2), (2,1)];

        let mut answer = degree(&edges);

        assert_eq!(HashMap::from([(0,1),(1,1),(2,2)]), answer);

        for (k, v) in answer.clone().iter() {
            if v % 2 == 0 {
                answer.remove(k);
            }
        }

        assert_eq!(HashMap::from([(0,1),(1,1)]), answer);
    }

    #[test]
    fn moderate_degree() {
        let edges : Vec<Pair> = vec![(0,1), (0,2), (0,3), (0,4)];

        let mut answer = degree(&edges);

        assert_eq!(HashMap::from([(0,4),(1,1),(2,1),(3,1),(4,1)]), answer);

        for (k, v) in answer.clone().iter() {
            if v % 2 == 0 {
                answer.remove(k);
            }
        }

        assert_eq!(HashMap::from([(1,1),(2,1),(3,1),(4,1)]), answer);
    }

    #[test]
    fn simple_mwm() {
        let city_a = City::new(0.0, 0.0);
        let city_b = City::new(10.0, 0.0);
        let city_c = City::new(5.0, 5.0);

        let cities = vec![city_a, city_b, city_c];

        let edges : Vec<Pair> = vec![(0,2), (2,1)];

        let mut degree = degree(&edges);

        for (k, v) in degree.clone().iter() {
            if v % 2 == 0 {
                degree.remove(k);
            }
        }

        let odd_cities = convert(&degree, &cities);

        let mut pairs = minimum_weight_matching(&odd_cities);

        update_mwm_indexes(&mut pairs, &degree);

        assert_eq!(vec![(1,0)],pairs);
    }

    #[test]
    fn moderate_mwm() {
        let city_a = City::new(5.0, 5.0);
        let city_b = City::new(4.0, 6.0);
        let city_c = City::new(6.0, 6.0);
        let city_d = City::new(4.0, 4.0);
        let city_e = City::new(6.0, 4.0);

        let cities = vec![city_a, city_b, city_c, city_d, city_e];

        let edges = prims(&cities, 0);

        assert_eq!(vec![(0,1), (0,2), (0,3), (0,4)], edges);

        let mut degree = degree(&edges);

        for (k, v) in degree.clone().iter() {
            if v % 2 == 0 {
                degree.remove(k);
            }
        }

        assert_eq!(HashMap::from([(1,1),(2,1),(3,1),(4,1)]), degree);

        let odd_cities = convert(&degree, &cities);

        let mut pairs = minimum_weight_matching(&odd_cities);

        update_mwm_indexes(&mut pairs, &degree);

        assert_eq!(vec![(2,1), (4,3)],pairs);
    }

    #[test]
    fn simple_unite() {
        let mspt = vec![(0,2), (2,1)];
        let mwm = vec![(1,0)];

        let united = unite(&mspt, &mwm);

        assert_eq!(vec![(0,2), (2,1), (1,0)], united);
    }

    #[test]
    fn moderate_unite() {
        let mspt = vec![(0,1), (0,2), (0,3), (0,4)];
        let mwm = vec![(2,1), (4,3)];

        let united = unite(&mspt, &mwm);

        assert_eq!(vec![(0,1), (0,2), (0,3), (0,4), (2,1), (4,3)], united);
    }

    #[test]
    fn euler_tour_moderate() {
        let city_a = City::new(5.0, 5.0);
        let city_b = City::new(4.0, 6.0);
        let city_c = City::new(6.0, 6.0);
        let city_d = City::new(4.0, 4.0);
        let city_e = City::new(6.0, 4.0);

        let cities = vec![city_a, city_b, city_c, city_d, city_e];

        let united = vec![(0,1), (0,2), (0,3), (0,4), (2,1), (4,3)];

        let route = euler_tour(&united, &cities);

        assert_eq!(5, route.len());
    }
}