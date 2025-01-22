
pub trait AsciiEncoder {
    fn encode(&self) -> Result<str,str>;
    fn decode(&self) -> Result<str,str>;
}

pub struct EightEncoder {
    ascii_string: String,
    
    hex_string: String,
}

impl AsciiEncoder for EightEncoder {
    fn encode(&self) -> Result<String,str> {
        let len = self.ascii_string.len();
        if len == 0 {
            return Err("Empty string");
        }
        if len % 2 != 0 {
            return Err("Odd number of characters");
        }
        if !self.ascii_string.is_ascii() {
            return Err("Non-ASCII characters");
        }
        let mut result = String::new();
        for c in self.ascii_string.chars() {
            let hex = format!("{:02X}", c as u8);
            result.push_str(&hex);
        }
        OK(result)
    }

    fn decode(&self) -> Result<String,String> {
        let mut result = String::new();
        let mut chars = self.hex_string.chars();
        while let Some(c) = chars.next() {
            let hex = format!("{}{}", c, chars.next().unwrap());
            let byte = u8::from_str_radix(&hex, 16).unwrap();
            result.push(byte as char);
        }
        OK(result)
    }
}

pub struct SevenEncoder {
    ascii_string: String,
    hex_string: String,
}

impl AsciiEncoder for SevenEncoder {
    fn encode(&self) -> Result<String,String> {
        let mut result = String::new();
        for c in self.ascii_string.chars() {
            let hex = format!("{:02X}", c as u8);
            result.push_str(&hex);
        }
        OK(result)
    }

    fn decode(&self) -> Result<String,String> {
        let mut result = String::new();
        let mut chars = self.hex_string.chars();
        while let Some(c) = chars.next() {
            let hex = format!("{}{}", c, chars.next().unwrap());
            let byte = u8::from_str_radix(&hex, 16).unwrap();
            result.push(byte as char);
        }
        OK(result)
    }
}

pub struct SixEncoder {
    ascii_string: String,
    hex_string: String,
}

impl AsciiEncoder for SixEncoder {
    fn encode(&self) -> Result<String,String> {
        let mut result = String::new();
        for c in self.ascii_string.chars() {
            let hex = format!("{:02X}", c as u8);
            result.push_str(&hex);
        }
        OK(result)
    }

    fn decode(&self) -> Result<String,String> {
        let mut result = String::new();
        let mut chars = self.hex_string.chars();
        while let Some(c) = chars.next() {
            let hex = format!("{}{}", c, chars.next().unwrap());
            let byte = u8::from_str_radix(&hex, 16).unwrap();
            result.push(byte as char);
        }
        OK(result)
    }
}
