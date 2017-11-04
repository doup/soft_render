extern crate sdl2;
extern crate wavefront_obj;

use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;

fn put_pixel(canvas: &WindowCanvas, x: i16, y: i16, color: u32) {
    canvas.pixel(x, SCREEN_HEIGHT as i16 - y, color).unwrap();
}

fn line(canvas: &WindowCanvas, mut x0: i16, mut y0: i16, mut x1: i16, mut y1: i16, color: u32) {
    let mut steep = false;

    // If is longer vertically than horizontally swap x-y components
    if (x0 - x1).abs() < (y0 - y1).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    // Swap points if necessary
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    for x in x0..x1 {
        let t = (x - x0) as f32 / (x1 - x0) as f32;
        let y = y0 as f32 * (1.0 - t) + y1 as f32 * t;

        if steep {
            put_pixel(canvas, y as i16, x as i16, color);
        } else {
            put_pixel(canvas, x as i16, y as i16, color);
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("soft_render", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    // Set OBJ file path
    let mut obj_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    obj_file.push("src/african_head.obj");

    // Load OBJ
    let mut file = File::open(obj_file).expect("Unable to open the file");
    let mut obj_string = String::new();
    file.read_to_string(&mut obj_string).expect("Unable to read the file");

    let obj = wavefront_obj::obj::parse(obj_string).unwrap();
    let object = &obj.objects[0];
    let shapes = &object.geometry[0].shapes;

    'main: loop {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        for shape in shapes {
            match shape.primitive {
                wavefront_obj::obj::Primitive::Triangle(vtx_1, vtx_2, vtx_3) => {
                    let triangle = [vtx_1.0, vtx_2.0, vtx_3.0];

                    for i in 0..2 {
                        let a = object.vertices[triangle[i]];
                        let b = object.vertices[triangle[(i + 1) % 3]];

                        let x0 = (a.x + 1.0) * SCREEN_WIDTH as f64 / 2.0;
                        let y0 = (a.y + 1.0) * SCREEN_HEIGHT as f64 / 2.0;
                        let x1 = (b.x + 1.0) * SCREEN_WIDTH as f64 / 2.0;
                        let y1 = (b.y + 1.0) * SCREEN_HEIGHT as f64 / 2.0;

                        line(&canvas, x0 as i16, y0 as i16, x1 as i16, y1 as i16, 0xFFFFFFFFu32); 
                    }
                },
                _ => {}
            }
        }

        canvas.present();

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        break 'main
                    }
                },
                _ => {},
            }
        }
    }
}
