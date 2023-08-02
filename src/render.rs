extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use std::time::Duration;
use std::thread::sleep;
use std::collections::HashSet;

use bit_render::*;

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
    let mut canvas = Renderer::new(window, theme).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut mouse_buttons_buffer = HashSet::new();

    // TODO make into heap for enchanced manipulation
    let mut node_list: Vec<Node> = Vec::new();
    let radius = 16;

    //node_list.push(Node::new(100, 100, false).unwrap());

    let mut input_mode = InputMode::Node;

    let mut inputted: bool = false;
    let mut move_mode: bool = true;

    let mut frame_count: u32 = 0;
    const INPUT_DELAY: u32 = 40;

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                // map key to input mode Q => Node; W => Wire; E => ..; R => Funcs
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => input_mode = InputMode::Node,
                Event::KeyDown { keycode: Some(Keycode::W), .. } => input_mode = InputMode::Wire,
                _ => {},
            }
        }

        canvas.render(&node_list);

        let mouse_state = event_pump.mouse_state();
        let mouse_buttons = mouse_state.pressed_mouse_buttons().collect();
        // mouse pressed
        let mouse_new_buttons = &mouse_buttons - &mouse_buttons_buffer;
        // mouse released
        let mouse_old_buttons = &mouse_buttons_buffer - &mouse_buttons;

        if (!mouse_new_buttons.is_empty() || !mouse_old_buttons.is_empty()) && !inputted {
            println!("X = {:?}, Y ={:?}, {:?} : {:?}", mouse_state.x(), mouse_state.y(), mouse_old_buttons, mouse_new_buttons);
            match input_mode {
                InputMode::Node => 'input_action: {
                    if mouse_new_buttons.contains(&MouseButton::Left) {
                        // add node
                        if node_list.len() == 0 {
                            // if there are no nodes just add on spot
                            node_list.push(Node::from(mouse_state.x() as u32,
                            mouse_state.y() as u32, true).unwrap() ); 
                        } else {
                            // if there are nodes, check to avoid intercection
                            // find closest node's distance and index
                            let closest_node_data: (u32, usize) = node_closest_dist_get(&node_list, mouse_state.x(), mouse_state.y()).unwrap();
                            if closest_node_data.0 >= radius as u32 * 2 {
                                node_list.push(Node::from(mouse_state.x() as u32,
                                mouse_state.y() as u32, true).unwrap() ); 
                            } else {
                                // if intersects change state of node
                                let node = &mut node_list[closest_node_data.1];
                                node.state = !node.state;
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
                        if node_list.len() == 0 {
                            break 'input_action;
                        }
                        // find node
                        let closest_node_data: (u32, usize) = node_closest_dist_get(&node_list, mouse_state.x(), mouse_state.y()).unwrap();
                        
                        // mouse intersects node
                        if closest_node_data.0 <= radius as u32 * 2 {
                            // move
                            let move_node_index = closest_node_data.1;
                            node_list[move_node_index].set_loc(mouse_state.x(), mouse_state.y()); 
                            move_mode = true;
                        }
                    } else if mouse_old_buttons.contains(&MouseButton::Middle) {
                            move_mode = false;
                            inputted = true;

                    } else if mouse_new_buttons.contains(&MouseButton::Right) {
                        // removes node

                    }
                    
                },
                _ => panic!("invalid input_mode"),
            }

            if !move_mode {
                mouse_buttons_buffer = mouse_new_buttons;
            } 

            
            println!("{}", node_list.len());

        }

        if frame_count % INPUT_DELAY == 0 {
            inputted = false;
        } else {
        }

        frame_count += 1;

        sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }

}



/// get closest node from point, returns a tuple with the node and the distance
fn node_closest_dist_get(node_list: &Vec<Node>, x: i32, y: i32) -> Result<(u32, usize), String> {
    if !(node_list.len() > 0) {
        return Err("invalid len for node_list".to_string())
    } 
    let mut closest_dist = WIDTH;
    let mut closest_index = 0;
    let mut index = 0;
    for node in node_list.iter() {
        let cur_dist = dist(x, y, node.x(), node.y());
          //println!("===================================");
          //println!("node: {:?}", node);
          //println!("closest: {:?}", *closest);
          //println!("cur_dist: {:?}", cur_dist);
          //println!("closest_dist: {:?}", closest_dist);

        if cur_dist < closest_dist {
            closest_dist = cur_dist;
            closest_index = index;
        }
        index += 1;
    }
    Ok( (closest_dist, closest_index as usize) )
}



/// distance between two points
fn dist(x: i32, y: i32, x1: i32, y1: i32) -> u32 {
    let d = ( ( (y1 - y).pow(2) + (x1 - x).pow(2) ) as f64).sqrt();
    if d < 0_f64 {
        return (d * -1.0) as u32
    } else {
        d as u32
    }
}
    
