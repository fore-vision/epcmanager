use crate::EightEncoder;
use crate::SevenEncoder;
use crate::SixEncoder;


pub trait AsciiEncoder {
    fn encode(&self, ascii: &str) -> AsciiResult;
    fn decode(&self, hex: &str) -> AsciiResult;
    fn u8_to_ascii(data: u8) -> CharResult {
        match data {
            32..=126 => CharResult::OK(data as char), // 表示可能なASCII文字
            0 => CharResult::End,
            _ => CharResult::InvalidChar,                     // 制御文字や範囲外の場合はNone
        }
    }
    fn check_ascii(&self, ascii: &str) -> StringResult;
    fn check_ascii_len(&self, ascii: &str) -> StringResult {
        let len = ascii.len();
        if len == 0 {
            return StringResult::EmptyString;
        }
        if !ascii.is_ascii() {
            return StringResult::InvalidChar;
        }
        StringResult::OK
    }
    fn check_hex(&self, hex: &str) -> StringResult;
    fn check_hex_len(&self, hex: &str) -> StringResult {
        let len = hex.len();
        if len % 2 != 0 {
            return StringResult::OddNumber;
        }
        if len == 0 {
            return StringResult::EmptyString;
        }
        if !hex.is_ascii() {
            return StringResult::InvalidChar;
        }
        if hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return StringResult::OK;
        }
        StringResult::InvalidChar
    }

}

pub enum AsciiEncoderType {
    Eight(EightEncoder),
    Seven(SevenEncoder),
    Six(SixEncoder),
}

impl AsciiEncoder for AsciiEncoderType {
    fn encode(&self, ascii: &str) -> AsciiResult {
        match self {
            AsciiEncoderType::Eight(encoder) => encoder.encode(ascii),
            AsciiEncoderType::Seven(encoder) => encoder.encode(ascii),
            AsciiEncoderType::Six(encoder) => encoder.encode(ascii),
        }
    }
    fn decode(&self, hex: &str) -> AsciiResult {
        match self {
            AsciiEncoderType::Eight(encoder) => encoder.decode(hex),
            AsciiEncoderType::Seven(encoder) => encoder.decode(hex),
            AsciiEncoderType::Six(encoder) => encoder.decode(hex),
        }
    }
    fn check_ascii(&self, ascii: &str) -> StringResult {
        match self {
            AsciiEncoderType::Eight(encoder) => encoder.check_ascii(ascii),
            AsciiEncoderType::Seven(encoder) => encoder.check_ascii(ascii),
            AsciiEncoderType::Six(encoder) => encoder.check_ascii(ascii),
        }
    }
    fn check_hex(&self, hex: &str) -> StringResult {
        match self {
            AsciiEncoderType::Eight(encoder) => encoder.check_hex(hex),
            AsciiEncoderType::Seven(encoder) => encoder.check_hex(hex),
            AsciiEncoderType::Six(encoder) => encoder.check_hex(hex),
        }
    }
}

pub  struct BaseEncoder {
    bitcount: usize,
}

impl BaseEncoder {
    pub fn new(bitcount: usize) -> Self {
        Self {
            bitcount,
        }
    }
    pub fn get_bitcount(&self) -> usize {
        self.bitcount
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

#[derive(Clone, PartialEq, Eq,Debug)]

pub enum StringResult {
    OK,
    ShortString,
    LongString,
    EmptyString,
    InvalidChar,
    OddNumber
}

impl StringResult {
    pub fn get_message(&self) -> String {
        match self {
            StringResult::OK => String::from("OK"),
            StringResult::ShortString => String::from("Short string"),
            StringResult::LongString => String::from("Long string"),
            StringResult::EmptyString => String::from("Empty string"),
            StringResult::InvalidChar => String::from("Invalid character"),
            StringResult::OddNumber => String::from("Odd number of characters"),
        }
    }
    pub fn is_ok(&self) -> bool {
        match self {
            StringResult::OK => true,
            StringResult::ShortString => true,
            //StringResult::LongString => true,
            _ => false,
        }
    }
}

pub enum CharResult {
    OK(char),
    End,
    InvalidChar,
}



