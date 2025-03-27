extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer;


pub fn main(){

    let sdl_contex = sdl2::init().unwrap();
    let video_subsystem = sdl_contex.video().unwrap();

    let window = video_subsystem.window("Audio", 1400, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_contex.event_pump().unwrap();

    mixer::open_audio(44100, mixer::DEFAULT_FORMAT, 2, 4096).unwrap();

    let cannon = mixer::Chunk::from_file("./audios/cannon.wav").unwrap();

    let channel = mixer::Channel(0);

    'running: loop{

        for event in event_pump.poll_iter(){
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} =>{
                    break 'running;
                } |
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    mixer::Channel::play(channel, &cannon, 0).unwrap();
                },
                _ => {}
            }
        }

    }

}