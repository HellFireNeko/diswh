pub struct Color;

impl Color {
    pub const DEFAULT: i32 = 0;
    pub const AQUA: i32 = 1752220;
    pub const DARK_AQUA: i32 = 1146986;
    pub const GREEN: i32 = 5763719;
    pub const DARK_GREEN: i32 = 2067276;
    pub const BLUE: i32 = 3447003;
    pub const DARK_BLUE: i32 = 2123412;
    pub const PURPLE: i32 = 10181046;
    pub const DARK_PURPLE: i32 = 7419530;
    pub const PINK: i32 = 15277667;
    pub const DARK_PINK: i32 = 11342935;
    pub const GOLD: i32 = 15844367;
    pub const DARK_GOLD: i32 = 12745742;
    pub const ORANGE: i32 = 15105570;
    pub const DARK_ORANGE: i32 = 11027200;
    pub const RED: i32 = 15548997;
    pub const DARK_RED: i32 = 10038562;
    pub const GREY: i32 = 9807270;
    pub const DARK_GREY: i32 = 9936031;
    pub const DARKER_GREY: i32 = 8359053;
    pub const LIGHT_GREY: i32 = 12370112;
    pub const NAVY: i32 = 3426654;
    pub const DARK_NAVY: i32 = 2899536;
    pub const YELLOW: i32 = 16776960;

    pub fn from_hex(hex_val: &str) -> i32 {
        let len: usize = hex_val.len();
        let mut base: i32 = 1;
        let mut dec_val: i32 = 0;

        for i in (0..len).rev() {
            let c: char = hex_val.chars().nth(i).unwrap();
            if c.is_digit(10) {
                dec_val += (c as i32 - '0' as i32) * base;
                base *= 16;
            } else if c.is_ascii_uppercase() {
                dec_val += (c as i32 - 'A' as i32 + 10) * base;
                base *= 16;
            }
        }

        dec_val
    }
}