extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::rect::Point;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;


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

        let node_list = vec![Node{x: 200, y: 200}];
        // nodes
        self.canvas.set_draw_color(self.theme.node);
        for node in node_list.iter() {
            self.draw_circle(node.x as i32, node.y as i32, 12);
        }

        self.canvas.present();

    }
    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32) {
        // begin at upper perpendicular radius

        let diameter = radius * 2;

        let mut radius_x = radius - 1;
        let mut radius_y = 0;

        let mut tx = 1;
        let mut ty = 1;

        let mut err = tx - diameter;

        while radius_x >= radius_y {
            // points for each octanct of the circle
            self.canvas.draw_point(Point::new(center_x + radius_x, center_y + radius_y));
            self.canvas.draw_point(Point::new(center_x + radius_x, center_y - radius_y));
            self.canvas.draw_point(Point::new(center_x - radius_x, center_y + radius_y));
            self.canvas.draw_point(Point::new(center_x - radius_x, center_y - radius_y));
            self.canvas.draw_point(Point::new(center_x + radius_y, center_y + radius_x));
            self.canvas.draw_point(Point::new(center_x + radius_y, center_y - radius_x));
            self.canvas.draw_point(Point::new(center_x - radius_y, center_y + radius_x));
            self.canvas.draw_point(Point::new(center_x - radius_y, center_y - radius_x));
;
            if err <= 0 {
                radius_y += 1;
                err += ty;
                ty += 2;
            }

            if err > 0 {
                radius_x -= 1;
                tx += 2;
                err += tx - diameter;
            }
        }
    }
}

struct Circle {
    x: u32,
    y: u32,
    radius: u32,
}

impl Circle {
    fn new(x: u32, y: u32, radius: u32) -> Self {
        Self {x, y, radius}
    }
}

struct Node {
    x: u32,
    y: u32,
}

impl Node {
    pub fn new(x: u32, y: u32) -> Result<Self, String> {
        if x >= WIDTH || y >= HEIGHT {
            return Err(format!("invalid input for node {} {}", x, y));
        }
        Ok(Self {x, y})
    }
}


