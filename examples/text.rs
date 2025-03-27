extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, TextureQuery};
use sdl2::ttf;


pub fn main(){

    let sdl_contex = sdl2::init().unwrap();
    let video_subsystem = sdl_contex.video().unwrap();
    let ttf_contex = sdl2::ttf::init().unwrap();

    let window = video_subsystem.window("Text", 1200, 800)
    .position_centered()
    .build()
    .unwrap();

    let mut event_pump = sdl_contex.event_pump().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut texture_creator = canvas.texture_creator();

    let font  = ttf_contex.load_font("./fonts/Roboto.ttf", 128).unwrap();
    let surface = font.render("Hello World").blended(Color::RGB(255, 255, 255)).unwrap();
    let texture = texture_creator.create_texture_from_surface(surface).unwrap();

    'running: loop{

        

        for event in  event_pump.poll_iter(){
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode:Some(Keycode::Escape),.. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        canvas.clear();
        let TextureQuery{width,height, .. } = texture.query();
        let target = Rect::new(200, 200, 400, 200);
        canvas.copy(&texture, None,Some(target).unwrap());
        canvas.present();
    }

}