extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use std::time::Duration;
use std::thread::sleep;
use std::collections::HashSet;

use bit_render::*;

/// create color themes; eventually create in a seperate file


#[allow(dead_code)]
fn main() {
    println!("Hello, world!");

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("test", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let theme = Theme::black();
    let mut canvas = Renderer::new(window, theme).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let buttons_buffer = HashSet::new();

    // TODO make into heap for enchanced manipulation
    let mut node_list: Vec<Node> = Vec::new();
    let radius = 8;

    node_list.push(Node::new(100, 100, false).unwrap());

    let mut input_mode = InputMode::Node;

    let mut inputted = false;

    let mut frame_count = 0;

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                _ => {},
            }
        }

        canvas.render(&node_list);

        let mouse_state = event_pump.mouse_state();
        let mouse_buttons = mouse_state.pressed_mouse_buttons().collect();
        let new_buttons = &mouse_buttons - &buttons_buffer;
        let old_buttons = &buttons_buffer - &mouse_buttons;

        if (!new_buttons.is_empty() || !old_buttons.is_empty()) && !inputted {
            println!("X = {:?}, Y ={:?}, {:?} : {:?}", mouse_state.x(), mouse_state.y(), old_buttons, new_buttons);
            match input_mode {
                InputMode::Node => {
                    if node_list.len() > 0 {
                        if new_buttons.contains(&MouseButton::Left) {
                            // find closest node
                            let mut closest = Box::new(&node_list[0]);
                            let mut closest_dist = WIDTH;
                            for node in &node_list[0..] {
                                let cur_dist = dist(mouse_state.x(), mouse_state.y(), node.x(), node.y());
                                  println!("===================================");
                                  println!("node: {:?}", node);
                                  println!("closest: {:?}", *closest);
                                  println!("cur_dist: {:?}", cur_dist);
                                  println!("closest_dist: {:?}", closest_dist);
                                if cur_dist < closest_dist - radius {
                                    closest_dist = cur_dist;
                                    closest = Box::new(node);
                                }
                            }
                            // add node
                            if closest_dist >= radius as u32 * 2 {
                                node_list.push(Node::new(mouse_state.x() as u32,
                                mouse_state.y() as u32, true).unwrap() ); 
                            } else {
                                println!("could not add not due to proximity");
                            }

                            inputted = true;
                        } else if new_buttons.contains(&MouseButton::Middle) {
                            // move node
                            ();
                            inputted = true;
                         }
                    }
                },
                _ => panic!("invalid input_mode"),
            }
            println!("{}", node_list.len());
        }

        if frame_count % 30 == 0 {
            inputted = false;
        }

        frame_count += 1;

        sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }

}

fn dist(x: i32, y: i32, x1: i32, y1: i32) -> u32 {
    let d = ( ( (y1 - y).pow(2) + (x1 - x).pow(2) ) as f64).sqrt();
    if d < 0_f64 {
        return (d * -1.0) as u32
    } else {
        d as u32
    }
}
    
