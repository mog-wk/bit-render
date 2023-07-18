extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::thread::sleep;
use std::collections::HashSet;


use bit_render::*;



/// create sdl2 color themes; eventually create in a seperate file


#[allow(dead_code, unused_imports)]
fn main() -> Result<(), String> {
    println!("Hello, world!");

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

    let window = video_subsystem.window("test", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let theme = Theme::black();
    let mut canvas = Renderer::new(window, theme)?;


    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;
    let buttons_buffer = HashSet::new();


    //let mut node_list = Vec::<Node>::new();

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                _ => {},
            }
        }
        canvas.render();

        let mouse_state = event_pump.mouse_state();
        let mouse_buttons = mouse_state.pressed_mouse_buttons().collect();
        let new_buttons = &mouse_buttons - &buttons_buffer;
        let old_buttons = &buttons_buffer - &mouse_buttons;


        if !new_buttons.is_empty() || !old_buttons.is_empty() {
            println!("X = {:?}, Y ={:?}, {:?} : {:?}", mouse_state.x(), mouse_state.y(), old_buttons, new_buttons);
        }
        sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }


    Ok(())
}
