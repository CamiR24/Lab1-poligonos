use raylib::prelude::*;

pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width:u32, height:u32, background_color:Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {

    }

    pub fn set_pixel(&mut self, x: u32, y:u32) {
        
    }

    pub fn set_background_color(&mut self, color: Color) {
        
    }

    pub fn set_current_color(&mut self, color: Color) {
        
    }

    pub fn render_to_file(&self, file_path: &str) {
        
    }
}