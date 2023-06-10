use raylib::prelude::*;

fn main() {
    let width = 1280;
    let height = 720;
    
    let (mut rl, thread) = raylib::init()
    .size(width, height)
    .title("Rant")
    .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        d.draw_line(0, height / 2, width, height / 2, Color::BLACK);
        d.draw_line(width / 2, height , width / 2, 0, Color::BLACK);
        
        d.clear_background(Color::WHITE);
    }
}
