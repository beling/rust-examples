 use macroquad::prelude::*;

fn dywan(x: f32, y: f32, size: f32) {
    if size < 5.0 {
        draw_rectangle(x, y, size, size, BLACK);
    } else {
        let size = size / 3.0;
        let dsize = size * 2.0;
        dywan(x, y, size);
        dywan(x+size, y, size);
        dywan(x+dsize, y, size);
        dywan(x, y+size, size);
        dywan(x+dsize, y+size, size);
        dywan(x, y+dsize, size);
        dywan(x+size, y+dsize, size);
        dywan(x+dsize, y+dsize, size);
    }
}

#[macroquad::main("Dywan")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        let size = screen_width().min(screen_height());
        let x = (screen_width() - size) / 2.;
        let y = (screen_height() - size) / 2.;

        draw_rectangle(x, y, size, size, WHITE);
        dywan(x, y, size);
        next_frame().await
    }
}
