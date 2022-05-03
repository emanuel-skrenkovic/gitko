#![allow(dead_code)]

pub const KEY_NULL: i32 = 0;
pub const KEY_EOT: i32 = 4;
pub const KEY_BS: i32 = 8;
pub const KEY_LF: i32 = 10;
pub const KEY_NAK: i32 = 21;
pub const KEY_ETB: i32 = 27;
pub const KEY_ESC: i32 = 33;
pub const KEY_FORWARD_SLASH: i32 = 47;
pub const KEY_COLON: i32 = 58;
pub const KEY_C_UPPER: i32 = 67;
pub const KEY_B_LOWER: i32 = 98;
pub const KEY_C_LOWER: i32 = 99;
pub const KEY_D_LOWER: i32 = 100;
pub const KEY_Q_LOWER: i32 = 113;
pub const KEY_H_LOWER: i32 = 104;
pub const KEY_J_LOWER: i32 = 106;
pub const KEY_K_LOWER: i32 = 107;
pub const KEY_L_LOWER: i32 = 108;
pub const KEY_N_LOWER: i32 = 110;
pub const KEY_T_LOWER: i32 = 116;
pub const KEY_U_LOWER: i32 = 117;
pub const KEY_W_LOWER: i32 = 119;
pub const KEY_Y_LOWER: i32 = 121;
pub const KEY_DEL: i32 = 127;
pub const KEY_ZERO: i32 = 48;
pub const KEY_DOLLAR: i32 = 36;

const ASCII_CHAR_TABLE: [&str; 128] = [
    "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL", "BS", "\t", "LF", "VT", "FF", "CR",
    "SO", "SI", "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB", "CAN", "EM", "SUB", "ESC",
    "FS", "GS", "RS", "US", " ", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-",
    ".", "/", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?", "@",
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "[", "\\", "]", "^", "_", "`", "a", "b", "c", "d", "e", "f",
    "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y",
    "z", "{", "|", "}", "~", "DEL",
];

pub fn ascii_to_char(ascii_code: i32) -> &'static str {
    ASCII_CHAR_TABLE[ascii_code as usize]
}
