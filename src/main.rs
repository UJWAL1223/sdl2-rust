extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() -> Result<(), String>{
    let sdl2_contex = sdl2::init()?;
    let video_subsystem = sdl2_contex.video()?;

    let window = video_subsystem.window("Brick Breaker", 1200, 700)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl2_contex.event_pump().map_err(|e| e.to_string())?;

    'running: loop{
        for event in event_pump.poll_iter(){
            match event{
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape),.. } => {
                    break 'running;
                },
                _ => {}
            }
        }
    }
    
    Ok(())
}