use std::fmt;

pub enum Color {
    Green,
    Yellow,
    Gray,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Green => write!(f, "🟩"),
            Color::Yellow => write!(f, "🟨"),
            Color::Gray => write!(f, "⬜"),
        }
    }
}

impl Color {
    pub const fn encode(&self) -> u32 {
        match self {
            Color::Green => 0b11,
            Color::Yellow => 0b10,
            Color::Gray => 0b01,
        }
    }

    pub const fn decode(c: u32) -> Option<Self> {
        match c {
            0b11 => Some(Color::Green),
            0b10 => Some(Color::Yellow),
            0b01 => Some(Color::Gray),
            _ => None,
        }
    }
}

pub fn encode(colors: &[Color]) -> u32 {
    let mut pat = 0u32;
    for (i, color) in colors.iter().enumerate() {
        pat += color.encode() << (2 * i);
    }
    pat
}

pub fn pat_to_string(mut pat: u32) -> String {
    let mut s = String::new();
    while pat != 0u32 {
        match Color::decode(pat % 4) {
            Some(c) => s += &c.to_string(),
            None => s += "?",
        }

        pat >>= 2;
    }
    s
}
