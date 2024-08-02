#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn white() -> Self {
        Self { red: 255, green: 255, blue: 255 }
    }

    pub fn black() -> Self {
        Self { red: 0, green: 0, blue: 0 }
    }

    pub fn green() -> Self {
        Self { red: 0, green: 255, blue: 0 }
    }

    pub fn yellow() -> Self {
        Self { red: 255, green: 255, blue: 0 }
    }

    pub fn blue() -> Self {
        Self { red: 0, green: 0, blue: 255 }
    }

    pub fn red() -> Self {
        Self { red: 255, green: 0, blue: 0 }
    }

    pub fn to_u32(self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }
}
