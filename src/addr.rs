use std::str::FromStr;

use log::warn;

use crate::addr_input::AddrMode;

pub trait GenericAddressString {
    fn get_addr(&self) -> Option<u64>;
}

impl GenericAddressString for String {
    fn get_addr(&self) -> Option<u64> {
        let text = self.trim().to_lowercase();
        let mut text = text.as_str();
        let mut hex = !text.chars().all(|x| char::is_ascii_digit(&x));

        if let Some(hex_str) = text.strip_prefix("0x") {
            text = hex_str;
            hex = true;
        }

        if hex {
            u64::from_str_radix(text, 16).ok()
        } else {
            text.parse::<u64>().ok()
        }
    }
}

#[derive(Debug)]
pub enum AddrString {
    Hex(String),
    Dec(String),
}

impl AddrString {
    pub fn default_decimal() -> Self {
        Self::Dec("0".to_string())
    }

    pub fn new(s: String, m: AddrMode) -> Self {
        match m {
            AddrMode::Hex => Self::Hex(s),
            AddrMode::Dec => Self::Dec(s),
        }
    }

    pub fn mode(&self) -> AddrMode {
        match self {
            Self::Hex(..) => AddrMode::Hex,
            Self::Dec(..) => AddrMode::Dec,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Hex(s) => s.is_empty(),
            Self::Dec(s) => s.is_empty(),
        }
    }

    pub fn toggle_mode(&mut self) {
        let addr = match self.get_addr() {
            Some(addr) => addr,
            None => {
                warn!("Can't get addr from {}", self.buffer());
                return;
            }
        };

        *self = match self {
            Self::Dec(..) => Self::Hex(format!("0x{:X}", addr)),
            Self::Hex(..) => Self::Dec(format!("{}", addr)),
        }
    }

    pub fn buffer(&mut self) -> &mut String {
        match self {
            Self::Hex(s) => s,
            Self::Dec(s) => s,
        }
    }

    pub fn update_data(&mut self, d: u64) {
        match self {
            Self::Hex(ref mut s) => *s = format!("0x{:X}", d),
            Self::Dec(ref mut s) => *s = format!("{}", d),
        }
    }
}

impl GenericAddressString for AddrString {
    fn get_addr(&self) -> Option<u64> {
        match self {
            Self::Hex(s) => {
                let text = s.trim().to_lowercase();
                let mut text = text.as_str();
                if let Some(hex_str) = text.strip_prefix("0x") {
                    text = hex_str;
                }

                u64::from_str_radix(text, 16).ok()
            }

            Self::Dec(s) => s.trim().parse::<u64>().ok(),
        }
    }
}

impl Default for AddrString {
    fn default() -> Self {
        Self::new(String::from_str("0x0").unwrap(), AddrMode::Hex)
    }
}
