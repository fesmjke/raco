use raylib::prelude::*;
use std::fmt;

use algorithms::{brute_force, nearest_neighbour, christofides, aco};

use city::City;
use path::Path;
use preset::Preset;

use solution::Solution;
use drawable::Drawable;

pub mod city;
pub mod solution;
pub mod algorithms;
pub mod drawable;
pub mod path;
pub mod utils;
pub mod preset;

enum States {
    Brute,
    NN,
    Christofides,
    ACO,
}

impl fmt::Display for States {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            States::Brute => write!(f, "Brute force"),
            States::NN => write!(f, "NN"),
            States::Christofides => write!(f, "Christofides"),
            States::ACO => write!(f, "ACO")
        }
    }
}

fn main() {
    let width = 1280;
    let height = 720;
    let (mut rl, thread) = raylib::init()
    .size(width, height)
    .title("Rant")
    .build();

    let texture = rl.load_texture(&thread, "./assets/usa_light.png").expect("Could not load texture");

    let mut usa = Preset::new(texture);

    rl.set_target_fps(60);

    usa.create_cities(preset::DefaultPresets::USA, preset::PresetSize::Small);
    let mut cities : &Vec<City> = usa.get_cities();

    let n = cities.len();

    let mut state = States::ACO;

    let christofides = christofides::Christofides::new();
    let nn = nearest_neighbour::NearestNeighbour::new();
    let brute = brute_force::BruteForce::new();
    let aco = aco::Aco::new(n, 10);

    // Its looks horrible 
    let mut routes = aco.solve(&cities);

    let mut route_index = 1;

    let mut path = Path::new(routes[0].clone(), cities[0].clone());

    let mut distances : Vec<f32> = vec![];
    let mut best_route = 0 as usize;
    let mut chosen = false;

    while !rl.window_should_close() {        
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_R) {

            match state {
                States::Brute => {
                    state = States::NN;
                    routes = nn.solve(&cities);
                },
                States::NN => {
                    state = States::Christofides;
                    routes = christofides.solve(&cities);
                },
                States::Christofides => {
                    state = States::ACO;
                    routes = aco.solve(&cities);
                },
                States::ACO => {
                    if n > 5 {
                        state = States::NN;
                        routes = nn.solve(&cities);
                    } else {
                        state = States::Brute;
                        routes = brute.solve(&cities);
                    }
                },
            }

            route_index = 1;

            path = Path::new(routes[0].clone(), cities[0].clone());

            distances = vec![];
            best_route = 0;
            chosen = false;
        }
        
        let mut d = rl.begin_drawing(&thread);

        d.draw_texture(usa.get_texture(), 0, 0, Color::WHITE);

        // This looks horrible too
        match path.draw_mut(&mut d) {
            drawable::DrawState::Drawing => {},
            drawable::DrawState::Finished => {
                if distances.len() == routes.len() - 1 && !chosen {
                    best_route = distances.iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.total_cmp(b))
                    .map(|(index, _)| index).unwrap();
                    
                    chosen = true;

                    path = Path::new(routes[best_route].clone(), cities[0].clone());

                    continue;
                }

                if !chosen { 
                    distances.push(path.total_distance());

                    path = Path::new(routes[route_index].clone(), cities[0].clone());

                    route_index += 1;
                }
            }
        }

        if chosen {
            path.draw(&mut d);
        }

        d.draw_text(format!("Total distance : {:.2}", path.total_distance() / 100.0).as_str(), 0, 0, 24, Color::BLACK);
        d.draw_text(format!("Current route index: {}", route_index).as_str(), 0, 24, 24, Color::BLACK);
        d.draw_text(format!("Best route index: {}", best_route).as_str(), 0, 48, 24, Color::BLACK);
        d.draw_text(format!("Algorithm: {}", state).as_str(), 0, 72, 24, Color::BLACK);

        d.clear_background(Color::WHITE);
    }
}
