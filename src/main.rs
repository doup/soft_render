extern crate sdl2;
extern crate wavefront_obj;
extern crate rand;

mod vec2f;
mod vec3f;

use vec2f::Vec2f;
use vec3f::Vec3f;

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

#[derive(Debug, Copy, Clone)]
struct Vector2<T> {
    x: T,
    y: T,
}

impl Vector2<i16> {
    fn new() -> Vector2<i16> {
        Vector2 {
            x: 0,
            y: 0,
        }
    }
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

fn barycentric(tri: &[Vec2f; 3], p: Vec2f) -> Vec3f {
    let u = Vec3f::new(tri[2].x - tri[0].x, tri[1].x - tri[0].x, tri[0].x - p.x).cross(&Vec3f::new(tri[2].y - tri[0].y, tri[1].y - tri[0].y, tri[0].y - p.y));

    // triangle is degenerate, in this case return something with negative coordinates 
    if u.z.abs() < 1.0 {
        return Vec3f::new(-1.0, 1.0, 1.0)
    }

    Vec3f::new(
        1.0 - (u.x + u.y) / u.z,
        u.y / u.z,
        u.x / u.z
    )
}

fn triangle(canvas: &WindowCanvas, tri: &[Vec2f; 3], color: pixels::Color) {
    let bbox_min = Vec2f::new(
        tri.iter().map(|v| v.x).fold(std::f64::INFINITY, |a, b| a.min(b).max(0.0)), // Left
        tri.iter().map(|v| v.y).fold(std::f64::INFINITY, |a, b| a.min(b).max(0.0))  // Bottom
    );

    let bbox_max = Vec2f::new(
        tri.iter().map(|v| v.x).fold(std::f64::NEG_INFINITY, |a, b| a.max(b).min((SCREEN_WIDTH - 1) as f64)),  // Right
        tri.iter().map(|v| v.y).fold(std::f64::NEG_INFINITY, |a, b| a.max(b).min((SCREEN_HEIGHT - 1) as f64))  // Top
    );

    for x in (bbox_min.x as i16)..(bbox_max.x.ceil() as i16) {
        for y in (bbox_min.y as i16)..(bbox_max.y.ceil() as i16) {
            let p = Vec2f::new(x as f64 + 0.5, y as f64 + 0.5);
            let bc_screen = barycentric(tri, p);

            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            put_pixel(canvas, p.x.floor() as i16, p.y.floor() as i16, color);
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

    let mut model = vec![];

    for shape in shapes {
        match shape.primitive {
            wavefront_obj::obj::Primitive::Triangle(vtx_1, vtx_2, vtx_3) => {
                model.push([
                    Vec3f::new(
                        object.vertices[vtx_1.0].x,
                        object.vertices[vtx_1.0].y,
                        object.vertices[vtx_1.0].z
                    ),
                    Vec3f::new(
                        object.vertices[vtx_2.0].x,
                        object.vertices[vtx_2.0].y,
                        object.vertices[vtx_2.0].z
                    ),
                    Vec3f::new(
                        object.vertices[vtx_3.0].x,
                        object.vertices[vtx_3.0].y,
                        object.vertices[vtx_3.0].z
                    ),
                ]);
            },
            _ => {}
        }
    }

    // let mut t0 = vec![Vector2 {x: 10,  y: 70},  Vector2 {x: 50,  y: 160}, Vector2 {x: 70,  y: 80}]; 
    // let mut t1 = vec![Vector2 {x: 180, y: 50},  Vector2 {x: 150, y: 1},   Vector2 {x: 70,  y: 180}]; 
    // let mut t2 = vec![Vector2 {x: 180, y: 150}, Vector2 {x: 120, y: 160}, Vector2 {x: 130, y: 180}]; 

    // triangle(&canvas, &mut t0, pixels::Color::RGB(255, 0, 0)); 
    // triangle(&canvas, &mut t1, pixels::Color::RGB(255, 255, 255)); 
    // triangle(&canvas, &mut t2, pixels::Color::RGB(0, 255, 0)); 

    // canvas.present();

    let mut angle = 0.0;

    'main: loop {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        let angle_rad: f64 = (angle * 2.0 * 3.14159) / 360.0;

        for tri in model.iter() {
            // Screen coordinates triangle
            let mut tri_screen = [Vec2f::new_zero(); 3];

            for i in 0..3 {
                tri_screen[i] = Vec2f::new(
                    (tri[i].x + 1.0) * SCREEN_WIDTH as f64 / 2.0,
                    (tri[i].y + 1.0) * SCREEN_HEIGHT as f64 / 2.0
                );
            }

            // Get triangle normal
            let a = tri[2] - tri[0];
            let b = tri[1] - tri[0];
            let normal = a.cross(&b).normalize();

            // Get light intensity
            let light = Vec3f::new(angle_rad.sin(), 0.0, angle_rad.cos());
            let light_intensity = normal.dot(&light);
            let color = (255.0 * light_intensity) as u8;

            if light_intensity > 0.0 {
                triangle(&canvas, &mut tri_screen, pixels::Color::RGB(color, color, color));
            }
        }

        angle += 5.0;

        if angle > 360.0 {
            angle -= 360.0;
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
