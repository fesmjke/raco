use raylib::prelude::*;
use utils::choose_best;
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

    usa.create_cities(preset::DefaultPresets::USA, preset::PresetSize::Medium);
    let cities : &Vec<City> = usa.get_cities();

    let n = cities.len();

    let mut state = States::ACO;

    let christofides = christofides::Christofides::new();
    let nn = nearest_neighbour::NearestNeighbour::new();
    let brute = brute_force::BruteForce::new();
    let aco = aco::Aco::new(n, 500);

    let mut routes = aco.solve(cities);

    let mut total_routes = routes.len();

    let mut best_route = choose_best(&routes);

    let mut path = Path::new(routes.first().expect("first route is missing"));

    let mut iterator = routes.iter();

    let mut drawing = true;

    while !rl.window_should_close() {
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
            match state {
                States::Brute => {
                    state = States::NN
                },
                States::NN => {
                    state = States::Christofides
                },
                States::Christofides => {
                    state = States::ACO
                },
                States::ACO => {
                    if n > 5 {
                        state = States::NN
                    } else {
                        state = States::Brute
                    }
                },
            }
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ENTER) {
            match state {
                States::Brute => routes = brute.solve(cities),
                States::NN => routes = nn.solve(cities),
                States::Christofides => routes = christofides.solve(cities),
                States::ACO => routes = aco.solve(cities),
            }
            total_routes = routes.len();

            best_route = choose_best(&routes);

            iterator = routes.iter();
        }
        
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_S) {
            drawing = !drawing;
        }
        
        let mut d = rl.begin_drawing(&thread);

        d.draw_texture(usa.get_texture(), 0, 0, Color::WHITE);

        if drawing {
            match iterator.next() {
                Some(route) => {
                    path.replace(route);
                },
                None => path.replace(&routes[best_route]),
            }
        } else {
            path.replace(&routes[best_route]);
        }

        path.draw(&mut d);

        d.draw_text(format!("Total distance : {:.2}", path.total_distance() / 100.0).as_str(), 0, 0, 20, Color::BLACK);
        d.draw_text(format!("Current route index: {} / {}", iterator.len(), total_routes).as_str(), 0, 24, 20, Color::BLACK);
        d.draw_text(format!("Best route index: {} / {}", best_route, total_routes).as_str(), 0, 48, 20, Color::BLACK);
        d.draw_text(format!("Algorithm: {}", state).as_str(), 0, 72, 20, Color::BLACK);

        d.clear_background(Color::WHITE);
    }
}
