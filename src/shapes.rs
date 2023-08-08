use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[allow(dead_code)]
pub const PI: f64 = 3.1415926;
#[allow(dead_code)]
pub const HPI: f64 = PI / 2.0;
#[allow(dead_code)]
pub const DPI: f64 = PI * 2.0;



#[allow(dead_code)]
/// draw a rectangle with the top-left corner at x0 and y0 in canvas
pub fn draw_rect(canvas: &mut Canvas<Window>, x0: i32, y0: i32, width: i32, height: i32) {
    for i in 0..=width {
        for j in 0..=height {
            canvas.draw_point(Point::new(x0 + i, y0 + j)).unwrap();
        }
    }
}


/// draw circle in canvas with current sdl color
pub fn draw_circle(canvas: &mut Canvas<Window>, x0: i32, y0: i32, r: i32) {
    let d: i32 = r * 2;
    for i in 0..d {
        for j in 0..d {
            let dx = r - i;
            let dy = r - j;
            if (dx * dx + dy * dy) <= (r * r) {
                canvas.draw_point(Point::new(x0 + dx, y0 + dy)).expect("failed to draw point in canvas");
            }
        }
    }
}


#[allow(dead_code)]
/// draws an unfilled circle
pub fn draw_circunference(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, radius: i32) {
    // Mid Point circle algorithm
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
    canvas.draw_points(&point_arr[..]).unwrap();
}


#[allow(dead_code)]
/// draws a filled ellipse; has pixel overlap; use conservatly
pub fn draw_full_ellipse(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, radius: i32) {
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

    canvas.draw_point(Point::new(center_x, center_y)).unwrap(); // draw point in circle origin
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
    canvas.draw_points(&point_arr[..]).unwrap();
}

