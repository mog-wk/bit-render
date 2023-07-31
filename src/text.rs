/// render text into a sdl2 canvas


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

        canvas.present();

        sleep(Duration::new(0, 1_000_000_000_u32 / 60));

    }
}

