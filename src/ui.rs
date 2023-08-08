/// impl UI classes

use crate::{ InputMode, Theme, WIDTH, HEIGHT };
use crate::render_text;
use sdl2::pixels::Color;
use sdl2::video::WindowContext;
use sdl2::render::{ WindowCanvas, TextureCreator };
use std::path::Path;

pub struct UserInterface<'a> {
    input_mode: InputMode,
    text_box: Vec<&'a str>,
    text_box_area: (i32, i32, u32, u32),
}

impl<'a> UserInterface<'a> {
    pub fn new() -> Self {
        Self {
            input_mode: InputMode::Emmiter,
            text_box: Vec::<&'a str>::new(),
            text_box_area: (0, HEIGHT as i32 - 200, WIDTH, 200),
        }
    }
    pub fn set_input_mode(&mut self, new_input_mode: InputMode) {
        self.input_mode = new_input_mode;
    }
    pub fn get_input_mode(&self) -> &InputMode {
        &self.input_mode
    }
    pub fn render(&self, canvas: &mut WindowCanvas) {

    }
    pub fn debug(&self, canvas: &mut WindowCanvas) {
        // display input node
        let alp = 120;
        match self.input_mode {
            InputMode::Emmiter  => canvas.set_draw_color(Color::RGB(120, 0, 0)),
            InputMode::Wire     => canvas.set_draw_color(Color::RGB(0, 120, 0)),
            InputMode::Receiver => canvas.set_draw_color(Color::RGB(120, 120, 0)),
            InputMode::Function => canvas.set_draw_color(Color::RGBA(0, 0, 120, alp)),
        }

        canvas.fill_rect(
            sdl2::rect::Rect::new(WIDTH as i32 - 32, 2, 30, 30)
            ).unwrap();
    }

    pub fn text_box_add(&mut self, text: &'a str) {
        self.text_box.push(text);
    }

    pub fn text_box_render(&self, canvas: &mut WindowCanvas,
                           texture_creator: &TextureCreator<WindowContext>,
                           theme: &Theme, font: &sdl2::ttf::Font) {
        canvas.set_draw_color(theme.text);

        let text_len = self.text_box_area.2 / self.text_box.len() as u32;

        let mut x_off_set = self.text_box_area.0;

        for text in self.text_box.iter() {
            render_text(canvas, texture_creator, font, text,
                        x_off_set, self.text_box_area.1,
                        text_len, self.text_box_area.3);
            x_off_set += text_len as i32;
        }
    }
}

pub struct Controller {
}


