extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 500;
const SCREEN_HEIGHT: u32 = 500;

fn put_pixel(canvas: &WindowCanvas, x: i16, y: i16, color: u32) {
    canvas.pixel(x, y, color).unwrap();
}

fn line(canvas: &WindowCanvas, x0: i16, y0: i16, x1: i16, y1: i16) {
    for i in 0..200 {
        let step = i as f32 / 200.0;
        let x = x0 as f32 * (1.0 - step) + x1 as f32 * step;
        let y = y0 as f32 * (1.0 - step) + y1 as f32 * step;

        put_pixel(canvas, x as i16, y as i16, 0xFFFFFFFFu32);
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

    'main: loop {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        line(&canvas, 20, 20, 200, 60);
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
