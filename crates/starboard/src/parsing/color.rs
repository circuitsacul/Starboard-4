use common::constants;

pub fn parse_color(input: &str) -> Result<i32, &str> {
    // For now, this just handles hex colors. Allowed formats should be:
    // - #<code>
    // - 0x<code>
    // - <code>

    let parsed = input.trim_start_matches("0x").trim_start_matches('#');
    let parsed = i32::from_str_radix(parsed, 16);

    match parsed {
        Ok(val) => match val > constants::HEX_MAX {
            false => Ok(val),
            true => Err("Color code was too large. Maximum value is `#FFFFFF`."),
        },
        Err(_) => Err("Invalid color code. Please pass something like `#FFE19C`."),
    }
}
