//! render text into a sdl2 canvas

use sdl2::rect::Point;
const CHARBOX_LEN: i32 = 64; // default 64
const CHARBOX_LEN_HALF: i32 = CHARBOX_LEN / 2;
const CHARBOX_LEN_QUARTER: i32 = CHARBOX_LEN / 4;

const TOP_OFFSET: i32 = 4;
const BOTTON_OFFSET: i32 = 4;

pub fn main() {
    extern crate sdl2;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;

    use std::time::Duration;
    use std::thread::sleep;

    let sdl_context = sdl2::init().unwrap();

    let video_sys = sdl_context.video().unwrap();

    let window = video_sys.window("text_test", 800, 600)
        .position_centered().opengl().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'run,
                _ => (),
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // draw text here
        canvas.set_draw_color(Color::RGB(127, 127, 0));

        A(&mut canvas, 100, 12);
        B(&mut canvas, 200, 12);
        C(&mut canvas, 300, 12);
        D(&mut canvas, 400, 12);
        E(&mut canvas, 500, 12);
        F(&mut canvas, 600, 12);
        G(&mut canvas, 100, 84);
        H(&mut canvas, 200, 84);
        I(&mut canvas, 300, 84);
        J(&mut canvas, 400, 84);
        K(&mut canvas, 500, 84);
        L(&mut canvas, 600, 84);
        M(&mut canvas, 100, 156);
        N(&mut canvas, 200, 156);
        O(&mut canvas, 300, 156);
        P(&mut canvas, 400, 156);
        Q(&mut canvas, 500, 156);
        R(&mut canvas, 600, 156);

        canvas.present();

        sleep(Duration::new(0, 1_000_000_000_u32 / 60));

    }
}

/// return points in arc an arc from p1(x1, y1) to p2x0, y0) of length r
/// positive values of r draw to right while negative draws to the left
fn arc(x1: i32, y1: i32,
       x0: i32, y0: i32, r: i32,
       tx_inc: i32, ty_inc: i32,
       quadrants: (bool, bool, bool, bool)) -> Vec<(i32, i32)> {
    // get center
    let center: (i32, i32) = ( ( (x1 + x0)/2).abs(), ( (y1 + y0)/2).abs() );
    let mut x = r.abs() - 1;
    let mut y = 0;

    let mut tx = 1;
    let mut ty = 1;

    let mut point_arr: Vec<(i32, i32)> = Vec::new();
    let mut err = tx - r * 2;
    while x >= y {
        if quadrants.0 == true {
            point_arr.push((center.0 + x, center.1 - y));
            point_arr.push((center.0 + y, center.1 - x));
        }

        if quadrants.1 == true {
            point_arr.push((center.0 + x, center.1 + y));
            point_arr.push((center.0 + y, center.1 + x));

        }
        if quadrants.2 == true {
            point_arr.push((center.0 - x, center.1 + y));
            point_arr.push((center.0 - y, center.1 + x));
        }
        if quadrants.3 == true {
            point_arr.push((center.0 - x, center.1 - y));
            point_arr.push((center.0 - y, center.1 - x));
        }

        if err <= 0 {
            y += 1;
            err += ty;
            ty += ty_inc; // make arc pointy default is += 2
        } else {
            x -= 1;
            tx += tx_inc; // default in +=2
            err += tx - r * 2;
        }
    }
    point_arr
}

/// draw A in canvas at x, y
#[allow(non_snake_case, dead_code)]
pub fn A(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = 12;

    // left line
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_HALF, y + TOP_OFFSET),
        Point::new(x + off_set, y + CHARBOX_LEN - BOTTON_OFFSET),
                    ).unwrap();
    // right line
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_HALF, y + TOP_OFFSET),
        Point::new(x + CHARBOX_LEN - off_set, y + CHARBOX_LEN - BOTTON_OFFSET)
        ).unwrap();
    // 
    canvas.draw_line(
        Point::new(x + off_set * 2, y + 4 + CHARBOX_LEN_HALF),
        Point::new(x + CHARBOX_LEN - off_set * 2, y + 4 + CHARBOX_LEN_HALF),
        ).unwrap();
}

#[allow(non_snake_case, dead_code)]
pub fn B(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = 24;
    let top = Point::new(x + off_set, y + TOP_OFFSET - 2);
    let bot = Point::new(x + off_set, y + CHARBOX_LEN - BOTTON_OFFSET + 2);
    canvas.draw_line(top, bot).unwrap();
    let point_arr = arc(x + off_set, y + TOP_OFFSET - 2,
                        x + off_set, y + CHARBOX_LEN_HALF,
                        CHARBOX_LEN_QUARTER, 3, 2,
                        (true, true, false, false));
    for pos in point_arr.into_iter() {
        canvas.draw_point(Point::new(pos.0, pos.1)).unwrap();
    }
    let point_arr = arc(x + off_set, y + CHARBOX_LEN_HALF,
                        x + off_set, y + CHARBOX_LEN - BOTTON_OFFSET + 2,
                        CHARBOX_LEN_QUARTER, 3, 2,
                        (true, true, false, false));
    for pos in point_arr.into_iter() {
        canvas.draw_point(Point::new(pos.0, pos.1)).unwrap();
    }
}

#[allow(non_snake_case, dead_code)]
pub fn C(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = CHARBOX_LEN_HALF + 12;
    let point_arr = arc(x + off_set, y + TOP_OFFSET,
                        x + off_set, y + CHARBOX_LEN - BOTTON_OFFSET,
                        CHARBOX_LEN_HALF - 4, 3, 2,
                        (false, false, true, true));

    for pos in point_arr.into_iter() {
        canvas.draw_point(Point::new(pos.0, pos.1)).unwrap();
    }
}
#[allow(non_snake_case, dead_code)]
pub fn D(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = CHARBOX_LEN_QUARTER;
    let top = Point::new(x + off_set, y + TOP_OFFSET);
    let bot = Point::new(x + off_set, y + CHARBOX_LEN - BOTTON_OFFSET);
    canvas.draw_line(top, bot).unwrap();
    let point_arr = arc(x + off_set, y + TOP_OFFSET,
                        x + off_set, y + CHARBOX_LEN - BOTTON_OFFSET,
                        CHARBOX_LEN_HALF - 2, 2, 2,
                        (true, true, false, false));

    for pos in point_arr.into_iter() {
        canvas.draw_point(Point::new(pos.0, pos.1)).unwrap();
    }
}
#[allow(non_snake_case, dead_code)]
pub fn E(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = CHARBOX_LEN_QUARTER;
    let line_width = CHARBOX_LEN_HALF;

    canvas.draw_line(
        Point::new(x + off_set, y + TOP_OFFSET),
        Point::new(x + off_set, y + CHARBOX_LEN - BOTTON_OFFSET),
        ).unwrap();
    // top line
    canvas.draw_line(
        Point::new(x + off_set, y + TOP_OFFSET),
        Point::new(x + off_set + line_width, y + TOP_OFFSET),
        ).unwrap();
    // middle line
    canvas.draw_line(
        Point::new(x + off_set, y + CHARBOX_LEN_HALF),
        Point::new(x + off_set + line_width, y + CHARBOX_LEN_HALF),
        ).unwrap();
    // botton line
    canvas.draw_line(
        Point::new(x + off_set, y + CHARBOX_LEN - BOTTON_OFFSET),
        Point::new(x + off_set + line_width, y + CHARBOX_LEN - BOTTON_OFFSET),
        ).unwrap();
}
#[allow(non_snake_case, dead_code)]
pub fn F(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();

    let off_set = CHARBOX_LEN_QUARTER;
    let line_width = CHARBOX_LEN_HALF;

    // vertical line
    canvas.draw_line(
        Point::new(x + off_set, y + TOP_OFFSET),
        Point::new(x + off_set, y + CHARBOX_LEN - BOTTON_OFFSET),
        ).unwrap();
    // top line
    canvas.draw_line(
        Point::new(x + off_set, y + TOP_OFFSET),
        Point::new(x + off_set + line_width, y + TOP_OFFSET),
        ).unwrap();
    // middle line
    canvas.draw_line(
        Point::new(x + off_set, y + CHARBOX_LEN_HALF),
        Point::new(x + off_set + line_width, y + CHARBOX_LEN_HALF),
        ).unwrap();
}
#[allow(non_snake_case, dead_code)]
pub fn G(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let point_arr = arc(x + CHARBOX_LEN_HALF , y + TOP_OFFSET,
                        x + CHARBOX_LEN_HALF , y + CHARBOX_LEN - BOTTON_OFFSET,
                        CHARBOX_LEN_HALF - 4, 3, 2,
                        (false, false, true, true));
    for pos in point_arr.into_iter() {
        canvas.draw_point(Point::new(pos.0, pos.1)).unwrap();
    }
    let point_arr = arc(x + CHARBOX_LEN_HALF - 12 , y + CHARBOX_LEN - 12, 
                        x + CHARBOX_LEN_HALF , y + CHARBOX_LEN_QUARTER - BOTTON_OFFSET,
                        CHARBOX_LEN_HALF - 4, 3, 2,
                        (false, true, false, false));

    for pos in point_arr.into_iter() {
        canvas.draw_point(Point::new(pos.0, pos.1)).unwrap();
    }

    // middle line
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN - 12, y + CHARBOX_LEN_HALF),
        Point::new(x + CHARBOX_LEN_HALF, y + CHARBOX_LEN_HALF),
        ).unwrap();
    // top line

    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_HALF, y + TOP_OFFSET),
        Point::new(x + 3 * CHARBOX_LEN_QUARTER, y + TOP_OFFSET + 4),
        ).unwrap();

}

#[allow(non_snake_case, dead_code)]
pub fn H(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    // left vertical line
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_QUARTER, y + TOP_OFFSET),
        Point::new(x + CHARBOX_LEN_QUARTER, y + CHARBOX_LEN - BOTTON_OFFSET),
        ).unwrap();
    // right vertical line
    canvas.draw_line(
        Point::new(x + 3 * CHARBOX_LEN_QUARTER, y + TOP_OFFSET),
        Point::new(x + 3 * CHARBOX_LEN_QUARTER, y + CHARBOX_LEN - BOTTON_OFFSET),
        ).unwrap();
    // middle line
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_QUARTER, y + CHARBOX_LEN_HALF),
        Point::new(x + 3 * CHARBOX_LEN_QUARTER, y + CHARBOX_LEN_HALF),
        ).unwrap();
}

#[allow(non_snake_case, dead_code)]
pub fn I(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();

    // middle line
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_HALF, y + TOP_OFFSET),
        Point::new(x + CHARBOX_LEN_HALF, y + CHARBOX_LEN - BOTTON_OFFSET),
        ).unwrap();
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_QUARTER, y + TOP_OFFSET),
        Point::new(x + 3 * CHARBOX_LEN_QUARTER, y + TOP_OFFSET),
        ).unwrap();
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_QUARTER, y + CHARBOX_LEN - BOTTON_OFFSET),
        Point::new(x + 3 * CHARBOX_LEN_QUARTER, y + CHARBOX_LEN - BOTTON_OFFSET),
        ).unwrap();
}


#[allow(non_snake_case, dead_code)]
pub fn J(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_HALF, y + TOP_OFFSET * 2),
        Point::new(x + CHARBOX_LEN_HALF, y + 3 * CHARBOX_LEN_QUARTER + BOTTON_OFFSET),
        ).unwrap();
    let point_arr = arc(x + CHARBOX_LEN_HALF, y, 
                        x + CHARBOX_LEN_HALF, y + CHARBOX_LEN, 
                        CHARBOX_LEN_HALF - 12, 1, 1,
                        (false, false, true, false));

    for pos in point_arr.into_iter() {
        canvas.draw_point(Point::new(pos.0, pos.1)).unwrap();
    }
}

#[allow(non_snake_case, dead_code)]
pub fn K(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_QUARTER, y + TOP_OFFSET),
        Point::new(x + CHARBOX_LEN_QUARTER, y + CHARBOX_LEN - BOTTON_OFFSET),
        ).unwrap();
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_QUARTER + 2, y + CHARBOX_LEN_HALF),
        Point::new(x + CHARBOX_LEN_HALF + 12, y + TOP_OFFSET),
        ).unwrap();
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_QUARTER + 2, y + CHARBOX_LEN_HALF),
        Point::new(x + CHARBOX_LEN_HALF + 12, y + CHARBOX_LEN - BOTTON_OFFSET),
        ).unwrap();

}

#[allow(non_snake_case, dead_code)]
pub fn L(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_QUARTER, y + TOP_OFFSET),
        Point::new(x + CHARBOX_LEN_QUARTER, y + CHARBOX_LEN - BOTTON_OFFSET),
        ).unwrap();
    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_QUARTER, y + CHARBOX_LEN - BOTTON_OFFSET),
        Point::new(x + CHARBOX_LEN_HALF + 12, y + CHARBOX_LEN - BOTTON_OFFSET),
        ).unwrap();
}

#[allow(non_snake_case, dead_code)]
pub fn M(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();

}

#[allow(non_snake_case, dead_code)]
pub fn N(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();

}

#[allow(non_snake_case, dead_code)]
pub fn O(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = CHARBOX_LEN_HALF;
    let point_arr = arc(x + off_set, y + TOP_OFFSET,
                        x + off_set, y + CHARBOX_LEN - BOTTON_OFFSET,
                        CHARBOX_LEN_HALF - 8, 3, 2,
                        (true, true, true, true));

    for pos in point_arr.into_iter() {
        canvas.draw_point(Point::new(pos.0, pos.1)).unwrap();
    }

}

#[allow(non_snake_case, dead_code)]
pub fn P(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();

}

#[allow(non_snake_case, dead_code)]
pub fn Q(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = CHARBOX_LEN_HALF;
    let point_arr = arc(x + off_set, y + TOP_OFFSET,
                        x + off_set, y + CHARBOX_LEN - BOTTON_OFFSET,
                        CHARBOX_LEN_HALF - 8, 3, 2,
                        (true, true, true, true));

    for pos in point_arr.into_iter() {
        canvas.draw_point(Point::new(pos.0, pos.1)).unwrap();
    }

    canvas.draw_line(
        Point::new(x + CHARBOX_LEN_QUARTER * 3 - 8, y + CHARBOX_LEN_QUARTER * 3 - 8),
        Point::new(x + CHARBOX_LEN - 8, y + CHARBOX_LEN - 8),
        ).unwrap();

}

#[allow(non_snake_case, dead_code)]
pub fn R(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();

}

#[allow(non_snake_case, dead_code)]
pub fn S(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = 14;

}

#[allow(non_snake_case, dead_code)]
pub fn T(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = 14;

}

#[allow(non_snake_case, dead_code)]
pub fn U(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = 14;

}

#[allow(non_snake_case, dead_code)]
pub fn V(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = 14;

}

#[allow(non_snake_case, dead_code)]
pub fn X(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = 14;

}

#[allow(non_snake_case, dead_code)]
pub fn Y(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = 14;

}

#[allow(non_snake_case, dead_code)]
pub fn W(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = 14;

}

#[allow(non_snake_case, dead_code)]
pub fn Z(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    canvas.draw_rect(sdl2::rect::Rect::new(x, y, CHARBOX_LEN as u32, CHARBOX_LEN as u32)).unwrap();
    let off_set = 14;

}
