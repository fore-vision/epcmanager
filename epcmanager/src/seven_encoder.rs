use crate::ascii_encoder::{AsciiEncoder, AsciiResult, CharResult, BaseEncoder};


pub struct SevenEncoder {
 base: BaseEncoder,
}

impl SevenEncoder {
    pub fn new(bitcount: usize) -> Self {
        Self {
            base: BaseEncoder::new(bitcount),
        }
    }
}
impl AsciiEncoder for SevenEncoder {
    fn encode(&self, ascii: &str) -> AsciiResult {
        let ascii_string = String::from(ascii);
        let len = ascii_string.len();
        if len == 0 {
            return AsciiResult::EmptyString;
        }

        if !ascii_string.is_ascii() {
            return AsciiResult::InvalidChar;
        }
        let mut result = String::new();
        let mut counter = 0;
        let mut current = 0;
        let mut left = 0;
        let mut right = 0;
        for c in ascii_string.chars() {
            let hex = c as u8;
            dbg!(hex);
            dbg!(current);
            dbg!(counter);
            dbg!(left);
            dbg!(right);
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
        }
        if counter > 0 {
            result.push_str(&format!("{:02X}", current));
        }
        let hexlen = result.len();
        dbg!(len);
        dbg!(hexlen);
        let bitcount = self.base.get_bitcount();
        if hexlen * 4 < bitcount {
            result.push_str(&"0".repeat((bitcount - hexlen*4)/4));
            return AsciiResult::OKAdded(result)
        } else {
            return AsciiResult::OK(result);
        } 

    }

    fn decode(&self, hex: &str) -> AsciiResult {
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
        let mut current = 0;
        let mut index = 0;
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
            if index >= hex_string.len() {
                break;
            }
        }
        if stop  {
            AsciiResult::OKEnded(result)
         } else {
            AsciiResult::OK(result)
         } 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_seven_encode_fun(bit: usize, ascii: &str,res: AsciiResult) {
        let encoder = SevenEncoder::new(bit);
        let result = encoder.encode(ascii);
        assert_eq!(result, res);
    }

    fn test_seven_decode_fun(bit:usize, hex: &str, res: AsciiResult) {
        let encoder = SevenEncoder::new(bit);
        let result = encoder.decode(hex);
        assert_eq!(result, res);
    }
    #[test]
    fn test_seven_encode(){
        test_seven_encode_fun(96, "1234567890123", AsciiResult::OK(String::from("62C99B46AD9BB872C18B2660")));
        test_seven_encode_fun(96, "123456789012", AsciiResult::OKAdded(String::from("62C99B46AD9BB872C18B2000")));
        test_seven_encode_fun(96, "", AsciiResult::EmptyString);
        test_seven_encode_fun(96, "アイウエオか", AsciiResult::InvalidChar);
        test_seven_encode_fun(96, "abcdEFghijkml", AsciiResult::OK(String::from("C38B1E48B1B3E8D3AB5EDD80")));

    }
    #[test]
    fn test_seven_decode(){
        test_seven_decode_fun(96, "", AsciiResult::EmptyString);
        test_seven_decode_fun(96, "62C99B46AD9BB872C18B2660", AsciiResult::OK(String::from("1234567890123")));
        test_seven_decode_fun(96, "62C99B46AD9BB872C18B2000", AsciiResult::OKEnded(String::from("123456789012")));
        test_seven_decode_fun(96, "葵ういえ", AsciiResult::InvalidChar);
        test_seven_decode_fun(96, "C38B1E48B1B3E8D3AB5EDD80", AsciiResult::OK(String::from("abcdEFghijkml")));
    }

}