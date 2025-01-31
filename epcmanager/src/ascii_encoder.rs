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



