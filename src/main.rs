use std::fs;
use std::io;
use std::io::Read;

mod constants;
mod licensee;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = String::from("./roms/pred.gb");
    if !fs::exists(&filename)? {
        println!("File not found");
        std::process::exit(1);
    }
    let data: Vec<u8> = fs::read(&filename)?;

    // Game Title
    let game_title = std::str::from_utf8(&data[0x134..0x143])?;
    println!(" >> Game Title: {} <<", game_title);

    // Maker code
    let maker_code = &data[0x13F..0x142];
    println!("Maker Code: {:?}", maker_code);

    // CGB flag
    let cgb_flag = data[0x143];
    println!(
        "CGB Flag: {}, Support: {}",
        cgb_flag,
        if cgb_flag == 0x80 { "Yes" } else { "No" }
    );
    // new licensee code
    let new_licensee_code = std::str::from_utf8(&data[0x144..0x145])?;
    println!(
        "New Licensee Code: {}",
        licensee::resolve_new_licensee_code(new_licensee_code)
    );

    // SGB flag
    let sgb_flag = data[0x146];
    println!(
        "SGB Flag: {}, Support: {}",
        sgb_flag,
        if sgb_flag == 0x03 { "Yes" } else { "No" }
    );
    // Cartridge type
    let cartridge_type = data[0x147];
    println!(
        "Cartridge Type: {:#?}; {}",
        cartridge_type,
        constants::get_cart_type(cartridge_type)
    );

    // ROM Size
    let rom_size = data[0x148];
    println!("ROM Size: {}", constants::get_rom_size(rom_size));

    // RAM Size
    let ram_size = data[0x149];
    println!("RAM Size: {}", constants::get_ram_size(ram_size));

    // Region
    let destination_code = data[0x14A];
    println!(
        "Destination: {}",
        if destination_code == 0x00 {
            "Japan"
        } else {
            "Overseas"
        }
    );

    // Licensee code
    let licensee_code = data[0x14B];
    println!(
        "Licensee Code: {}",
        licensee::resolve_old_licensee_code(licensee_code)
    );

    // game version
    let mask_rom_version = data[0x14C];
    println!("Mask ROM Version: {:#X}", mask_rom_version);

    println!(" >> Header validation:");
    // header checksum
    let header_checksum = data[0x14D];
    println!("Header Checksum: {:#X}", header_checksum);

    let mut calc_checksum: u8 = 0x00;
    for addr in 0x134..0x14D {
        calc_checksum = calc_checksum.wrapping_sub(data[addr] + 1);
    }
    println!("Calculated Checksum: {:#X}", calc_checksum);
    println!(
        " >> Verification {}",
        if calc_checksum == header_checksum {
            "Passed"
        } else {
            "Failed"
        }
    );

    println!("=============");
    println!("Tile test:");

    let mut cont = true;
    let mut offset = 0x64000;
    let mut input_buffer = String::new();

    while cont {
        offset += 0x10;

        let window_tile = &data[offset..offset + 0x10];
        let displayable_tile = utils::tile_to_displayable(window_tile);
        println!("{:#X}", offset);
        println!("{}", displayable_tile);

        io::stdin().read_line(&mut input_buffer)?;
        if input_buffer.trim() == "exit" {
            cont = false;
        }
        input_buffer.clear();
    }

    // let window_tile = &data[0x641C0..0x641D0];
    // let displayable_tile = utils::tile_to_displayable(window_tile);

    // println!("{}", displayable_tile);
    Ok(())
}
