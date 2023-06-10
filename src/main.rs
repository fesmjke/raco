use city::City;
use raylib::prelude::*;
use drawable::Drawable;

mod city;
pub mod drawable;

fn main() {
    let width = 1280;
    let height = 720;
    
    let (mut rl, thread) = raylib::init()
    .size(width, height)
    .title("Rant")
    .build();

    rl.set_target_fps(60);

    let city_start = City::new(50.0, 50.0);
    let city_end = City::new(200.0, 50.0);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        d.draw_line(0, height / 2, width, height / 2, Color::BLACK);
        d.draw_line(width / 2, height , width / 2, 0, Color::BLACK);
        
        city_start.draw(&mut d);
        d.draw_line_v(city_start.position(), city_end.position(), Color::BLACK);
        city_end.draw(&mut d);

        d.clear_background(Color::WHITE);
    }
}
