use std::path::Path;

use log::{error, info};

use crate::{args::Args, cartridge, display::Display};

pub struct Emulator {
    display: Display,
}

impl Emulator {
    fn create_display() -> Display {
        info!("Initializing display.");
        Display::new().unwrap_or_else(|error| {
            error!("Unable to create display: \"{error}\"");
            std::process::exit(1);
        })
    }

    fn get_rom_name_from_path(path: &str) -> String {
        Path::new(path)
            .file_stem()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    pub fn new(args: Args) -> Self {
        cartridge::load_rom(&args.path).unwrap_or_else(|error| {
            error!("Unable to load ROM \"{}\", \"{error}", args.path);
            std::process::exit(2);
        });
        let mut display = Self::create_display();
        display.set_rom_name(Self::get_rom_name_from_path(&args.path));
        Self { display }
    }

    fn update_display(&mut self) {
        self.display.update().unwrap_or_else(|error| {
            error!("Unable to update window, error: \"{error}\"");
            std::process::exit(2);
        })
    }

    pub fn run(&mut self) {
        for (index, pixel) in self.display.buffer_mut().iter_mut().enumerate() {
            *pixel = index as u32 % 673433;
        }

        while self.display.is_open() {
            self.update_display();
        }
    }
}
