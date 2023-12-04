use std::{error::Error, time::Duration};

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;
pub const NAME: &str = "newnes";

pub const WINDOW_OPTIONS: WindowOptions = WindowOptions {
    scale: Scale::X2,
    scale_mode: ScaleMode::AspectRatioStretch,
    resize: true,
    borderless: false,
    title: true,
    topmost: false,
    transparency: false,
    none: false,
};

pub struct Display {
    rom_name: String,
    window: Window,
}

impl Display {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut window = Window::new(NAME, WIDTH, HEIGHT, WINDOW_OPTIONS)?;
        window.limit_update_rate(Some(Duration::from_micros(16666)));
        Ok(Self {
            rom_name: String::new(),
            window,
        })
    }

    pub fn set_rom_name(&mut self, rom_name: String) {
        self.window.set_title(format!("{NAME} | {rom_name}").as_str());
        self.rom_name = rom_name;
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn update(&mut self, buffer: &[u32]) -> Result<(), Box<dyn Error>> {
        self.window.update_with_buffer(buffer, WIDTH, HEIGHT)?;
        Ok(())
    }
}
