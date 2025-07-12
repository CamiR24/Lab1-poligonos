mod framebuffer;
mod line;
mod utils;

use raylib::prelude::*;
use framebuffer::FrameBuffer;
use utils::{flip_y, fill_polygon, draw_polygon};

fn main() {
    let image_width = 800;
    let image_height = 600;
    let mut framebuffer = FrameBuffer::new(image_width, image_height, Color::WHITE);

    framebuffer.set_background_color(Color::new(255, 255, 255, 255));
    framebuffer.clear();

    //poligono 1

    let polygon_points1 = vec![
        Vector2::new(165.0, flip_y(380.0, image_height)),
        Vector2::new(185.0, flip_y(360.0, image_height)),
        Vector2::new(180.0, flip_y(330.0, image_height)),
        Vector2::new(207.0, flip_y(345.0, image_height)),
        Vector2::new(233.0, flip_y(330.0, image_height)),
        Vector2::new(230.0, flip_y(360.0, image_height)),
        Vector2::new(250.0, flip_y(380.0, image_height)),
        Vector2::new(220.0, flip_y(385.0, image_height)),
        Vector2::new(205.0, flip_y(410.0, image_height)),
        Vector2::new(193.0, flip_y(383.0, image_height)),
    ];

    framebuffer.set_current_color(Color::SKYBLUE);
    fill_polygon(&mut framebuffer, &polygon_points1);

    framebuffer.set_current_color(Color::DARKBLUE);
    draw_polygon(&mut framebuffer, &polygon_points1);

    //poligono 2

    let polygon_points2 = vec![
        Vector2::new(321.0, flip_y(335.0, image_height)),
        Vector2::new(288.0, flip_y(286.0, image_height)),
        Vector2::new(339.0, flip_y(251.0, image_height)),
        Vector2::new(374.0, flip_y(302.0, image_height)),
    ];

    framebuffer.set_current_color(Color::SKYBLUE);
    fill_polygon(&mut framebuffer, &polygon_points2);

    framebuffer.set_current_color(Color::DARKBLUE);
    draw_polygon(&mut framebuffer, &polygon_points2);

    //poligono 3

    let polygon_points3 = vec![
        Vector2::new(377.0, flip_y(249.0, image_height)),
        Vector2::new(411.0, flip_y(197.0, image_height)),
        Vector2::new(436.0, flip_y(249.0, image_height)),
    ];

    framebuffer.set_current_color(Color::SKYBLUE);
    fill_polygon(&mut framebuffer, &polygon_points3);

    framebuffer.set_current_color(Color::DARKBLUE);
    draw_polygon(&mut framebuffer, &polygon_points3);

    //poligono 4

    let polygon_points4 = vec![
        Vector2::new(413.0, flip_y(177.0, image_height)),
        Vector2::new(448.0, flip_y(159.0, image_height)),
        Vector2::new(502.0, flip_y(88.0, image_height)),
        Vector2::new(553.0, flip_y(53.0, image_height)),
        Vector2::new(535.0, flip_y(36.0, image_height)),
        Vector2::new(676.0, flip_y(37.0, image_height)),
        Vector2::new(660.0, flip_y(52.0, image_height)),
        Vector2::new(750.0, flip_y(145.0, image_height)),
        Vector2::new(761.0, flip_y(179.0, image_height)),
        Vector2::new(672.0, flip_y(192.0, image_height)),
        Vector2::new(659.0, flip_y(214.0, image_height)),
        Vector2::new(615.0, flip_y(214.0, image_height)),
        Vector2::new(632.0, flip_y(230.0, image_height)),
        Vector2::new(580.0, flip_y(230.0, image_height)),
        Vector2::new(597.0, flip_y(215.0, image_height)),
        Vector2::new(552.0, flip_y(214.0, image_height)),
        Vector2::new(517.0, flip_y(144.0, image_height)),
        Vector2::new(466.0, flip_y(180.0, image_height)),
    ];

    framebuffer.set_current_color(Color::SKYBLUE);
    fill_polygon(&mut framebuffer, &polygon_points4);

    framebuffer.set_current_color(Color::DARKBLUE);
    draw_polygon(&mut framebuffer, &polygon_points4);

    //poligono 5

    let polygon_points5 = vec![
        Vector2::new(682.0, flip_y(175.0, image_height)),
        Vector2::new(708.0, flip_y(120.0, image_height)),
        Vector2::new(735.0, flip_y(148.0, image_height)),
        Vector2::new(739.0, flip_y(170.0, image_height)),
    ];

    framebuffer.set_current_color(Color::WHITE);
    fill_polygon(&mut framebuffer, &polygon_points5);

    framebuffer.set_current_color(Color::DARKBLUE);
    draw_polygon(&mut framebuffer, &polygon_points5);

    framebuffer.render_to_file("out.png");
    framebuffer.render_to_file("out.bmp");
}
