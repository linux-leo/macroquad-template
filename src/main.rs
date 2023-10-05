mod screen;

use macroquad::prelude::*;
use crate::screen::VirtualScreen;

fn window_conf() -> Conf {
    let config = Conf {
        window_title: "Test".to_owned(),
        window_width: 256,
        window_height: 192,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let screen = VirtualScreen::new(config.window_width, config.window_height);
    screen.apply_camera();

    loop {
        clear_background(LIGHTGRAY);

        draw_rectangle(128., 96., 50., 50., GREEN);

        next_frame().await
    }
}