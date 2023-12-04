use clap::Parser;

/// NES emulator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the ROM file
    #[arg(short, long)]
    pub path: String,
}
