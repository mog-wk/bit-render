extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::rect::Point;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;


use shapes::*;
mod shapes;


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


pub struct Renderer {
    canvas: WindowCanvas,
    theme: Theme,
}

impl Renderer {
    pub fn new(window: Window, theme: Theme) -> Result<Self, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Self { canvas, theme })
    }

    pub fn render(&mut self, emmiter_list: &Vec<Emmiter>) {
        // background
        self.canvas.set_draw_color(self.theme.background);
        self.canvas.clear();

        // nodes
        // NOTE redering multiple circles breaks the render

        self.canvas.set_draw_color(self.theme.node_1);
        for emmiter in emmiter_list {
            let node_color = match emmiter.state {
                true => self.theme.node_1,
                false => self.theme.node_0,
            };
            self.canvas.set_draw_color(node_color);
            draw_circle(&mut self.canvas, emmiter.x(), emmiter.y(), 16);
        }
        self.canvas.present();
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
    pub loc: Point,
    pub state: bool,
}

impl Emmiter {
    /// new node at middle of canvas and Off
    pub fn new() -> Self {
        Self { loc: Point::new( (WIDTH / 2) as i32, (HEIGHT / 2) as i32 ), state: false }
    }
    /// new node from data
    pub fn from(x: u32, y: u32, state: bool) -> Result<Self, String> {
        if x >= WIDTH || y >= HEIGHT {
            return Err(format!("invalid input for node {} {}", x, y));
        }
        Ok(Self { loc: Point::new(x as i32, y as i32), state })
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

#[derive(Debug)]
pub struct Wire {

}


#[allow(dead_code)]
impl Wire {

}

#[allow(dead_code)]
struct UI {

}
