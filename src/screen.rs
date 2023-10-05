use macroquad::prelude::*;

pub struct VirtualScreen {
    virtual_width: f32,
    virtual_height: f32,
    scale: Vec2,
    offset: Vec2,
}

impl VirtualScreen {
    pub fn new(virtual_width: f32, virtual_height: f32) -> Self {
        let scale = vec2(screen_width() / virtual_width, screen_height() / virtual_height);
        
        fn calculate_offset(screen: f32, length: f32, scale: f32) -> f32 {
            (screen - length * scale) / 2.0
        }

        let offset = vec2(
            calculate_offset(screen_width(), virtual_width, scale.x),
            calculate_offset(screen_height(), virtual_height, scale.y)
        );

        VirtualScreen {
            virtual_width,
            virtual_height,
            scale,
            offset,
        }
    }

    pub fn apply_camera(&self) {
        set_camera(&Camera2D {
            target: vec2(self.virtual_width / 2.0, self.virtual_height / 2.0),
            zoom: vec2(1.0 / self.scale.x, 1.0 / self.scale.y),
            offset: self.offset,
            ..Default::default()
        });
    }
}