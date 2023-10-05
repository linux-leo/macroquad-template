use macroquad::prelude::*;

pub struct VirtualScreen {
    virtual_width: f32,
    virtual_height: f32,
}

impl VirtualScreen {
    pub fn new(virtual_width: f32, virtual_height: f32) -> Self {
        VirtualScreen {
            virtual_width,
            virtual_height,
        }
    }

    pub fn apply_camera(&self) {
        let screen_aspect = screen_width() / screen_height();
        let virtual_aspect = self.virtual_width / self.virtual_height;

        let (scale_x, scale_y) = if screen_aspect > virtual_aspect {
            // The screen is wider than the desired aspect ratio
            let scale = screen_height() / self.virtual_height;
            (scale, scale)
        } else {
            // The screen is taller or equal to the desired aspect ratio
            let scale = screen_width() / self.virtual_width;
            (scale, scale)
        };

        set_camera(&Camera2D {
            target: vec2(self.virtual_width / 2.0, self.virtual_height / 2.0),
            zoom: vec2(1.0 / scale_x, 1.0 / scale_y),
            ..Default::default()
        });
    }
}