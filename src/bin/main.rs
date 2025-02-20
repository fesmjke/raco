use macroquad::prelude::*;
use raco::city::City;

// enum States {
//     Brute,
//     NN,
//     Christofides,
//     ACO,
// }

// impl fmt::Display for States {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             States::Brute => write!(f, "Brute force"),
//             States::NN => write!(f, "NN"),
//             States::Christofides => write!(f, "Christofides"),
//             States::ACO => write!(f, "ACO"),
//         }
//     }
// }

#[macroquad::main("Texture")]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        
        next_frame().await
    }
}
