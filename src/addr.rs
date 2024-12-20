
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
