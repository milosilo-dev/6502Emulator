use std::{cell::RefCell, rc::Rc};

use minifb::{Key, Scale, Window, WindowOptions};

use crate::platform::{keyboard::Keyboard, text::Text};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub struct Fb{
    buffer: Vec<u32>,
    window: Window,
    keyboard: Rc<RefCell<Keyboard>>,
    font: Text,
}

impl Fb{
    pub fn default(keyboard: Rc<RefCell<Keyboard>>) -> Self{
        let mut window = Window::new(
            "Test - ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let buffer = vec![0; WIDTH * HEIGHT];

        window.set_target_fps(60);
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        let font = Text::new(WIDTH, HEIGHT, 1);

        Self { 
            buffer,
            window,
            keyboard,
            font
        }
    }

    pub fn draw_text(&mut self, x: usize, y: usize, content: &str) {
        self.font.draw(&mut self.buffer, (x, y), content);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, colour: u32) {
        if x < WIDTH && y < HEIGHT {
            self.buffer[y * WIDTH + x] = colour;
        }
    }

    pub fn update(&mut self) -> bool {
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();

        self.keyboard.borrow_mut().update_keys(&self.window);

        self.window.is_active() && !self.window.is_key_down(Key::Escape)
    }
}