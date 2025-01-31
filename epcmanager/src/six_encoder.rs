use crate::ascii_encoder::{AsciiEncoder, AsciiResult, CharResult};

enum SixResult {
    OK,
    Added,
    Removed,
}

pub struct SixEncoder {
    bitcount: usize,
    ascii_string: String,
    hex_string: String,
}

impl AsciiEncoder for SixEncoder {
    fn encode(&self) -> AsciiResult {
        let mut sixResult = SixResult::OK;
        let len = self.ascii_string.len();
        if len == 0 {
            return AsciiResult::EmptyString;
        }

        if !self.ascii_string.is_ascii() {
            return AsciiResult::InvalidChar;
        }
        let encode_string = if len * 6 > self.bitcount {
            sixResult = SixResult::Removed;
            self.ascii_string[0..self.bitcount / 6].to_string()
        } else if len * 6 < self.bitcount {
            let mut padded_string = self.ascii_string.clone();
            padded_string.push_str(&" ".repeat((self.bitcount - len * 6) / 6));
            sixResult = SixResult::Added;
            padded_string.clone()

        } else {
            self.ascii_string.clone()
        };
        let mut counter = 0;
        let mut current = 0;
        let mut result = String::new();
        let mut next = 0;
        for c in encode_string.chars() {
            let byte = c as u8;
            println!("byte = {:},counter = {},current = {}, next ={}", byte,counter,current,next);
            let code =if byte > 0x40 {
                byte - 0x40
            } else {
                byte
            };
            match counter {
                0 => {
                    current = code << 2;
                }
                2 => {
                    current += code;
                }
                _ => {
                    let first = counter - 2;
                    current += code >> first;
                    let second = 8-first;
                    let mask = ((1<< first) -1) & 0xff;
                    let sub = (mask & code) & 0xff;
                    next = (sub << second) & 0xff;



                }
                
            }
            counter +=6;
            if counter >= 8 {
                let hex = format!("{:02X}", current);
                result.push_str(&hex);
                counter -= 8;
                current = next;
                next = 0;
                println!("hex = {:}", hex);
            }

        }
        if counter > 0 {
            let hex = format!("{:02X}", current);
            result.push_str(&hex);
            println!("counter= {}, hex = {:}",counter, hex);
        }
        match sixResult {
            SixResult::OK => AsciiResult::OK(result),
            SixResult::Added => AsciiResult::OKAdded(result),
            SixResult::Removed => AsciiResult::OKRemoved(result),            
        }
    }

    fn decode(&self) -> AsciiResult {
        let mut result = String::new();
        let mut chars = self.hex_string.chars();
        while let Some(c) = chars.next() {
            let hex = format!("{}{}", c, chars.next().unwrap());
            let byte = u8::from_str_radix(&hex, 16).unwrap();
            result.push(byte as char);
        }
        AsciiResult::OK(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_six_encode_fun(bit: usize, ascii: &str,res: AsciiResult) {
        let encoder = SixEncoder {
            bitcount: bit,
            ascii_string: ascii.to_string(),
            hex_string: "".to_string(),
        };
        let result = encoder.encode();
        assert_eq!(result, res);
    }

    #[test]
    fn test_six_encode(){
        test_six_encode_fun(96, "ABCDEFGHIJKLMNOP", AsciiResult::OK(String::from("0420C41461C824A2CC34E3D0")));
        test_six_encode_fun(96, "ABCDEFGHIJKLMNO", AsciiResult::OKAdded(String::from("0420C41461C824A2CC34E3E0")));
    }

}