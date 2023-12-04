use std::path::Path;

use newnes::{args::Args, display::Display};
use clap::Parser;
use log::{error, info};

fn create_display() -> Display {
    info!("Initializing display.");
    Display::new().unwrap_or_else(|error| {
        error!("Unable to create display: \"{error}\"");
        std::process::exit(1);
    })
}

fn update_display(display: &mut Display, buffer: &[u32]) {
    display.update(buffer).unwrap_or_else(|error| {
        error!("Unable to update window, error: \"{error}\"");
        std::process::exit(2);
    })
}

fn init_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init()
}

fn get_rom_name_from_path(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_default()
}

fn main() {
    init_logger();
    let args = Args::parse();

    let mut display = create_display();
    display.set_rom_name(get_rom_name_from_path(&args.path));

    let mut buffer = vec![0u32; 256 * 240];

    for (index, pixel) in buffer.iter_mut().enumerate() {
        *pixel = index as u32 % 673433;
    }

    while display.is_open() {
        update_display(&mut display, &buffer);
    }
}
