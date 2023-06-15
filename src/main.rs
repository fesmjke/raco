use city::City;
use path::Path;
use drawable::Drawable;
use raylib::prelude::*;
use rand::Rng;
use solution::Solution;

pub mod city;
pub mod brute_force;
pub mod solution;
pub mod drawable;
pub mod path;

fn main() {
    let width = 1280;
    let height = 720;
    let (mut rl, thread) = raylib::init()
    .size(width, height)
    .title("Rant")
    .build();

    rl.set_target_fps(60);

    let mut cities : Vec<City> = vec![];

    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        cities.push(City::new(x as f32, y as f32))
    }

    let brute = brute_force::BruteForce::new();

    // Its looks horrible 
    let routes = brute.solve(&cities);

    let mut route_index = 1;

    let mut path = Path::new(routes[0].clone(), cities[0].clone());

    let mut distances : Vec<f32> = vec![];
    let mut best_route = 0 as usize;
    let mut choosed = false;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.draw_line(0, height / 2, width, height / 2, Color::BLACK);
        d.draw_line(width / 2, height , width / 2, 0, Color::BLACK);

        // This looks horrible too
        match path.draw_mut(&mut d) {
            drawable::DrawState::Drawing => {},
            drawable::DrawState::Finished => {
                if distances.len() == routes.len() - 1 && !choosed {
                    best_route = distances.iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.total_cmp(b))
                    .map(|(index, _)| index).unwrap();
                    
                    choosed = true;

                    path = Path::new(routes[best_route].clone(), cities[0].clone());

                    continue;
                }

                if !choosed { 
                    distances.push(path.total_distance());

                    path = Path::new(routes[route_index].clone(), cities[0].clone());

                    route_index += 1;
                }
            }
        }

        if choosed {
            path.draw(&mut d);
        }

        d.draw_text(format!("Total distance : {:.2}", path.total_distance() / 100.0).as_str(), 0, 0, 24, Color::BLACK);
        d.draw_text(format!("Current route index: {}", route_index).as_str(), 0, 24, 24, Color::BLACK);
        d.draw_text(format!("Best route index: {}", best_route).as_str(), 0, 48, 24, Color::BLACK);

        d.clear_background(Color::WHITE);
    }
}
