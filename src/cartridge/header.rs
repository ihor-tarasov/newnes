use std::{fmt, io};

const HEADER_SIZE: usize = 16;

const MAGIC_NUMBER_START: usize = 0;
const MAGIC_NUMBER_END: usize = 3;
const PRG_ROM_UNITS_COUNT_OFFSET: usize = 4;
const CHR_ROM_UNITS_COUNT_OFFSET: usize = 5;
const PRG_RAM_UNITS_COUNT_OFFSET: usize = 8;

const FLAGS_6_OFFSET: usize = 6;
const FLAGS_7_OFFSET: usize = 7;
const FLAGS_9_OFFSET: usize = 9;
const FLAGS_10_OFFSET: usize = 10;

const MIRRORING_OFFSET: usize = FLAGS_6_OFFSET;
const BATTERY_OFFSET: usize = FLAGS_6_OFFSET;
const TRAINER_OFFSET: usize = FLAGS_6_OFFSET;
const FOUR_SCREEN_OFFSET: usize = FLAGS_6_OFFSET;
const MAPPER_LOWER_OFFSET: usize = FLAGS_6_OFFSET;
const MAPPER_UPPER_OFFSET: usize = FLAGS_7_OFFSET;
const CONSOLE_TYPE_OFFSET: usize = FLAGS_7_OFFSET;
const NES_2_0_OFFSET: usize = FLAGS_7_OFFSET;
const PRG_RAM_OFFSET: usize = FLAGS_10_OFFSET;
const BUS_CONFLICTS_OFFSET: usize = FLAGS_10_OFFSET;

const MAPPER_LOWER_BIT_OFFSET: u8 = 4;
const MAPPER_UPPER_BIT_OFFSET: u8 = 0;
const CONSOLE_TYPE_BIT_OFFSET: u8 = 0;
const NES_2_0_BIT_OFFSET: u8 = 2;
const TV_SYSTEM_BIT_OFFSET: u8 = 0;

const MIRRORING_BIT_MASK: u8 = 0b0000_0001;
const BATTERY_BIT_MASK: u8 = 0b0000_0010;
const TRAINER_BIT_MASK: u8 = 0b0000_0100;
const FOUR_SCREEN_BIT_MASK: u8 = 0b0000_1000;
const MAPPER_BIT_MASK: u8 = 0b1111_0000;
const CONSOLE_TYPE_BIT_MASK: u8 = 0b0000_0011;
const NES_2_0_BIT_MASK: u8 = 0b0000_1100;
const TV_SYSTEM_FLAG_9_BIT_MASK: u8 = 0b0000_0001;
const TV_SYSTEM_FLAG_10_BIT_MASK: u8 = 0b0000_0011;
const PRG_RAM_BIT_MASK: u8 = 0b0000_1000;
const BUS_CONFLICTS_BIT_MASK: u8 = 0b0001_0000;

const PRG_ROM_UNIT_SIZE: usize = 0x4000;
const CHR_ROM_UNIT_SIZE: usize = 0x2000;
const PRG_RAM_UNIT_SIZE: usize = 0x2000;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mirroring {
    /// Horizontal arrangement (CIRAM A10 = PPU A10)
    Vertical,
    /// Vertical arrangement (CIRAM A10 = PPU A11)
    Horizontal,
}

impl fmt::Display for Mirroring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mirroring::Vertical => write!(f, "Vertical"),
            Mirroring::Horizontal => write!(f, "Horizontal"),
        }
    }
}

impl Mirroring {
    fn new(bit: bool) -> Self {
        if bit {
            Self::Vertical
        } else {
            Self::Horizontal
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConsoleType {
    NintendoEntertainmentSystem,
    NintendoVsSystem,
    NintendoPlaychoise10,
    ExtendedConsoleType,
}

impl ConsoleType {
    fn new(bits: u8) -> Self {
        match bits {
            0 => ConsoleType::NintendoEntertainmentSystem,
            1 => ConsoleType::NintendoVsSystem,
            2 => ConsoleType::NintendoPlaychoise10,
            3 => ConsoleType::ExtendedConsoleType,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for ConsoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsoleType::NintendoEntertainmentSystem => {
                write!(f, "Nintendo Entertainment System/Family Computer")
            }
            ConsoleType::NintendoVsSystem => write!(f, "Nintendo Vs. System"),
            ConsoleType::NintendoPlaychoise10 => write!(f, "Nintendo Playchoice 10"),
            ConsoleType::ExtendedConsoleType => write!(f, "Extended Console Type"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TVSystem {
    NTSC,
    PAL,
    DualCompatible,
}

impl TVSystem {
    fn from_flag_9(bit: bool) -> Self {
        if bit {
            Self::PAL
        } else {
            Self::NTSC
        }
    }

    fn from_flag_10(bits: u8) -> Self {
        match bits {
            0 => TVSystem::NTSC,
            1 => TVSystem::DualCompatible,
            2 => TVSystem::PAL,
            3 => TVSystem::DualCompatible,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for TVSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TVSystem::NTSC => write!(f, "NTSC"),
            TVSystem::PAL => write!(f, "PAL"),
            TVSystem::DualCompatible => write!(f, "NTSC and PAL"),
        }
    }
}

pub struct Header([u8; HEADER_SIZE]);

impl Header {
    pub fn read<R>(read: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let mut header = [0; HEADER_SIZE];
        read.read_exact(&mut header)?;
        Ok(Self(header))
    }

    /// Constant $4E $45 $53 $1A (ASCII "NES" followed by MS-DOS end-of-file)
    pub fn magic_number(&self) -> &[u8] {
        &self.0[MAGIC_NUMBER_START..=MAGIC_NUMBER_END]
    }

    /// Size of PRG ROM in 16 KB units
    pub fn prg_rom_units_count(&self) -> u8 {
        self.0[PRG_ROM_UNITS_COUNT_OFFSET]
    }

    pub fn prg_rom_size(&self) -> usize {
        self.prg_rom_units_count() as usize * PRG_ROM_UNIT_SIZE
    }

    /// Size of CHR ROM in 8 KB units (value 0 means the board uses CHR RAM)
    pub fn chr_rom_units_count(&self) -> u8 {
        self.0[CHR_ROM_UNITS_COUNT_OFFSET]
    }

    pub fn chr_rom_size(&self) -> usize {
        self.chr_rom_units_count() as usize * CHR_ROM_UNIT_SIZE
    }

    fn bit(&self, offset: usize, mask: u8) -> bool {
        self.0[offset] & mask != 0
    }

    fn bits(&self, offset: usize, mask: u8, shift: u8) -> u8 {
        (self.0[offset] & mask) >> shift
    }

    pub fn mirroring(&self) -> Mirroring {
        Mirroring::new(self.bit(MIRRORING_OFFSET, MIRRORING_BIT_MASK))
    }

    /// Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory
    pub fn battery(&self) -> bool {
        self.bit(BATTERY_OFFSET, BATTERY_BIT_MASK)
    }

    /// 512-byte trainer at $7000-$71FF (stored before PRG data)
    pub fn trainer(&self) -> bool {
        self.bit(TRAINER_OFFSET, TRAINER_BIT_MASK)
    }

    /// Ignore mirroring control or above mirroring bit; instead provide four-screen VRAM
    pub fn four_screen(&self) -> bool {
        self.bit(FOUR_SCREEN_OFFSET, FOUR_SCREEN_BIT_MASK)
    }

    fn mapper_lower_nibble(&self) -> u8 {
        self.bits(
            MAPPER_LOWER_OFFSET,
            MAPPER_BIT_MASK,
            MAPPER_LOWER_BIT_OFFSET,
        )
    }

    fn mapper_upper_nibble(&self) -> u8 {
        self.bits(
            MAPPER_UPPER_OFFSET,
            MAPPER_BIT_MASK,
            MAPPER_UPPER_BIT_OFFSET,
        )
    }

    pub fn mapper(&self) -> u8 {
        self.mapper_lower_nibble() | self.mapper_upper_nibble()
    }

    pub fn console_type(&self) -> ConsoleType {
        ConsoleType::new(self.bits(
            CONSOLE_TYPE_OFFSET,
            CONSOLE_TYPE_BIT_MASK,
            CONSOLE_TYPE_BIT_OFFSET,
        ))
    }

    /// NES 2.0 is a more recent extension to the format that allows more flexibility
    /// in ROM and RAM size, among other things.
    pub fn nes_2_0(&self) -> bool {
        self.bits(NES_2_0_OFFSET, NES_2_0_BIT_MASK, NES_2_0_BIT_OFFSET) == 2
    }

    /// Size of PRG RAM in 8 KB units (Value 0 infers 8 KB for compatibility)
    pub fn prg_ram_units_count(&self) -> u8 {
        self.0[PRG_RAM_UNITS_COUNT_OFFSET]
    }

    pub fn prg_ram_size(&self) -> usize {
        self.prg_ram_units_count() as usize * PRG_RAM_UNIT_SIZE
    }

    pub fn tv_system_flag_9(&self) -> TVSystem {
        TVSystem::from_flag_9(self.bit(FLAGS_9_OFFSET, TV_SYSTEM_FLAG_9_BIT_MASK))
    }

    pub fn tv_system_flag_10(&self) -> TVSystem {
        TVSystem::from_flag_10(self.bits(
            FLAGS_10_OFFSET,
            TV_SYSTEM_FLAG_10_BIT_MASK,
            TV_SYSTEM_BIT_OFFSET,
        ))
    }

    /// PRG RAM ($6000-$7FFF)
    pub fn prg_ram(&self) -> bool {
        self.bit(PRG_RAM_OFFSET, PRG_RAM_BIT_MASK)
    }

    pub fn bus_conflicts(&self) -> bool {
        self.bit(BUS_CONFLICTS_OFFSET, BUS_CONFLICTS_BIT_MASK)
    }
}
