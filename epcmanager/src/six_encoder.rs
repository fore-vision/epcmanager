use crate::ascii_encoder::{AsciiEncoder, AsciiResult};

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
        let mut six_result = SixResult::OK;
        let len = self.ascii_string.len();
        if len == 0 {
            return AsciiResult::EmptyString;
        }

        if !self.ascii_string.is_ascii() {
            return AsciiResult::InvalidChar;
        }
        let encode_string = if len * 6 > self.bitcount {
            six_result = SixResult::Removed;
            self.ascii_string[0..self.bitcount / 6].to_string()
        } else if len * 6 < self.bitcount {
            let mut padded_string = self.ascii_string.clone();
            padded_string.push_str(&" ".repeat((self.bitcount - len * 6) / 6));
            six_result = SixResult::Added;
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
        match six_result {
            SixResult::OK => AsciiResult::OK(result),
            SixResult::Added => AsciiResult::OKAdded(result),
            SixResult::Removed => AsciiResult::OKRemoved(result),            
        }
    }


    fn decode(&self) -> AsciiResult {
        fn decode_char(byte: u8) -> u8 {
            match byte {
                0 => 0,
                1..0x20 => byte + 0x40,
                _ => byte,
            }
        }
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
        let mut next = 0;
        let mut stop = false;

        loop {
            let firstc = chars.next();
            let secondc = chars.next();
            println!("firstc = {:?}, secondc = {:?}", firstc, secondc);
            if firstc.is_none() || secondc.is_none() {
                break;
            }
            let hex = format!("{}{}", firstc.unwrap(), secondc.unwrap());
            let ubyte = u8::from_str_radix(&hex, 16);
            if ubyte.is_err() {
                return AsciiResult::InvalidChar;
            } else {
                let byte = ubyte.unwrap();
                println!("byte = {:02X}", byte);
                match counter %3 {
                    0 => {
                        let v = (byte  >> 2) & 0x3f;
                        let d = decode_char(v);
                        println!("0 counter = {}, v = {:02X}, d = {:02X}",counter, v, d);
                        match d {
                            32 => {
                                stop = true;
                            }
                            33..127 => {
                                result.push(d as char);
                                next = (byte & 0x03) << 4;
                                println!("next = {:02X}", next);
                            }
                            _ => {
                                return AsciiResult::InvalidChar;
                            }
                        }
                    }
                    1 => {
                        let v = (byte >> 4) & 0x0f + next;
                        let d = decode_char(v);
                        println!("1 counter = {}, v = {:02X}, d = {:02X}",counter, v, d);
                        match d {
                            32 => {
                                stop = true;
                            }
                            33..127 => {
                                result.push(d as char);
                                next = (byte & 0x0f) << 2;
                                println!("next = {:02X}", next);
                            }
                            _ => {
                                return AsciiResult::InvalidChar;
                            }
                        }
                    }
                    _ => {
                        let v = ((byte >> 6) & 0x03) + next;
                        let d = decode_char(v);
                        println!("2 counter = {}, v = {:02X}, d = {:02X}, next = {}",counter, v, d,next);
                        match d {
                            32 => {
                                stop = true;
                            }
                            33..127 => {
                                result.push(d as char);
                                next = byte & 0x3f;
                                let nd = decode_char(next);
                                println!("next = {:02X}, nd = {:02X}", next, nd);
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
        let encoder = SixEncoder {
            bitcount: bit,
            ascii_string: ascii.to_string(),
            hex_string: "".to_string(),
        };
        let result = encoder.encode();
        assert_eq!(result, res);
    }


    fn test_six_decode_fun(bit:usize, hex: &str, res: AsciiResult) {
        let encoder = SixEncoder {
            bitcount: bit,
            ascii_string: "".to_string(),
            hex_string: hex.to_string(),
        };
        let result = encoder.decode();
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