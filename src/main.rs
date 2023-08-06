extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::{ WindowCanvas };
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use std::time::Duration;
use std::thread::sleep;
use std::collections::HashSet;
use std::path::Path;

use bit_render::*;

mod render;
use render::shapes::draw_circle;

/// create color themes; eventually create in a seperate file

const DEBUG: bool = true;

#[allow(dead_code)]
pub fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("test", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let theme = Theme::black();
    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    // prepare fonts
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path: &Path = Path::new(&"fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 64).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut mouse_buttons_buffer = HashSet::new();

    let mut emmiter_list: Vec<Emmiter> = Vec::new();
    let mut emmiter_list_buffer: Vec<(i32, i32)> = Vec::new();
    let mut wire_list: Vec<Wire> = Vec::new();

    let radius = 16;

    let mut input_mode = InputMode::Emmiter;

    let mut inputted: bool = false;
    let mut move_mode: bool = true;

    let mut frame_count: u32 = 0;
    const INPUT_DELAY: u32 = 40;

    canvas.set_draw_color(theme.background);
    canvas.clear();
    render_text(&mut canvas, &texture_creator, &font, &"123").unwrap();


    'run: loop {
        // --------------------------------------------------------------------
        // Input Handler
        // --------------------------------------------------------------------
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                // Mode Change 
                Event::KeyDown { keycode: Some(Keycode::Q), .. } =>
                    input_mode = InputMode::Emmiter,
                Event::KeyDown { keycode: Some(Keycode::W), .. } =>
                    input_mode = InputMode::Wire,
                Event::KeyDown { keycode: Some(Keycode::E), .. } =>
                    input_mode = InputMode::Receiver,
                Event::KeyDown { keycode: Some(Keycode::R), .. } =>
                    input_mode = InputMode::Function,

                //_ => println!("{:?}", event),
                _ => (),
            }
        }
        let mouse_state = event_pump.mouse_state();
        let mouse_buttons = mouse_state.pressed_mouse_buttons().collect();
        // mouse pressed
        let mouse_new_buttons = &mouse_buttons - &mouse_buttons_buffer;
        // mouse released
        let mouse_old_buttons = &mouse_buttons_buffer - &mouse_buttons;

        // --------------------------------------------------------------------
        // Update
        // --------------------------------------------------------------------
        if (!mouse_new_buttons.is_empty() || !mouse_old_buttons.is_empty()) && !inputted {
            println!("X = {:?}, Y ={:?}, {:?} : {:?}",
                     mouse_state.x(), mouse_state.y(),
                     mouse_old_buttons, mouse_new_buttons);
            match input_mode {
                InputMode::Emmiter => 'input_action: {
                    if mouse_new_buttons.contains(&MouseButton::Left) {
                        // add emmiter
                        if emmiter_list.len() == 0 {
                            // if there are no nodes just add on spot
                            emmiter_list.push( Emmiter::from(
                                    mouse_state.x() as u32,
                                    mouse_state.y() as u32,
                                    true, HashSet::new()).unwrap()
                                ); 
                        } else {
                            // if there are nodes, check to avoid intercection
                            // find closest node's distance and index
                            let closest_node_data: (u32, usize) = node_closest_dist_get(
                                &emmiter_list, mouse_state.x(), mouse_state.y()
                                ) .unwrap();
                            if closest_node_data.0 < radius as u32 {
                                // if intersects change state of node
                                let node = &mut emmiter_list[closest_node_data.1];
                                node.state = !node.state;
                            } else {
                                // add emmiter
                                emmiter_list.push(Emmiter::from(mouse_state.x() as u32,
                                mouse_state.y() as u32, true, HashSet::new()).unwrap() ); 
                            }
                        }
                        inputted = true;

                    } else if mouse_new_buttons.contains(&MouseButton::Middle) {
                        // move node while middle mouse button is pressed
                        // if there are no nodes do nothing
                        // TODO make move smoothier
                        // TODO make check for intersection with mouse pointer and
                        // closest node but make move range not dependent on closest
                        // node's radius
                        if emmiter_list.len() == 0 {
                            break 'input_action;
                        }
                        // find node
                        let closest_node_data: (u32, usize) = node_closest_dist_get(
                            &emmiter_list, mouse_state.x(), mouse_state.y()).unwrap();
                        
                        // mouse intersects node
                        if closest_node_data.0 <= radius as u32 {
                            // move
                            // add move_node to buffer
                            emmiter_list_buffer.push(emmiter_list[closest_node_data.1].get_loc());
                            emmiter_list[closest_node_data.1].set_loc(
                                mouse_state.x(), mouse_state.y()
                                ); 
                            move_mode = true;
                        }
                    } else if mouse_old_buttons.contains(&MouseButton::Middle) {
                            move_mode = false;
                            inputted = true;

                    } else if mouse_new_buttons.contains(&MouseButton::Right) {
                        // removes node
                        let closest_node_data = match node_closest_dist_get(
                            &emmiter_list, mouse_state.x(), mouse_state.y()
                            ) {
                            Some(v) => v,
                            None => break 'input_action,
                        };
                        if closest_node_data.0 < radius as u32 {
                            emmiter_list_buffer.push(
                                emmiter_list[closest_node_data.1].get_loc()
                                );
                            emmiter_list.remove(closest_node_data.1);
                        }
                    }
                    
                },
                InputMode::Wire => {
                    // add wire on left click
                    if mouse_new_buttons.contains(&MouseButton::Left) {
                        let closest_node_data = node_closest_dist_get(
                            &emmiter_list, mouse_state.x(), mouse_state.y()).unwrap();
                        println!("{:?}", closest_node_data );

                    }
                },
            InputMode::Function => {
            }
                _ => panic!("invalid input_mode"),
            }

            if !move_mode {
                mouse_buttons_buffer = mouse_new_buttons;
            } 
            
            println!("{}", emmiter_list.len());
        }
        
        // --------------------------------------------------------------------
        // render
        // --------------------------------------------------------------------

        // background
        canvas.set_draw_color(theme.background);
        // clear previous locations if any

        for emmiter_location in emmiter_list_buffer {
            draw_circle(&mut canvas, emmiter_location.0, emmiter_location.1, radius);
        }


        // nodes
        canvas.set_draw_color(theme.node_1);
        for emmiter in emmiter_list.iter() {
            let node_color = match emmiter.state {
                true => theme.node_1,
                false => theme.node_0,
            };
            canvas.set_draw_color(node_color);
            draw_circle(&mut canvas, emmiter.x(), emmiter.y(), 16);
        }
            // render DEBUG
            if DEBUG {
                // display input node
                //let alp = 0.3;
                match input_mode {
                    InputMode::Emmiter => {
                        canvas.set_draw_color(Color::RGB(120, 0, 0));
                    },
                    InputMode::Wire => {
                        canvas.set_draw_color(Color::RGB(0, 120, 0));
                    },
                    InputMode::Receiver => {
                        canvas.set_draw_color(Color::RGB(120, 120, 0));
                    },
                    _ => (),
                    /*
                    InputMode::Func() => {
                        canvas.set_draw_color(Color::RGBA(0, 0, 120, alp));
                    },
                    */
                }

                canvas.fill_rect(
                    sdl2::rect::Rect::new(WIDTH as i32 - 32, 2, 30, 30)
                    ).unwrap();
            }
        emmiter_list_buffer =  Vec::new();
        canvas.present();

        if frame_count % INPUT_DELAY == 0 {
            inputted = false;
        } else {
        }

        frame_count += 1;

        sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }

}

