extern crate sdl2;

use sdl2::rect::{ Point, Rect };
use sdl2::video::{ WindowContext };
use sdl2::pixels::Color;
use sdl2::render::{ WindowCanvas, TextureCreator };
use std::collections::HashSet;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub enum InputMode {
    Emmiter,
    Receiver,
    Wire,
    Function,
}

pub enum ColorNames {
    Black = 0x141414,
    AbsoluteBlack = 0x000000,
    Crinson = 0x500000,
    Red = 0xc80000,
}

pub struct Theme {
    pub background: Color,
    pub node_1: Color,
    pub node_0: Color,
    pub wire: Color,
    pub text: Color,
}


impl Theme {
    pub fn black() -> Self {
        Self {
            background: Color::RGB(20, 20, 20),
            node_1: Color::RGB(120, 80, 0),
            node_0: Color::RGB(120, 0, 0),
            wire: Color::RGB(120, 0, 0),
            text: Color::RGB(180, 180, 180),
        }
    }
}


pub trait Node {
    fn x(&self) -> i32; 
    fn y(&self) -> i32;
    fn set_loc(&mut self, x: i32, y: i32) -> ();
    fn get_loc(&self) -> (i32, i32);
    fn switch_state(&mut self) -> ();
}


#[derive(Debug)]
pub struct Emmiter {
    loc: Point,
    pub state: bool,
    pub connections: HashSet<Wire>,
}

impl Emmiter {
    /// new node at middle of canvas and Off
    pub fn new() -> Self {
        Self { loc: Point::new( (WIDTH / 2) as i32, (HEIGHT / 2) as i32 ),
        state: false,
        connections: HashSet::new(), }
    }
    /// new node from data
    pub fn from(x: u32, y: u32, state: bool, connections: HashSet<Wire>)
        -> Result<Self, String> {
        if x >= WIDTH || y >= HEIGHT {
            return Err(format!("invalid input for node {} {}", x, y));
        }
        Ok(Self { loc: Point::new(x as i32, y as i32), state, connections })
    }
    pub fn add_wire(emmiter: &mut Emmiter, wire: Wire) {
        //-> Result<(), String> {
        emmiter.connections.insert(wire);
    }
}

impl Node for Emmiter {
    fn x(&self) -> i32 {
        self.loc.x
    }
    fn y(&self) -> i32 {
        self.loc.y
    }
    fn set_loc(&mut self, x: i32, y: i32) {
        self.loc = Point::new(x,y);
    }
    fn get_loc(&self) -> (i32, i32) {
        (self.loc.x, self.loc.y)
    }
    fn switch_state(&mut self) {
        self.state = !self.state;
    }
}

pub fn emmit_signal(emmiter: &Emmiter) {
    for wire in emmiter.connections.iter() {

    }
}

struct Receiver {
    loc: Point,
    state: bool,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Wire {

}


#[allow(dead_code)]
impl Wire {

}

#[allow(dead_code)]
struct UI {

}

pub fn render_text(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, font: &sdl2::ttf::Font, text: &str) -> Result<(), String> {

    // test
    let hello_text: String = "Hello World".to_string();
    let surface = font.render(&hello_text).blended(Color::RGBA(127, 127, 0, 128)).map_err(|e| e.to_string())?;

    let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;

    let target = Rect::new(10, 0, 400, 200);

    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}

/// get closest node from point, returns a tuple with the node and the distance
pub fn node_closest_dist_get<T>(node_list: &Vec<T>, x: i32, y: i32)
    -> Option<(u32, usize)> where T: Node {
    if !(node_list.len() > 0) {
        return None
    } 
    let mut closest_dist = WIDTH;
    let mut closest_index = 0;
    let mut index = 0;
    for node in node_list.iter() {
        let cur_dist = dist(x, y, node.x(), node.y());

        if cur_dist < closest_dist {
            closest_dist = cur_dist;
            closest_index = index;
        }
        index += 1;
    }
    Some( (closest_dist, closest_index as usize) )
}

/// distance between two points
pub fn dist(x: i32, y: i32, x1: i32, y1: i32) -> u32 {
    let d = ( ( (y1 - y).pow(2) + (x1 - x).pow(2) ) as f64).sqrt();
    if d < 0_f64 {
        return (d * -1.0) as u32
    } else {
        d as u32
    }
}
