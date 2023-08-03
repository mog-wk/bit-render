extern crate sdl2;

use sdl2::rect::Point;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::collections::HashSet;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub enum InputMode {
    Emmiter,
    Receiver,
    Wire,
    Func(fn()),
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


#[allow(dead_code)]
pub trait Node {
    fn x(&self) -> i32; 
    fn y(&self) -> i32;
    fn set_loc(&mut self, x: i32, y: i32) -> ();
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

    fn switch_state(&mut self) {
        self.state = !self.state;
    }
}

pub fn emmit_signal(emmiter: &Emmiter) {
    for wire in emmiter.connections.iter() {

    }
}


#[derive(Debug)]
pub struct Wire {

}


#[allow(dead_code)]
impl Wire {

}

#[allow(dead_code)]
struct UI {

}
