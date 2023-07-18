extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::rect::Point;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub const PI: f64 = 3.1415926535897;
//pub const PI: f64 = 3.14159265358979323846264338327950288419716939937510;
pub const HPI: f64 = PI / 2.0;
pub const DPI: f64 = PI * 2.0;


pub enum ColorNames {
    Black = 0x141414,
    AbsoluteBlack = 0x000000,
    Crinson = 0x500000,
    Red = 0xc80000,
}

pub struct Theme {
    pub background: Color,
    pub node: Color,
    pub wire: Color,
    pub text: Color,
}


impl Theme {
    pub fn black() -> Self {
        Self {
            background: Color::RGB(20, 20, 20),
            node: Color::RGB(120, 0, 0),
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

    pub fn render(&mut self) {
        // background
        self.canvas.set_draw_color(self.theme.background);
        self.canvas.clear();

        // nodes
        let node_list = vec![Node{x: 200, y: 200}, Node{x: WIDTH - 200, y: 200}];
        self.canvas.set_draw_color(self.theme.node);
        for node in node_list.iter() {
            self.draw_circle(node.x as i32, node.y as i32, 72);
        }

        self.canvas.present();

    }

    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32) {

        //self.draw_circunference(center_x, center_y, radius);
        let radius = radius as f32;

        let mut theta: f32 = 0.0;

        const RESOLUTION: f32 = 0.0002;

        let center = Point::new(center_x, center_y);
        while theta < HPI as f32 {
            let x = (radius * theta.cos()) as i32;
            let y = (radius * theta.sin()) as i32;

            //println!("{:?} {:?}", (center_x, center_y), (x, y));
            self.canvas.draw_line( center, Point::new(center_x + x, center_y + y) ).unwrap();
            self.canvas.draw_line( center, Point::new(center_x - x, center_y + y) ).unwrap();
            self.canvas.draw_line( center, Point::new(center_x + x, center_y - y) ).unwrap();
            self.canvas.draw_line( center, Point::new(center_x - x, center_y - y) ).unwrap();
            theta += RESOLUTION;
        }
    }


    pub fn draw_circunference(&mut self, center_x: i32, center_y: i32, radius: i32) {
        // begin at upper perpendicular radius

        let diameter = radius * 2;

        let mut x = radius - 1;
        let mut y = 0;

        let mut tx = 1;
        let mut ty = 1;

        let mut err = tx - diameter;

        // TODO make into array of len: (radius * 8 * 35 / 49) + 8 - 1 & -8 
        let mut point_arr: Vec::<Point> = vec![];

        //self.canvas.draw_point(Point::new(center_x, center_y)); // draw point in circle origin
        while x >= y {
            // points for each octanct of the circle
            point_arr.push(Point::new(center_x + x, center_y + y));
            point_arr.push(Point::new(center_x + x, center_y - y));
            point_arr.push(Point::new(center_x - x, center_y + y));
            point_arr.push(Point::new(center_x - x, center_y - y));
            point_arr.push(Point::new(center_x + y, center_y + x));
            point_arr.push(Point::new(center_x + y, center_y - x));
            point_arr.push(Point::new(center_x - y, center_y + x));
            point_arr.push(Point::new(center_x - y, center_y - x));
            
            if err <= 0 {
                y += 1;
                err += ty;
                ty += 2;
            }

            if err > 0 {
                x -= 1;
                tx += 2;
                err += tx - diameter;
            }
        }
        self.canvas.draw_points(&point_arr[..]).unwrap();
    }


    pub fn draw_full_ellipse(&mut self, center_x: i32, center_y: i32, radius: i32) {
        // begin at upper perpendicular radius

        let diameter = radius * 2;

        let mut x = radius - 1;
        let mut y = 0;

        let mut tx = 1;
        let mut ty = 1;

        let mut err = tx - diameter;

        // TODO make into array of len: (radius * 8 * 35 / 49) + 8 - 1 & -8 

        let mut point_arr: Vec::<Point> = vec![];
        //let len: Box<i32> = Box::new(((radius * 8 * 35 * 49) + 8 - 1) & -8);

        self.canvas.draw_point(Point::new(center_x, center_y)).unwrap(); // draw point in circle origin
        while x >= y {
            for i in 1..=x {
                // points for each octanct of the circle
                point_arr.push(Point::new(center_x + i, center_y + y));
                point_arr.push(Point::new(center_x + i, center_y - y));
                point_arr.push(Point::new(center_x - i, center_y + y));
                point_arr.push(Point::new(center_x - i, center_y - y));
            }

            // TODO render is applied twice in circle square center
            for i in 1..=x {
                point_arr.push(Point::new(center_x + y, center_y + i));
                point_arr.push(Point::new(center_x + y, center_y - i));
                point_arr.push(Point::new(center_x - y, center_y + i));
                point_arr.push(Point::new(center_x - y, center_y - i));
            }

            
            if err <= 0 {
                y += 1;
                err += ty;
                ty += 2;
            }

            if err > 0 {
                x -= 1;
                tx += 2;
                err += tx - diameter;
            }
        }
        self.canvas.draw_points(&point_arr[..]).unwrap();
    }
}

struct Node {
    x: u32,
    y: u32,
}


#[allow(dead_code)]
impl Node {
    pub fn new(x: u32, y: u32) -> Result<Self, String> {
        if x >= WIDTH || y >= HEIGHT {
            return Err(format!("invalid input for node {} {}", x, y));
        }
        Ok(Self {x, y})
    }
}


