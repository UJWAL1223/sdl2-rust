extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
pub fn main(){

    let sdl_contex = sdl2::init().unwrap();
    let video_subsystem = sdl_contex.video().unwrap();

    let window = video_subsystem.window("Moving Rectangles", 1400, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_contex.event_pump().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut x_pos = 600;
    let mut y_pos = 200;

    'running: loop{
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(Rect::new(x_pos, y_pos, 200, 100));

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape),..} => {
                    break 'running;
                } |
                Event::KeyDown { keycode:Some(Keycode::W),.. } => {
                    y_pos -= 5;
                } |
                Event::KeyDown { keycode:Some(Keycode::S),.. } => {
                    y_pos += 5;
                } |
                Event::KeyDown { keycode:Some(Keycode::A),.. } => {
                    x_pos -= 5;
                } |
                Event::KeyDown { keycode:Some(Keycode::D),.. } => {
                    x_pos += 5;
                }
                _ => {}
            }
        }

        canvas.present();

    }

}