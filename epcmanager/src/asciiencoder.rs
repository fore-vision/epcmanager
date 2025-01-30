use iced::widget::shader::wgpu::{hal::metal::AccelerationStructure, naga::proc::index};



pub trait AsciiEncoder {
    fn encode(&self) -> AsciiResult;
    fn decode(&self) -> AsciiResult;
    fn u8_to_ascii(data: u8) -> CharResult {
        match data {
            32..=126 => CharResult::OK(data as char), // 表示可能なASCII文字
            0 => CharResult::End,
            _ => CharResult::InvalidChar,                     // 制御文字や範囲外の場合はNone
        }
    }
}

#[derive(Clone, PartialEq, Eq,Debug)]
pub enum AsciiResult {
    OK(String),
    OKAdded(String),
    OKRemoved(String),
    OKEnded(String),
    EmptyString,
    OddNumber,
    InvalidChar,

}

pub enum CharResult {
    OK(char),
    End,
    InvalidChar,
}


pub struct EightEncoder {
    bitcount: usize,
    ascii_string: String,
    
    hex_string: String,
}

impl AsciiEncoder for EightEncoder {
    fn encode(&self) -> AsciiResult {
        let len = self.ascii_string.len();
        if len == 0 {
            return AsciiResult::EmptyString;
        }
        if !self.ascii_string.is_ascii() {
            return AsciiResult::InvalidChar;
        }
        let mut result = String::new();
        println!("Bitcount: {}", self.bitcount);
        for c in self.ascii_string.chars() {
            let hex = format!("{:02X}", c as u8);
            println!("hex =  {}", hex);
            result.push_str(&hex);
        }
        println!(" len : {}", len);
        if len *8 < self.bitcount {
            result.push_str(&"0".repeat((self.bitcount - len*8)/4));
            AsciiResult::OKAdded(result)
        }else if len * 8 > self.bitcount {
            AsciiResult::OKRemoved(result[0..self.bitcount/4].to_string())
        } else {
            AsciiResult::OK(result)
        }
    }

    fn decode(&self) -> AsciiResult {
        let mut result = String::new();
        let mut chars = self.hex_string.chars();
        if self.hex_string.len() % 2 != 0 {
            return AsciiResult::OddNumber;
        }
        if self.hex_string.len() == 0 {
            return AsciiResult::EmptyString;
        }
        if !self.hex_string.is_ascii() {
            return AsciiResult::InvalidChar;
        }

        while let Some(c) = chars.next() {
            let hex = format!("{}{}", c, chars.next().unwrap());
            let ubyte = u8::from_str_radix(&hex, 16);
            match ubyte {
                Ok(byte) => {
                    let ascii = EightEncoder::u8_to_ascii(byte);
                    match ascii {
                        CharResult::OK(c) => {
                            result.push(c);
                        }
                        CharResult::End => {
                            return AsciiResult::OKEnded(result);
                        }
                        CharResult::InvalidChar => {
                            return AsciiResult::InvalidChar;
                        }
                    }

                }
                Err(_) => {
                    return AsciiResult::InvalidChar;
                }
                
            }
        }
        if result.len() * 8 > self.bitcount {
            return AsciiResult::OKRemoved(result[0..self.bitcount/8].to_string());
        } else {
            return AsciiResult::OK(result);
        } 
    }
}

pub struct SevenEncoder {
    bitcount: usize,
    ascii_string: String,
    hex_string: String,
}

impl AsciiEncoder for SevenEncoder {
    fn encode(&self) -> AsciiResult {
        let len = self.ascii_string.len();
        if len == 0 {
            return AsciiResult::EmptyString;
        }

        if !self.ascii_string.is_ascii() {
            return AsciiResult::InvalidChar;
        }
        let mut result = String::new();
        let mut counter = 0;
        let mut current = 0;
        let mut left = 0;
        let mut right = 0;
        for c in self.ascii_string.chars() {
            let hex = c as u8;
            match counter {
                0 => {
                    current = hex << 1;
                    counter = 1;
                }
                7 => {
                    current += hex;
                    counter = 0;
                    result.push_str(&format!("{:02X}", current));
                }

                _ => {
                    left = hex >> (7-counter);
                    right = (hex << (counter+1)) & 0xFF;
                    current += left;
                    result.push_str(&format!("{:02X}", current));
                    current = right;
                    counter+=1;
                }
            }
            if counter > 0 {
                result.push_str(&format!("{:02X}", current));
            }
        }
        println!(" len : {}", len);
        if len *8 < self.bitcount {
            result.push_str(&"0".repeat((self.bitcount - len*8)/4));
            AsciiResult::OKAdded(result)
        }else if len * 8 > self.bitcount {
            AsciiResult::OKRemoved(result[0..self.bitcount/4].to_string())
        } else {
            AsciiResult::OK(result)
        }
    }

    fn decode(&self) -> AsciiResult {
        let mut result = String::new();
        let mut chars = self.hex_string.chars();
        if self.hex_string.len() % 2 != 0 {
            return AsciiResult::OddNumber;
        }
        if self.hex_string.len() == 0 {
            return AsciiResult::EmptyString;
        }
        if !self.hex_string.is_ascii() {
            return AsciiResult::InvalidChar;
        }

        let mut counter = 0;
        let mut current = 0;
        let mut index = 0;
        let mut stop = false;
        loop {
            let firstc = chars.next()
            let secondc = chars.next();
            if firstc == None || secondc == None {
                AsciiResult::InvalidChar;
            }
            let hex = format!("{}{}", firstc.unwrap(), secondc.unwrap());
            let ubyte = u8::from_str_radix(&hex, 16);
            match ubyte {
                Ok(byte) => {

                    match counter {
                        0 => {
                            let v = byte >> 1;
                            let ch = SevenEncoder::u8_to_ascii(v);
                            match ch {
                                CharResult::OK(c) => {
                                    result.push(c);
                                }
                                CharResult::End => {
                                    stop = true;
                                }
                                CharResult::InvalidChar => {
                                    return AsciiResult::InvalidChar;
                                }
                                
                            }
                            current = byte & 0x01;
                            counter = 1;
                            index += 2;
                        }
                        6 => {
                            let v = (byte >> 7) | (current << 1);
                            let six = SevenEncoder::u8_to_ascii(v);
                            match six {
                                CharResult::OK(c) => {
                                    result.push(c);
                                }
                                CharResult::End => {
                                    stop = true;
                                }
                                CharResult::InvalidChar => {
                                    return AsciiResult::InvalidChar;
                                }
                            }
                            current = byte & 0x7F;
                            let seven = SevenEncoder::u8_to_ascii(current);
                            match seven {
                                CharResult::OK(c) => {
                                    result.push(c);
                                }
                                CharResult::End => {
                                    stop = true;
                                }
                                CharResult::InvalidChar => {
                                    return AsciiResult::InvalidChar;
                                }
                            }
                            counter = 0;
                        }
                        _ => {
                            let chh = (byte >> (1+counter)) | (current << (7-counter));
                            let ch = SevenEncoder::u8_to_ascii(chh);
                            match ch {
                                CharResult::OK(c) => {
                                    result.push(c);
                                }
                                CharResult::End => {
                                    stop = true;
                                }
                                CharResult::InvalidChar => {
                                    return AsciiResult::InvalidChar;
                                }
                            }
                            current = byte & ((1 << (counter +1))-1);
                            counter += 1;
                            index += 2;
                        }
                    }
                }
                Err(_) => {
                    return AsciiResult::InvalidChar;
                }
            }
            if stop {
                break;
            }
            if index >= self.hex_string.len() {
                break;
            }
        }
        if result.len() * 8 > self.bitcount {
            return AsciiResult::OKRemoved(result[0..self.bitcount/8].to_string());
        } else {
            return AsciiResult::OK(result);
        } 
    }
}

pub struct SixEncoder {
    ascii_string: String,
    hex_string: String,
}

impl AsciiEncoder for SixEncoder {
    fn encode(&self) -> AsciiResult {
        let len = self.ascii_string.len();
        if len == 0 {
            return AsciiResult::EmptyString;
        }

        if !self.ascii_string.is_ascii() {
            return AsciiResult::InvalidChar;
        }
        let mut result = String::new();
        for c in self.ascii_string.chars() {
            let hex = format!("{:02X}", c as u8);
            result.push_str(&hex);
        }
        AsciiResult::OK(result)
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


    fn test_eight_encode_fun(bit: usize, ascii: &str,res: AsciiResult) {
        let encoder = EightEncoder {
            bitcount: bit,
            ascii_string: ascii.to_string(),
            hex_string: "".to_string(),
        };
        let result = encoder.encode();
        assert_eq!(result, res);
    }
    fn test_eight_decode_fun(bit:usize, hex: &str, res: AsciiResult) {
        let encoder = EightEncoder {
            bitcount: bit,
            ascii_string: "".to_string(),
            hex_string: hex.to_string(),
        };
        let result = encoder.decode();
        assert_eq!(result, res);
    }
    #[test]
    fn test_eight_encode() {
        test_eight_encode_fun(96, "123456781234", AsciiResult::OK(String::from("313233343536373831323334")));
        test_eight_encode_fun(96, "12345678123", AsciiResult::OKAdded(String::from("313233343536373831323300")));
        test_eight_encode_fun(96, "1234567812345678",AsciiResult::OKRemoved(String::from( "313233343536373831323334")));
        test_eight_encode_fun(96, "", AsciiResult::EmptyString);
        test_eight_encode_fun(96, "12345678あいう", AsciiResult::InvalidChar);
        test_eight_encode_fun(96, "ABCDEFGHD", AsciiResult::OKAdded(String::from("414243444546474844000000")));
        test_eight_encode_fun(32, "ABCDEFGHD", AsciiResult::OKRemoved(String::from("41424344")));
        test_eight_encode_fun(64, "ABCDEFGHD", AsciiResult::OKRemoved(String::from("4142434445464748")));
        test_eight_encode_fun(128, "^-)('&&%%%$<>;+[]", AsciiResult::OKRemoved(String::from("5E2D2928272626252525243C3E3B2B5B")));


    }
    #[test]
    fn test_eight_decode(){
        test_eight_decode_fun(96, "313233343536373831323334", AsciiResult::OK(String::from("123456781234")));
        test_eight_decode_fun(96, "313233343536373831323300", AsciiResult::OKEnded(String::from("12345678123")));
        test_eight_decode_fun(96, "31323334353637383132333434", AsciiResult::OKRemoved(String::from("123456781234")));
        test_eight_decode_fun(96, "", AsciiResult::EmptyString);
        test_eight_decode_fun(96, "313233343536373831323334あい", AsciiResult::InvalidChar);
        test_eight_decode_fun(96, "414243444546474844000000", AsciiResult::OKEnded(String::from("ABCDEFGHD")));
        test_eight_decode_fun(128, "5E2D2928272626252525243C3E3B2B5B", AsciiResult::OK(String::from("^-)('&&%%%$<>;+[")));
    }

}