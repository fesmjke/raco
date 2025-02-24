use macroquad::{
    prelude::*,
    ui::{
        hash, root_ui,
        widgets::{self, Group},
    },
};
use raco::{algorithms::BruteForce, city::City, tsp::TSP, Rendereable};

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;
const MENU_WIDTH: f32 = 240.0;
const MENU_HEIGHT: f32 = 200.0;

#[macroquad::main("Texture")]
async fn main() {
    let texture: Texture2D = load_texture("assets/usa_light.png")
        .await
        .expect("Unable to load texture `usa_light.png`");

    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    // let route = Route::new(vec![city_a, city_e, city_b, city_c, city_d, city_q]);

    // let mut best_route = false;
    // let mut algorithm = 0;

    let mut tsp = TSP::new(texture);

    let mut test_cities: Vec<City> = vec![];
    loop {
        clear_background(WHITE);

        // widgets::Window::new(hash!(), vec2(0., 0.), vec2(MENU_WIDTH, MENU_HEIGHT))
        //     .movable(false)
        //     .label("Menu")
        //     .ui(&mut *root_ui(), |ui| {
        //         // if ui.button(Vec2::new(0.0, 0.0), "first") {
        //         //     println!("first");
        //         // }

        //         ui.combo_box(hash!(), "select", &["Brute-Force", "NN"], &mut algorithm);
        //     });

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();

            if test_cities.len() < 6 {
                test_cities.push(City::new(x, y, 5.));

                println!("{}", test_cities.len());

                tsp.get_routes::<BruteForce>(&test_cities);
            } else {
                println!("UNABLE TO ADD MORE, CHANGE ALGORITHM!");
            }
        } else if is_mouse_button_pressed(MouseButton::Right) {
            test_cities.pop();
            tsp.get_routes::<BruteForce>(&test_cities);
        }

        tsp.render();

        tsp.best_route.render();

        next_frame().await
    }
}
