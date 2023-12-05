use std::{fs::File, io};

use log::{error, info, warn};

use self::header::Header;

pub mod header;

pub struct Cartridge;

fn log_header_info(header: &Header) {
    if header.magic_number() != b"NES\x1A" {
        error!("Incorrect \"NES\\x1A\" magic number.");
    }
    info!(
        "Size of PRG ROM is {} units ({} KB)",
        header.prg_rom_units_count(),
        header.prg_rom_size() / 0x400
    );
    info!(
        "Size of CHR ROM is {} units ({} KB)",
        header.chr_rom_units_count(),
        header.chr_rom_size() / 0x400
    );
    info!("Mirroring: {}", header.mirroring());
    info!("Battery: {}", if header.battery() { "Yes" } else { "No" });
    info!("Trainer: {}", if header.trainer() { "Yes" } else { "No" });
    info!(
        "Four-screen: {}",
        if header.four_screen() { "Yes" } else { "No" }
    );
    info!("Mapper: 0x{:02X}", header.mapper());
    info!("Console type: {}", header.console_type());
    if header.nes_2_0() {
        error!("NES 2.0 cartridge formats are not supported");
    }
    info!(
        "Size of PRG RAM is {} units ({} KB)",
        header.prg_ram_units_count(),
        header.prg_ram_size() / 0x400
    );
    if header.tv_system_flag_9() == header.tv_system_flag_10() {
        info!("TV system: {}", header.tv_system_flag_10());
    } else {
        warn!(
            "TV system in flags 9 and 10 are different: flag 9 - {}, flag 10 - {}",
            header.tv_system_flag_9(),
            header.tv_system_flag_10()
        );
    }
    info!("PRG RAM: {}", if header.prg_ram() { "Yes" } else { "No" });
    info!(
        "Bus conflicts: {}",
        if header.bus_conflicts() { "Yes" } else { "No" }
    );
}

pub fn load_rom(path: &str) -> io::Result<()> {
    info!("Loading ROM \"{path}\"");
    let mut file = File::open(path)?;
    let header = Header::read(&mut file)?;
    log_header_info(&header);
    Ok(())
}
