extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main(){

    let sdl_contex = sdl2::init().unwrap();
    let video_subsystem = sdl_contex.video().unwrap();

    let window = video_subsystem.window("Creating window", 1000, 700)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_contex.event_pump().unwrap();

    'running: loop{

        for event in event_pump.poll_iter(){
            match event{
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>{
                    break 'running;
                },
                _ => {}

            }
        }

    }

}