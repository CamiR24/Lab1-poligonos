use raylib::prelude::*;
use crate::framebuffer::FrameBuffer;
use crate::line::line;

pub fn flip_y(y: f32, image_height: u32) -> f32 {
    (image_height as f32) - y
}

pub fn draw_polygon(framebuffer: &mut FrameBuffer, points: &[Vector2]) {
    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()];
        line(framebuffer, start, end);
    }
}

pub fn get_bounding_box(polygon: &[Vector2]) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for point in polygon {
        min_x = min_x.min(point.x as i32);
        max_x = max_x.max(point.x as i32);
        min_y = min_y.min(point.y as i32);
        max_y = max_y.max(point.y as i32);
    }

    (min_x, max_x, min_y, max_y)
}

pub fn point_in_polygon(polygon: &[Vector2], x: f32, y: f32) -> bool {
    let mut inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let xi = polygon[i].x;
        let yi = polygon[i].y;
        let xj = polygon[j].x;
        let yj = polygon[j].y;

        let intersect = ((yi > y) != (yj > y))
            && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);

        if intersect {
            inside = !inside;
        }

        j = i;
    }

    inside
}

pub fn fill_polygon(framebuffer: &mut FrameBuffer, polygon: &[Vector2]) {
    let (min_x, max_x, min_y, max_y) = get_bounding_box(polygon);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let xf = x as f32;
            let yf = y as f32;

            if point_in_polygon(polygon, xf, yf) {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}


