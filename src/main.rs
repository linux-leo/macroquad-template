mod screen;

use macroquad::prelude::*;
use crate::screen::VirtualScreen;

struct PixCfg<'a> {
    window_title: &'a str,
    window_width: f32,
    window_height: f32,
    scaling_factor: f32,
    fullscreen: bool,
}

const pix_cfg: PixCfg = PixCfg {
    window_title: "hi",
    window_width: 256.0,
    window_height: 192.0,
    scaling_factor: 4.0,
    fullscreen: false,
};

fn window_conf() -> Conf {
    Conf  {
        window_title: pix_cfg.window_title.to_string(),
        window_width: (pix_cfg.window_width*pix_cfg.scaling_factor) as i32,
        window_height: (pix_cfg.window_height*pix_cfg.scaling_factor) as i32,
        fullscreen: pix_cfg.fullscreen,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen = VirtualScreen::new(pix_cfg.window_width, pix_cfg.window_height);
    screen.apply_camera();
    loop {
        clear_background(LIGHTGRAY);

        draw_rectangle(128., 96., 50., 50., GREEN);

        next_frame().await
    }
}