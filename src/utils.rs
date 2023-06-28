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

