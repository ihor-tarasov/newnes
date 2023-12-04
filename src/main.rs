use clap::Parser;
use newnes::{args::Args, emulator::Emulator, logger};

fn main() {
    logger::init();
    let args = Args::parse();
    Emulator::new(args).run();
}
