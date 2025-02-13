use iced::widget::shader::wgpu::hal::auxil::db;

use crate::ascii_encoder::{AsciiEncoder, AsciiResult, BaseEncoder};

enum SixResult {
    OK,
    Added,
    Removed,
}

pub struct SixEncoder {
    base: BaseEncoder,
}
impl SixEncoder {
    pub fn new(bitcount: usize) -> Self {
        Self {
            base: BaseEncoder::new(bitcount),
        }
    }
}

impl AsciiEncoder for SixEncoder {
    fn encode(&self, ascii: &str) -> AsciiResult {
        let mut six_result = SixResult::OK;
        let ascii_string = String::from(ascii);
        let len = ascii_string.len();
        if len == 0 {
            return AsciiResult::EmptyString;
        }

        if !ascii_string.is_ascii() {
            return AsciiResult::InvalidChar;
        }
        let bitcount = self.base.get_bitcount();
        let encode_string = if len * 6 > bitcount {
            six_result = SixResult::Removed;
            ascii_string[0..bitcount / 6].to_string()
        } else if len * 6 < bitcount {
            let mut padded_string = ascii_string.clone();
            padded_string.push_str(&" ".repeat((bitcount - len * 6) / 6));
            six_result = SixResult::Added;
            padded_string.clone()

        } else {
            ascii_string.clone()
        };
        let mut counter = 0;
        let mut current = 0;
        let mut result = String::new();
        let mut next = 0;
        for c in encode_string.chars() {
            let byte = c as u8;
            dbg!(byte);
            dbg!(counter);
            dbg!(current);
            dbg!(next);
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
            dbg!(&counter);
            dbg!(hex);

        }
        match six_result {
            SixResult::OK => AsciiResult::OK(result),
            SixResult::Added => AsciiResult::OKAdded(result),
            SixResult::Removed => AsciiResult::OKRemoved(result),            
        }
    }


    fn decode(&self, hex: &str) -> AsciiResult {
        fn decode_char(byte: u8) -> u8 {
            match byte {
                0 => 0,
                1..0x20 => byte + 0x40,
                _ => byte,
            }
        }
        let mut result = String::new();
        let hex_string = String::from(hex);
        let mut chars = hex_string.chars();
        if hex_string.len() % 2 != 0 {
            return AsciiResult::OddNumber;
        }
        if hex_string.len() == 0 {
            return AsciiResult::EmptyString;
        }
        if !hex_string.is_ascii() {
            return AsciiResult::InvalidChar;
        }

        let mut counter = 0;
        let mut next = 0;
        let mut stop = false;

        loop {
            let firstc = chars.next();
            let secondc = chars.next();
            dbg!(&firstc);
            dbg!(&secondc);
            if firstc.is_none() || secondc.is_none() {
                break;
            }
            let hex = format!("{}{}", firstc.unwrap(), secondc.unwrap());
            let ubyte = u8::from_str_radix(&hex, 16);
            if ubyte.is_err() {
                return AsciiResult::InvalidChar;
            } else {
                let byte = ubyte.unwrap();
                dbg!(byte);
                dbg!(counter);
                match counter %3 {
                    0 => {
                        let v = (byte  >> 2) & 0x3f;
                        let d = decode_char(v);
                        dbg!(v);
                        dbg!(d);
                        match d {
                            32 => {
                                stop = true;
                            }
                            33..127 => {
                                result.push(d as char);
                                next = (byte & 0x03) << 4;
                                dbg!(next);
                            }
                            _ => {
                                return AsciiResult::InvalidChar;
                            }
                        }
                    }
                    1 => {
                        let v = ((byte >> 4) & 0x0f) + next;
                        let d = decode_char(v);
                        dbg!(v);
                        dbg!(d);
                        match d {
                            32 => {
                                stop = true;
                            }
                            33..127 => {
                                result.push(d as char);
                                next = (byte & 0x0f) << 2;
                                dbg!(next);
                            }
                            _ => {
                                return AsciiResult::InvalidChar;
                            }
                        }
                    }
                    _ => {
                        let v = ((byte >> 6) & 0x03) + next;
                        let d = decode_char(v);
                        dbg!(v);
                        dbg!(d);
                        match d {
                            32 => {
                                stop = true;
                            }
                            33..127 => {
                                result.push(d as char);
                                next = byte & 0x3f;
                                let nd = decode_char(next);
                                dbg!(nd);
                                dbg!(next);
                                match nd {
                                    32 => {
                                        stop = true;
                                    }
                                    33..127 => {
                                        result.push(nd as char);
                                    }
                                    _ => {
                                        return AsciiResult::InvalidChar;
                                    }
                                }
                            }
                            _ => {
                                return AsciiResult::InvalidChar;
                            }
                        }
                    }
                }
                counter += 1;
                if stop {
                    break;
                }
            }

        }
        AsciiResult::OK(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_six_encode_fun(bit: usize, ascii: &str,res: AsciiResult) {
        let encoder = SixEncoder::new(bit);
        let result = encoder.encode(ascii);
        assert_eq!(result, res);
    }


    fn test_six_decode_fun(bit:usize, hex: &str, res: AsciiResult) {
        let encoder = SixEncoder::new(bit);
        let result = encoder.decode(hex);
        assert_eq!(result, res);
    }
    #[test]
    fn test_six_encode(){
        test_six_encode_fun(96, "ABCDEFGHIJKLMNOP", AsciiResult::OK(String::from("0420C41461C824A2CC34E3D0")));
        test_six_encode_fun(96, "ABCDEFGHIJKLMNO", AsciiResult::OKAdded(String::from("0420C41461C824A2CC34E3E0")));
        test_six_decode_fun(96, "", AsciiResult::EmptyString);
        test_six_decode_fun(96, "アイウエオか", AsciiResult::InvalidChar);
    }

    #[test]
    fn text_six_decode() {
        test_six_decode_fun(96, "0420C41461C824A2CC34E3D0", AsciiResult::OK(String::from("ABCDEFGHIJKLMNOP")));
        test_six_decode_fun(96, "0420C41461C824A2CC34E3E0", AsciiResult::OK(String::from("ABCDEFGHIJKLMNO")));
    }


}