pub fn get_cart_type(cartridge_type: u8) -> String {
    match cartridge_type {
        0x00 => "ROM Only".to_string(),
        0x01 => "MBC1".to_string(),
        0x02 => "MBC1+RAM".to_string(),
        0x03 => "MBC1+RAM+BATTERY".to_string(),
        0x05 => "MBC2".to_string(),
        0x06 => "MBC2+BATTERY".to_string(),
        0x08 => "ROM+RAM".to_string(),
        0x09 => "ROM+RAM+BATTERY".to_string(),
        0x0B => "MMM01".to_string(),
        0x0C => "MMM01+RAM".to_string(),
        0x0D => "MMM01+RAM+BATTERY".to_string(),
        0x0F => "MBC3+TIMER+BATTERY".to_string(),
        0x10 => "MBC3+TIMER+RAM+BATTERY".to_string(),
        0x11 => "MBC3".to_string(),
        0x12 => "MBC3+RAM".to_string(),
        0x13 => "MBC3+RAM+BATTERY".to_string(),
        0x19 => "MBC5".to_string(),
        0x1A => "MBC5+RAM".to_string(),
        0x1B => "MBC5+RAM+BATTERY".to_string(),
        0x1C => "MBC5+RUMBLE".to_string(),
        0x1D => "MBC5+RUMBLE+RAM".to_string(),
        0x1E => "MBC5+RUMBLE+RAM+BATTERY".to_string(),
        0x20 => "MBC6".to_string(),
        0x22 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY".to_string(),
        0xFC => "POCKET CAMERA".to_string(),
        0xFD => "BandAI TAMA5".to_string(),
        0xFE => "HuC-3".to_string(),
        0xFF => "HuC-1+RAM+BATTERY".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn get_rom_size(rom_size: u8) -> String {
    match rom_size {
        0x00 => "32KiB - 2 Banks".to_string(),
        0x01 => "64KiB - 4 Banks".to_string(),
        0x02 => "128KiB - 8 Banks".to_string(),
        0x03 => "256KiB - 16 Banks".to_string(),
        0x04 => "512KiB - 32 Banks".to_string(),
        0x05 => "1MiB - 64 Banks".to_string(),
        0x06 => "2MiB - 128 Banks".to_string(),
        0x07 => "4MiB - 256 Banks".to_string(),
        0x08 => "8MiB - 512 Banks".to_string(),
        0x52 => "1.1MiB - 72 Banks".to_string(),
        0x53 => "1.2MiB - 80 Banks".to_string(),
        0x54 => "1.5MiB - 96 Banks".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn get_num_rom_banks(rom_size: u8) -> u16 {
    match rom_size {
        0x00 => 2,
        0x01 => 4,
        0x02 => 8,
        0x03 => 16,
        0x04 => 32,
        0x05 => 64,
        0x06 => 128,
        0x07 => 256,
        0x08 => 512,
        0x52 => 72,
        0x53 => 80,
        0x54 => 96,
        _ => 0,
    }
}

pub fn get_ram_size(ram_size: u8) -> String {
    match ram_size {
        0x00 => "None".to_string(),
        0x01 => "2KiB (Unused)".to_string(),
        0x02 => "8KiB".to_string(),
        0x03 => "32KiB".to_string(),
        0x04 => "128KiB".to_string(),
        0x05 => "64KiB".to_string(),
        _ => "Unknown".to_string(),
    }
}
