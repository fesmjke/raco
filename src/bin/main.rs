use macroquad::{
    prelude::*,
    ui::{
        hash, root_ui,
        widgets::{self},
    },
};
use raco::{
    algorithms::{Algorithms, BruteForce},
    city::City,
    tsp::TSP,
    Rendereable,
};

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

const MENU_WIDTH: f32 = 340.0;
const MENU_HEIGHT: f32 = SCREEN_HEIGHT;

const DRAW_WINDOW_WIDTH: f32 = SCREEN_WIDTH - MENU_WIDTH;
const DRAW_WINDOW_HEIGHT: f32 = SCREEN_HEIGHT;

#[macroquad::main("raco")]
async fn main() {
    let texture: Texture2D = load_texture("assets/usa_light.png")
        .await
        .expect("Unable to load texture `usa_light.png`");

    let ui_rect = Rect::new(0., 0., MENU_WIDTH, MENU_HEIGHT);
    let draw_rect = Rect::new(ui_rect.w, ui_rect.y, DRAW_WINDOW_WIDTH, DRAW_WINDOW_HEIGHT);

    let scale_x = draw_rect.w / texture.width();
    let scale_y = draw_rect.h / texture.height();
    let scale = scale_x.min(scale_y);
    let texture_x = draw_rect.x + (draw_rect.w - texture.width() * scale) / 2.0;
    let texture_y = draw_rect.y + (draw_rect.h - texture.height() * scale) / 2.0;

    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut algorithms = [Algorithms::BruteForce, Algorithms::NearestNeighbour];

    let mut check_box_algorithm = 0;
    let mut tsp = TSP::new();
    let mut best_distance = String::new();
    let mut best_route = true;
    let mut worts_route = false;

    let mut test_cities: Vec<City> = vec![];
    loop {
        clear_background(WHITE);

        set_default_camera();

        widgets::Window::new(hash!("Menu"), vec2(0., 0.), vec2(MENU_WIDTH, MENU_HEIGHT))
            .movable(false)
            .label("Menu")
            .ui(&mut *root_ui(), |ui| {
                // select algorithm
                ui.tree_node(hash!(), "Algorithms", |ui| {
                    ui.combo_box(
                        hash!(),
                        "Selected",
                        &["Brute-Force", "Nearest Neighbour"],
                        &mut check_box_algorithm,
                    );

                    // match check_box_algorithm {
                    //     _ => {
                    //         todo!()
                    //     }
                    // }
                });

                ui.tree_node(hash!(), "Options", |ui| {
                    ui.checkbox(hash!(), "best route", &mut best_route);
                    ui.checkbox(hash!(), "worst route", &mut worts_route);
                });

                ui.tree_node(hash!(), "Details", |ui| {
                    ui.label(None, &best_distance);
                });
            });

        // drawing rectangle
        draw_rectangle(
            ui_rect.w,
            ui_rect.y,
            DRAW_WINDOW_WIDTH,
            DRAW_WINDOW_HEIGHT,
            WHITE,
        );

        draw_texture_ex(
            &texture,
            texture_x,
            texture_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(texture.width() * scale, texture.height() * scale)),
                ..Default::default()
            },
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();

            if x > MENU_WIDTH {
                if test_cities.len() < 6 {
                    test_cities.push(City::new(x, y, 5.));

                    println!("{}", test_cities.len());

                    tsp.get_routes::<BruteForce>(&test_cities);
                    best_distance = format!("Best distance: {}", tsp.best_route.path_length());
                } else {
                    println!("UNABLE TO ADD MORE, CHANGE ALGORITHM!");
                }
            }
        } else if is_mouse_button_pressed(MouseButton::Right) {
            test_cities.pop();
            tsp.get_routes::<BruteForce>(&test_cities);
            best_distance = format!("Best distance: {}", tsp.best_route.path_length());
        }

        tsp.best_route.render();

        next_frame().await
    }
}
