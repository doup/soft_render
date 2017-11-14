extern crate image;
extern crate rand;
extern crate sdl2;
extern crate wavefront_obj;

mod model;
mod vec2f;
mod vec3f;
mod triangle;

use model::load_model;
use vec2f::Vec2f;
use vec3f::Vec3f;
use triangle::Triangle;

use std::f64;
use std::path::PathBuf;

use image::GenericImage;

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

fn triangle(canvas: &WindowCanvas, tri: &Triangle, tri_screen: &[Vec2f; 3], pixel_shader: &PixelShader, zbuffer: &mut [f64; SCREEN_BUFFER]) {
    let bbox_min = Vec2f::new(
        tri_screen.iter().map(|v| v.x).fold(std::f64::INFINITY, |a, b| a.min(b).max(0.0)), // Left
        tri_screen.iter().map(|v| v.y).fold(std::f64::INFINITY, |a, b| a.min(b).max(0.0))  // Bottom
    );

    let bbox_max = Vec2f::new(
        tri_screen.iter().map(|v| v.x).fold(f64::NEG_INFINITY, |a, b| a.max(b).min((SCREEN_WIDTH - 1) as f64)), // Right
        tri_screen.iter().map(|v| v.y).fold(f64::NEG_INFINITY, |a, b| a.max(b).min((SCREEN_HEIGHT - 1) as f64)) // Top
    );

    for x in (bbox_min.x as usize)..(bbox_max.x.ceil() as usize) {
        for y in (bbox_min.y as usize)..(bbox_max.y.ceil() as usize) {
            let mut sample = Vec2f::new(x as f64 + 0.5, y as f64 + 0.5);
            let bc_screen = barycentric(tri_screen, sample);

            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            let sample_z = tri.vertices[0].z * bc_screen.x + tri.vertices[1].z * bc_screen.y + tri.vertices[2].z * bc_screen.z;
            
            let zindex = (y * SCREEN_WIDTH as usize) + x;

            if zbuffer[zindex] < sample_z {
                zbuffer[zindex] = sample_z;

                let u = tri.uv[0].x * bc_screen.x + tri.uv[1].x * bc_screen.y + tri.uv[2].x * bc_screen.z;
                let v = tri.uv[0].y * bc_screen.x + tri.uv[1].y * bc_screen.y + tri.uv[2].y * bc_screen.z;

                put_pixel(canvas, x as i16, y as i16, pixel_shader.render(u, v));
            }
        }
    }
}

trait PixelShader {
    fn render(&self, u: f64, v: f64) -> pixels::Color;
}

struct ShaderDiffuse {
    image: image::DynamicImage,
    light_intensity: f64,
}

impl PixelShader for ShaderDiffuse {
    fn render(&self, u: f64, v: f64) -> pixels::Color {
        let texture = self.image.get_pixel(
            (u * self.image.width() as f64) as u32,
            (self.image.height() as f64 - v * self.image.height() as f64) as u32
        );

        pixels::Color::RGB(
            (self.light_intensity * (texture.data[0] as f64)) as u8,
            (self.light_intensity * (texture.data[1] as f64)) as u8,
            (self.light_intensity * (texture.data[2] as f64)) as u8
        )
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

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let mut diffuse_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    diffuse_path.push("assets/african_head_diffuse.jpg");
    let diffuse = image::open(diffuse_path).unwrap();

    // Set OBJ file path
    let mut model_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    model_path.push("assets/african_head.obj");

    let model = load_model(model_path);

    // let shader_diffuse = |light_intensity: f64, diffuse: &Box<image::DynamicImage>| {
    //     Box::new(|u: f64, v: f64| -> sdl2::pixels::Color {
    //         let texture = diffuse.get_pixel(
    //             (u * diffuse.width() as f64) as u32,
    //             (diffuse.height() as f64 - v * diffuse.height() as f64) as u32
    //         );

    //         pixels::Color::RGB(
    //             (light_intensity * (texture.data[0] as f64)) as u8,
    //             (light_intensity * (texture.data[1] as f64)) as u8,
    //             (light_intensity * (texture.data[2] as f64)) as u8
    //         )
    //     })
    // };

    // let shader_uv = |light_intensity: f64| {
    //     let color_channel = 255.0 * light_intensity;

    //     move |u: f64, v: f64| -> sdl2::pixels::Color {
    //         pixels::Color::RGB(
    //             (color_channel * u) as u8,
    //             (color_channel * v) as u8,
    //             color_channel as u8
    //         )
    //     }
    // };

    // let shader_flat = |light_intensity: f64| {
    //     let color_channel = (255.0 * light_intensity) as u8;
    //     let color = pixels::Color::RGB(color_channel, color_channel, color_channel);

    //     move |u: f64, v: f64| -> pixels::Color {
    //         color
    //     }
    // };

    let mut shader = ShaderDiffuse {
        image: diffuse,
        light_intensity: 1.0,
    };

    let mut light_dir = Vec3f::new(0.0, 0.0, -1.0);

    'main: loop {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // Reset ZBuffer
        let mut zbuffer = [f64::NEG_INFINITY; SCREEN_BUFFER];

        for tri in model.iter() {
            // Screen coordinates triangle
            let mut tri_screen = [Vec2f::new_zero(); 3];

            for i in 0..3 {
                tri_screen[i] = Vec2f::new(
                    (tri.vertices[i].x + 1.0) * SCREEN_WIDTH as f64 / 2.0,
                    (tri.vertices[i].y + 1.0) * SCREEN_HEIGHT as f64 / 2.0
                );
            }

            let normal = tri.normal();

            // Get light intensity
            let light_intensity = normal.dot(&light_dir);

            shader.light_intensity = if light_intensity < 0.0 { 0.0 } else { light_intensity };

            //let shader = shader_diffuse(light_intensity, &diffuse);
            //let shader = shader_uv(light_intensity);
            //let shader = shader_flat(light_intensity);
            //let shader = shader_diffuse(light_intensity, diffuse.clone());

            triangle(&canvas, &tri, &mut tri_screen, &shader, &mut zbuffer);
        }

        canvas.present();

        for event in events.poll_iter() {
            match event {
                Event::MouseMotion { x, y, .. } => {
                    light_dir = Vec3f::new(
                        -((x as f64 / SCREEN_WIDTH as f64) * 2.0 - 1.0),
                        (y as f64 / SCREEN_HEIGHT as f64) * 2.0 - 1.0,
                        -1.0
                    );

                    light_dir.normalize();
                },
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
