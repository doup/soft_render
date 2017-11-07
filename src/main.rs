extern crate sdl2;
extern crate rand;
extern crate wavefront_obj;

mod model;
mod vec2f;
mod vec3f;

use model::load_model;
use vec2f::Vec2f;
use vec3f::Vec3f;

use std::path::PathBuf;
use std::f64;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;
const SCREEN_BUFFER: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

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

fn barycentric(tri: &[Vec3f; 3], p: Vec3f) -> Vec3f {
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

fn triangle(canvas: &WindowCanvas, tri: &[Vec3f; 3], color: pixels::Color, zbuffer: &mut [f64; SCREEN_BUFFER]) {
    let bbox_min = Vec2f::new(
        tri.iter().map(|v| v.x).fold(std::f64::INFINITY, |a, b| a.min(b).max(0.0)), // Left
        tri.iter().map(|v| v.y).fold(std::f64::INFINITY, |a, b| a.min(b).max(0.0))  // Bottom
    );

    let bbox_max = Vec2f::new(
        tri.iter().map(|v| v.x).fold(f64::NEG_INFINITY, |a, b| a.max(b).min((SCREEN_WIDTH - 1) as f64)), // Right
        tri.iter().map(|v| v.y).fold(f64::NEG_INFINITY, |a, b| a.max(b).min((SCREEN_HEIGHT - 1) as f64)) // Top
    );

    for x in (bbox_min.x as usize)..(bbox_max.x.ceil() as usize) {
        for y in (bbox_min.y as usize)..(bbox_max.y.ceil() as usize) {
            let mut sample = Vec3f::new(x as f64 + 0.5, y as f64 + 0.5, 0.0);
            let bc_screen = barycentric(tri, sample);

            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            sample.z = tri[0].z * bc_screen.x + tri[1].z * bc_screen.y + tri[2].z * bc_screen.z;

            let zindex = (y * SCREEN_WIDTH as usize) + x;

            if zbuffer[zindex] < sample.z {
                zbuffer[zindex] = sample.z;
                put_pixel(canvas, x as i16, y as i16, color);
            }
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
    let mut model_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    model_path.push("assets/african_head.obj");

    let model = load_model(model_path);

    'main: loop {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        let mut zbuffer = [f64::NEG_INFINITY; SCREEN_BUFFER];

        for tri in model.iter() {
            // Screen coordinates triangle
            let mut tri_screen = [Vec3f::new_zero(); 3];

            for i in 0..3 {
                tri_screen[i] = Vec3f::new(
                    (tri[i].x + 1.0) * SCREEN_WIDTH as f64 / 2.0,
                    (tri[i].y + 1.0) * SCREEN_HEIGHT as f64 / 2.0,
                    tri[i].z
                );
            }

            // Get triangle normal
            let a = tri[2] - tri[0];
            let b = tri[1] - tri[0];
            let normal = a.cross(&b).normalize();

            // Get light intensity
            let light = Vec3f::new(0.0, 0.0, -1.0);
            let light_intensity = normal.dot(&light);
            let color = (255.0 * light_intensity) as u8;

            if light_intensity > 0.0 {
                triangle(&canvas, &mut tri_screen, pixels::Color::RGB(color, color, color), &mut zbuffer);
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
