pub fn tile_to_displayable(data: &[u8]) -> String {
    let mut result = String::new();
    for byte in 0..data.len() / 2 {
        let tile_low = data[byte * 2];
        let tile_high = data[byte * 2 + 1];

        for y in 0..8 {
            let pixel_low = (tile_low & (1 << (7 - y))) != 0;
            let pixel_high = (tile_high & (1 << (7 - y))) != 0;

            if pixel_low {
                if pixel_high {
                    result.push_str("\x1b[0;30m██\x1b[0m");
                } else {
                    result.push_str("\x1b[0;34m██\x1b[0m");
                }
            } else {
                if pixel_high {
                    result.push_str("\x1b[0;96m██\x1b[0m");
                } else {
                    result.push_str("\x1b[0;97m██\x1b[0m");
                }
                // 30 31 91 97 █
            }
        }
        result.push('\n');
    }
    return result;
}
