use crate::{constants, licensee};

pub fn extract_metadata(data: &Vec<u8>, debug: bool) -> CartridgeMetadata {
    // Game Title
    let game_title = std::str::from_utf8(&data[0x134..0x143]).unwrap();
    if debug {
        println!(" >> Game Title: {} <<", game_title);
    }

    // Maker code
    let maker_code = &data[0x13F..0x142];
    if debug {
        println!("Maker Code: {:?}", maker_code);
    }

    // CGB flag
    let cgb_flag = data[0x143];
    if debug {
        println!(
            "CGB Flag: {}, Support: {}",
            cgb_flag,
            if cgb_flag == 0x80 { "Yes" } else { "No" }
        );
    }
    // new licensee code
    let new_licensee_code = std::str::from_utf8(&data[0x144..0x145]).unwrap();
    if debug {
        println!(
            "New Licensee Code: {}",
            licensee::resolve_new_licensee_code(new_licensee_code)
        );
    }

    // SGB flag
    let sgb_flag = data[0x146];
    if debug {
        println!(
            "SGB Flag: {}, Support: {}",
            sgb_flag,
            if sgb_flag == 0x03 { "Yes" } else { "No" }
        );
    }
    // Cartridge type
    let cartridge_type = data[0x147];
    if debug {
        println!(
            "Cartridge Type: {:#?}; {}",
            cartridge_type,
            constants::get_cart_type(cartridge_type)
        );
    }

    // ROM Size
    let rom_size = data[0x148];
    if debug {
        println!("ROM Size: {}", constants::get_rom_size(rom_size));
    }

    // RAM Size
    let ram_size = data[0x149];
    if debug {
        println!("RAM Size: {}", constants::get_ram_size(ram_size));
    }

    // Region
    let destination_code = data[0x14A];
    if debug {
        println!(
            "Destination: {}",
            if destination_code == 0x00 {
                "Japan"
            } else {
                "Overseas"
            }
        );
    }

    // Licensee code
    let licensee_code = data[0x14B];
    if debug {
        println!(
            "Licensee Code: {}",
            licensee::resolve_old_licensee_code(licensee_code)
        );
    }

    // game version
    let mask_rom_version = data[0x14C];
    if debug {
        println!("Mask ROM Version: {:#X}", mask_rom_version);
    }
    if debug {
        println!(" >> Header validation:");
    }
    // header checksum
    let header_checksum = data[0x14D];
    if debug {
        println!("Header Checksum: {:#X}", header_checksum);
    }

    let mut calc_checksum: u8 = 0x00;
    for addr in 0x134..0x14D {
        calc_checksum = calc_checksum.wrapping_sub(data[addr] + 1);
    }
    if debug {
        println!("Calculated Checksum: {:#X}", calc_checksum);
        println!(
            " >> Verification {}",
            if calc_checksum == header_checksum {
                "Passed"
            } else {
                "Failed"
            }
        );
    }

    return CartridgeMetadata {
        game_title: (game_title.to_string()),
        rom_size: (rom_size),
        ram_size: (ram_size),
        cartridge_type: (cartridge_type),
    };
}

pub struct CartridgeMetadata {
    game_title: String,
    rom_size: u8,
    ram_size: u8,
    cartridge_type: u8,
}
impl CartridgeMetadata {
    pub fn game_title(&self) -> &str {
        &self.game_title
    }

    pub fn rom_size(&self) -> u8 {
        self.rom_size
    }

    pub fn ram_size(&self) -> u8 {
        self.ram_size
    }

    pub fn cartridge_type(&self) -> u8 {
        self.cartridge_type
    }
}
