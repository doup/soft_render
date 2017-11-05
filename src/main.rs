extern crate sdl2;
extern crate wavefront_obj;
extern crate rand;

use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use sdl2::gfx::primitives::DrawRenderer;

use wavefront_obj::obj::Vertex;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;

#[derive(Debug, Clone)]
struct Vector2<T> {
    x: T,
    y: T,
}

impl std::ops::Add for Vector2<i16> {
    type Output = Vector2<i16>;

    fn add(self, other: Vector2<i16>) -> Vector2<i16> {
        Vector2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl std::ops::Sub for Vector2<i16> {
    type Output = Vector2<i16>;

    fn sub(self, other: Vector2<i16>) -> Vector2<i16> {
        Vector2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl std::ops::Mul<f64> for Vector2<i16> {
    type Output = Vector2<i16>;

    fn mul(self, other: f64) -> Vector2<i16> {
        Vector2 {
            x: (self.x as f64 * other) as i16,
            y: (self.y as f64 * other) as i16,
        }
    }
}

fn put_pixel(canvas: &WindowCanvas, x: i16, y: i16, color: pixels::Color) {
    canvas.pixel(x, SCREEN_HEIGHT as i16 - y, color).unwrap();
}

fn line(canvas: &WindowCanvas, mut x0: i16, mut y0: i16, mut x1: i16, mut y1: i16, color: pixels::Color) {
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

fn vec_swap(vec: &mut Vec<Vector2<i16>>, a: usize, b: usize) {
    let tmp = vec[a].clone();
    vec[a] = vec[b].clone();
    vec[b] = tmp;
}

fn triangle(canvas: &WindowCanvas, mut tri: &mut Vec<Vector2<i16>>, color: pixels::Color) {
     // sort the vertices, t0, t1, t2 lower−to−upper (bubblesort yay!) 
    if tri[0].y > tri[1].y { vec_swap(&mut tri, 0, 1); }
    if tri[0].y > tri[2].y { vec_swap(&mut tri, 0, 2); }
    if tri[1].y > tri[2].y { vec_swap(&mut tri, 1, 2); }

    let total_height = tri[2].y - tri[0].y;
    let first_half_height = tri[1].y - tri[0].y;
    let tri_has_flat_bottom = tri[1].y == tri[0].y;

    for i in 0..total_height {
        let is_second_half = i > first_half_height || tri_has_flat_bottom;
        let segment_height = if is_second_half {
            tri[2].y - tri[1].y
        } else {
            first_half_height
        };

        let alpha = i as f64 / total_height as f64;
        let beta = if is_second_half {
            (i as f64 - first_half_height as f64) / segment_height as f64
        } else {
            i as f64 / segment_height as f64
        };

        let mut a: Vector2<i16> = tri[0].clone() + (tri[2].clone() - tri[0].clone()) * alpha;
        let mut b: Vector2<i16> = if is_second_half {
            tri[1].clone() + (tri[2].clone() - tri[1].clone()) * beta
        } else {
            tri[0].clone() + (tri[1].clone() - tri[0].clone()) * beta
        };

        if a.x > b.x {
            std::mem::swap(&mut a, &mut b);
        }

        for x in a.x..b.x {
            put_pixel(&canvas, x, tri[0].y + i, color);
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
    obj_file.push("assets/african_head.obj");

    // Load OBJ
    let mut file = File::open(obj_file).expect("Unable to open the file");
    let mut obj_string = String::new();
    file.read_to_string(&mut obj_string).expect("Unable to read the file");

    let obj = wavefront_obj::obj::parse(obj_string).unwrap();
    let object = &obj.objects[0];
    let shapes = &object.geometry[0].shapes;

    // let mut t0 = vec![Vector2 {x: 10,  y: 70},  Vector2 {x: 50,  y: 160}, Vector2 {x: 70,  y: 80}]; 
    // let mut t1 = vec![Vector2 {x: 180, y: 50},  Vector2 {x: 150, y: 1},   Vector2 {x: 70,  y: 180}]; 
    // let mut t2 = vec![Vector2 {x: 180, y: 150}, Vector2 {x: 120, y: 160}, Vector2 {x: 130, y: 180}]; 

    // triangle(&canvas, &mut t0, pixels::Color::RGB(255, 0, 0)); 
    // triangle(&canvas, &mut t1, pixels::Color::RGB(255, 255, 255)); 
    // triangle(&canvas, &mut t2, pixels::Color::RGB(0, 255, 0)); 

    // canvas.present();

    'main: loop {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        for shape in shapes {
            match shape.primitive {
                wavefront_obj::obj::Primitive::Triangle(vtx_1, vtx_2, vtx_3) => {
                    let vertices = [
                        object.vertices[vtx_1.0],
                        object.vertices[vtx_2.0],
                        object.vertices[vtx_3.0],
                    ];
                    let bbox = [
                        vertices.iter().map(|v| v.x).fold(std::f64::INFINITY, |a, b| a.min(b)), // Left
                        vertices.iter().map(|v| v.y).fold(std::f64::INFINITY, |a, b| a.min(b)), // Bottom
                        vertices.iter().map(|v| v.x).fold(std::f64::NEG_INFINITY, |a, b| a.max(b)), // Right
                        vertices.iter().map(|v| v.y).fold(std::f64::NEG_INFINITY, |a, b| a.max(b)), // Top
                    ];

                    // if bbox[2] > -0.5 || bbox[3] > -0.5 {
                    //     continue;
                    // }

                    let mut screen_tri = vec![];

                    for i in 0..3 {
                        screen_tri.push(Vector2 {
                            x: ((vertices[i].x + 1.0) * SCREEN_WIDTH as f64 / 2.0) as i16,
                            y: ((vertices[i].y + 1.0) * SCREEN_HEIGHT as f64 / 2.0) as i16,
                        });
                    }

                    triangle(
                        &canvas,
                        &mut screen_tri,
                        pixels::Color::RGB(
                            rand::random::<u8>(),
                            rand::random::<u8>(),
                            rand::random::<u8>()
                        )
                    );
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
