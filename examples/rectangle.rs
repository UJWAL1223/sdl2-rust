extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub fn main(){

    let sdl_contex = sdl2::init().unwrap();
    let  video_subsystem = sdl_contex.video().unwrap();

    let window = video_subsystem.window("Rectangle", 1200, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_contex.event_pump().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();



    'running: loop {

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(Rect::new(100, 100, 600, 200));

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(400, 400, 600, 200));

        for event in event_pump.poll_iter(){
            match event {
                Event::Quit { .. } | 
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                _ => {}
            }
        }

        canvas.present();

    }

}