use minifb::Window;

pub struct Keyboard{}

impl Keyboard {
    pub fn default() -> Self {
        Self {  }
    }

    pub fn update_keys(&mut self, window: &Window) {
        let _ = window.get_keys();
    }
}