 use macroquad::prelude::*;

fn carpet(x: f32, y: f32, size: f32) {
    if size < 5.0 {
        draw_rectangle(x, y, size, size, BLACK);
    } else {
        let size = size / 3.0;
        let dsize = size * 2.0;
        carpet(x, y, size);
        carpet(x+size, y, size);
        carpet(x+dsize, y, size);
        carpet(x, y+size, size);
        carpet(x+dsize, y+size, size);
        carpet(x, y+dsize, size);
        carpet(x+size, y+dsize, size);
        carpet(x+dsize, y+dsize, size);
    }
}

#[macroquad::main("Sierpiński carpet")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        let size = screen_width().min(screen_height());
        let x = (screen_width() - size) / 2.;
        let y = (screen_height() - size) / 2.;

        draw_rectangle(x, y, size, size, WHITE);
        carpet(x, y, size);
        next_frame().await
    }
}
