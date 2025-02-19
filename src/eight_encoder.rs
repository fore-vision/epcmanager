use crate::ascii_encoder::{AsciiEncoder, AsciiResult, BaseEncoder, CharResult, StringResult};

pub struct EightEncoder {
    base: BaseEncoder,
}

impl EightEncoder {
    pub fn new(bitcount: usize) -> Self {
        Self {
            base: BaseEncoder::new(bitcount),
        }
    }
}

impl AsciiEncoder for EightEncoder {
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
        let bitcount = self.base.get_bitcount();
        for c in ascii_string.chars() {
            let hex = format!("{:02X}", c as u8);
            result.push_str(&hex);
        }
        if len *8 < bitcount {
            result.push_str(&"0".repeat((bitcount - len*8)/4));
            AsciiResult::OKAdded(result)
        }else if len * 8 > bitcount {
            AsciiResult::OKRemoved(result[0..bitcount/4].to_string())
        } else {
            AsciiResult::OK(result)
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
        let bitcount = self.base.get_bitcount();
        if result.len() * 8 > bitcount {
            return AsciiResult::OKRemoved(result[0..bitcount/8].to_string());
        } else {
            return AsciiResult::OK(result);
        } 
    }
    fn check_ascii(&self, ascii: &str) -> StringResult {
        let result = self.check_ascii_len(ascii);
        if result != StringResult::OK {
            return result;
        }
        let len = ascii.len() * 8;
        
        let bitcount = self.base.get_bitcount();
        if len  < bitcount {
            return StringResult::ShortString;
        } else if len == bitcount {
            return StringResult::OK;
        } 
        return StringResult::LongString;
        
    }
    fn check_hex(&self, hex: &str) -> StringResult {
        let result = self.check_hex_len(hex);
        if result != StringResult::OK {
            return result;
        }
        let len = hex.len() * 4;
        let bitcount = self.base.get_bitcount();
        if len  < bitcount {
            return StringResult::ShortString;
        } else if len == bitcount {
            return StringResult::OK;
        } 
        return StringResult::LongString;
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    fn test_eight_encode_fun(bit: usize, ascii: &str,res: AsciiResult) {
        let encoder = EightEncoder::new(bit);
        let result = encoder.encode( ascii);
        assert_eq!(result, res);
    }
    fn test_eight_decode_fun(bit:usize, hex: &str, res: AsciiResult) {
        let encoder = EightEncoder::new(bit);
        let result = encoder.decode(hex);
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
