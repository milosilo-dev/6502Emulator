use std::{cell::RefCell, rc::Rc};

use minifb::{Window, WindowOptions};

use crate::platform::keyboard::Keyboard;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub struct Fb{
    buffer: Vec<u32>,
    window: Window,
    keyboard: Rc<RefCell<Keyboard>>,
}

impl Fb{
    pub fn default(keyboard: Rc<RefCell<Keyboard>>) -> Self{
        let window = Window::new(
            "Test - ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        Self { 
            buffer: vec![0; WIDTH * HEIGHT],
            window,
            keyboard
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, colour: u32) {
        if x < WIDTH && y < HEIGHT {
            self.buffer[y * WIDTH + x] = colour;
        }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();

        self.keyboard.borrow_mut().update_keys(&self.window);
    }
}