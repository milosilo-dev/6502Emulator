use minifb::{Key, Window};

#[derive(Copy, Clone)]
pub struct PlatformKey {
    pub row: usize,
    pub bit: u8,
}

pub fn map_key(key: Key) -> Option<PlatformKey> {
    match key {
        // ===== Letters =====
        Key::A => Some(PlatformKey { row: 1, bit: 4 }),
        Key::B => Some(PlatformKey { row: 4, bit: 6 }),
        Key::C => Some(PlatformKey { row: 2, bit: 6 }),
        Key::D => Some(PlatformKey { row: 2, bit: 4 }),
        Key::E => Some(PlatformKey { row: 0, bit: 2 }),
        Key::F => Some(PlatformKey { row: 3, bit: 4 }),
        Key::G => Some(PlatformKey { row: 4, bit: 4 }),
        Key::H => Some(PlatformKey { row: 5, bit: 4 }),
        Key::I => Some(PlatformKey { row: 5, bit: 2 }),
        Key::J => Some(PlatformKey { row: 6, bit: 4 }),
        Key::K => Some(PlatformKey { row: 7, bit: 4 }),
        Key::L => Some(PlatformKey { row: 7, bit: 2 }),
        Key::M => Some(PlatformKey { row: 6, bit: 6 }),
        Key::N => Some(PlatformKey { row: 5, bit: 6 }),
        Key::O => Some(PlatformKey { row: 6, bit: 2 }),
        Key::P => Some(PlatformKey { row: 7, bit: 0 }),
        Key::Q => Some(PlatformKey { row: 0, bit: 0 }),
        Key::R => Some(PlatformKey { row: 1, bit: 2 }),
        Key::S => Some(PlatformKey { row: 1, bit: 6 }),
        Key::T => Some(PlatformKey { row: 2, bit: 2 }),
        Key::U => Some(PlatformKey { row: 6, bit: 0 }),
        Key::V => Some(PlatformKey { row: 3, bit: 6 }),
        Key::W => Some(PlatformKey { row: 0, bit: 4 }),
        Key::X => Some(PlatformKey { row: 2, bit: 6 }),
        Key::Y => Some(PlatformKey { row: 4, bit: 2 }),
        Key::Z => Some(PlatformKey { row: 1, bit: 6 }),

        // ===== Numbers =====
        Key::Key0 => Some(PlatformKey { row: 7, bit: 1 }),
        Key::Key1 => Some(PlatformKey { row: 0, bit: 1 }),
        Key::Key2 => Some(PlatformKey { row: 0, bit: 3 }),
        Key::Key3 => Some(PlatformKey { row: 1, bit: 1 }),
        Key::Key4 => Some(PlatformKey { row: 1, bit: 3 }),
        Key::Key5 => Some(PlatformKey { row: 2, bit: 1 }),
        Key::Key6 => Some(PlatformKey { row: 2, bit: 3 }),
        Key::Key7 => Some(PlatformKey { row: 3, bit: 1 }),
        Key::Key8 => Some(PlatformKey { row: 3, bit: 3 }),
        Key::Key9 => Some(PlatformKey { row: 4, bit: 1 }),

        // ===== Special =====
        Key::Space => Some(PlatformKey { row: 6, bit: 7 }),
        Key::Enter => Some(PlatformKey { row: 7, bit: 7 }),

        Key::LeftShift | Key::RightShift =>
            Some(PlatformKey { row: 0, bit: 7 }),

        Key::LeftCtrl | Key::RightCtrl =>
            Some(PlatformKey { row: 1, bit: 7 }),

        // ===== Arrows =====
        Key::Left  => Some(PlatformKey { row: 3, bit: 0 }),
        Key::Right => Some(PlatformKey { row: 4, bit: 0 }),
        Key::Up    => Some(PlatformKey { row: 5, bit: 0 }),
        Key::Down  => Some(PlatformKey { row: 6, bit: 1 }),

        _ => None,
    }
}

pub struct Keyboard{
    rows: [u8; 8],
}

impl Keyboard {
    pub fn default() -> Self {
        Self {
            rows: [0b11111111; 8]
        }
    }

    pub fn update_keys(&mut self, window: &Window) {
        self.rows = [0b11111111; 8]; // Reset pressed keys
        let cur_keys = window.get_keys();
        for key in cur_keys{
            let platform_key = map_key(key);
            if platform_key.is_some(){

                let platform_key = platform_key.unwrap();
                let row = platform_key.row;
                let bit = platform_key.bit;
                self.rows[row] |= 1 << bit;
            }
        }
    }

    pub fn get_row(&self, row: u8) -> Option<u8> {
        if row < self.rows.len() as u8 - 1{
            return Some(self.rows[row as usize]);
        }
        None
    }

    pub fn get_key(&self, row: u8, col: u8) -> bool{
        println!("Got key: row: {row}, col: {col}");
        if row > self.rows.len() as u8 - 1{
            println!("Key press");
            return (self.rows[row as usize] >> col) & 1 != 0;
        }
        false
    }
}