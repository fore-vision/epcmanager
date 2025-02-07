
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

pub enum CharResult {
    OK(char),
    End,
    InvalidChar,
}



