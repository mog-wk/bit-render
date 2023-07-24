use sdl2::rect::Point;

use crate::Renderer;

pub const PI: f64 = 3.1415926;
pub const HPI: f64 = PI / 2.0;
pub const DPI: f64 = PI * 2.0;


#[allow(dead_code)]
pub fn draw_circle(render: &mut Renderer, center_x: i32, center_y: i32, radius: i32) {

    let radius = radius as f32;

    let mut theta: f32 = 0.0;

    const RESOLUTION: f32 = 0.0002;

    let center = Point::new(center_x, center_y);
    while theta < HPI as f32 {
        let x = (radius * theta.cos()) as i32;
        let y = (radius * theta.sin()) as i32;

        //println!("{:?} {:?}", (center_x, center_y), (x, y));
        render.canvas.draw_line( center, Point::new(center_x + x, center_y + y) ).unwrap();
        render.canvas.draw_line( center, Point::new(center_x - x, center_y + y) ).unwrap();
        render.canvas.draw_line( center, Point::new(center_x + x, center_y - y) ).unwrap();
        render.canvas.draw_line( center, Point::new(center_x - x, center_y - y) ).unwrap();
        theta += RESOLUTION;
    }
}


#[allow(dead_code)]
pub fn draw_circunference(render: &mut Renderer, center_x: i32, center_y: i32, radius: i32) {
    // begin at upper perpendicular radius

    let diameter = radius * 2;

    let mut x = radius - 1;
    let mut y = 0;

    let mut tx = 1;
    let mut ty = 1;

    let mut err = tx - diameter;

    // TODO make into array of len: (radius * 8 * 35 / 49) + 8 - 1 & -8 
    let mut point_arr: Vec::<Point> = vec![];

    //render.canvas.draw_point(Point::new(center_x, center_y)); // draw point in circle origin
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
    render.canvas.draw_points(&point_arr[..]).unwrap();
}


#[allow(dead_code)]
pub fn draw_full_ellipse(render: &mut Renderer, center_x: i32, center_y: i32, radius: i32) {
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

    render.canvas.draw_point(Point::new(center_x, center_y)).unwrap(); // draw point in circle origin
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
    render.canvas.draw_points(&point_arr[..]).unwrap();
}

