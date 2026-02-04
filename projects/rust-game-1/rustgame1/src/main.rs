use macroquad::prelude::*;

#[macroquad::main("RustGame1")]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        // Simple player: a white circle you control with WASD or arrow keys
        let speed = 200.0;
        let mut pos_x = screen_width() / 2.0;
        let mut pos_y = screen_height() / 2.0;

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            pos_x += speed * get_frame_time();
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            pos_x -= speed * get_frame_time();
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            pos_y += speed * get_frame_time();
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            pos_y -= speed * get_frame_time();
        }

        draw_circle(pos_x, pos_y, 30.0, WHITE);
        draw_text("RustGame1 – Move with WASD/Arrows", 20.0, 40.0, 40.0, WHITE);

        next_frame().await
    }
}
