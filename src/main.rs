use std::env;
use std::fs;

mod constants;
mod hardware;
mod licensee;
mod meta;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <ROM file>", args[0]);
        std::process::exit(1);
    }

    let filename = String::from(args[1].clone());
    if !fs::exists(&filename)? {
        println!("File not found");
        std::process::exit(1);
    }
    let data: Vec<u8> = fs::read(&filename)?;

    let mut cart_meta = meta::extract_metadata(&data, true);
    let mut device = hardware::Device::new(generate_banks(cart_meta.rom_size()));

    // println!("=============");
    // println!("Tile test:");

    // let mut cont = true;
    // let mut offset = 0x64000;
    // let mut input_buffer = String::new();

    // println!("{:#?}", device);

    // while cont {
    //     offset += 0x10;

    //     let window_tile = &data[offset..offset + 0x10];
    //     let displayable_tile = utils::tile_to_displayable(window_tile);
    //     println!("{:#X}", offset);
    //     println!("{}", displayable_tile);

    //     io::stdin().read_line(&mut input_buffer)?;
    //     if input_buffer.trim() == "exit" || input_buffer.trim() == "q" {
    //         cont = false;
    //     }
    //     input_buffer.clear();
    // }
    Ok(())
}

fn generate_banks(size: u8) -> Vec<hardware::MBC> {
    let mut rom_banks = Vec::new();
    let rom_banks_amount = constants::get_num_rom_banks(size);
    for bank in 0..rom_banks_amount {
        let rom_bank = hardware::MBC::new(Vec::new(), (bank as u32) * (0x4000 as u32));
        rom_banks.push(rom_bank);
    }
    return rom_banks;
}
