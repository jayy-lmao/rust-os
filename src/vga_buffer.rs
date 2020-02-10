#[allow(dead_code)]
#[derive(Debug, Clone, Copy PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);

impl ColourCode {
    fn new(foreground: Colour, backround: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy PartialEq, Eq)]
#[repr(transparent)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColourCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}


