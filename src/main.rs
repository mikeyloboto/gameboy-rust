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

    let cart_meta = meta::extract_metadata(&data, true);
    let mut device = hardware::Device::new(cart_meta);

    // Map initial ROM bank

    for bank_data in &data[0x14F..0x3FFF] {
        device.get_bank_mut(0).mem.push(*bank_data);
    }
    for bank_data in &data[0x4000..0x7FFF] {
        device.get_bank_mut(1).mem.push(*bank_data);
    }

    device.start_exec();

    // println!("{:?}", device.get_bank(0));

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
