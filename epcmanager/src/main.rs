use ascii_encoder::{AsciiEncoder, AsciiEncoderType};
use eight_encoder::EightEncoder;
use seven_encoder::SevenEncoder;
use six_encoder::SixEncoder;
use iced::widget::{button, text_input,column,combo_box,container};
use iced::Fill;
mod ascii_encoder;
mod eight_encoder;
mod seven_encoder;
mod six_encoder;

#[derive(Default,Clone,Copy, PartialEq, Eq,Debug)]
enum Bit {
    Eight,
    #[default]
    Seven,
    Six,
}
impl Bit {
    const ALL: [Bit; 3] = [Bit::Eight, Bit::Seven, Bit::Six];
    fn str(&self) -> &str {
        match self {
            Bit::Eight => "8",
            Bit::Seven => "7",
            Bit::Six => "6",
        }
    }
    fn get_encode(&self,bit: usize) -> AsciiEncoderType {
        match self {
            Bit::Eight => AsciiEncoderType::Eight(EightEncoder::new(bit)),
            Bit::Seven => AsciiEncoderType::Seven(SevenEncoder::new(bit)),
            Bit::Six => AsciiEncoderType::Six(SixEncoder::new(bit)),
        }

    }
}


impl std::fmt::Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Bit Encoder", self.str())
    }
}

#[derive(Default,Clone,Copy, PartialEq, Eq,Debug)]
enum NumBits {
    Bit128,
    #[default]
    Bit96,
    Bit64,
}

impl NumBits {
    const ALL: [NumBits; 3] = [NumBits::Bit128, NumBits::Bit96, NumBits::Bit64];
    fn str(&self) -> &str {
        match self {
            NumBits::Bit128 => "128",
            NumBits::Bit96 => "96",
            NumBits::Bit64 => "64",
        }
    }
    fn get_num(&self) -> usize {
        match self {
            NumBits::Bit128 => 128,
            NumBits::Bit96 => 96,
            NumBits::Bit64 => 64,
        }
    }
}

impl std::fmt::Display for NumBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Bit size", self.str())
    }
}

struct EpcManager {
    ascii_string: String,
    hex_string: String,
    bit: combo_box::State<Bit>,
    selected_bit: Option<Bit>,
    num_bits: combo_box::State<NumBits>,
    selected_num_bits: Option<NumBits>,

}

#[derive(Debug, Clone)]
enum Message {
    Encode,
    Decode,
    SelectedBit(Bit),
    SelectedNumBits(NumBits),
    AsciiChanged(String),
    HexChanged(String),
}

impl EpcManager {
    fn new() -> Self {
        Self {
            ascii_string: String::new(),
            hex_string: String::new(),
            bit: combo_box::State::new(Bit::ALL.to_vec()),
            selected_bit: Bit::Eight.into(),
            num_bits: combo_box::State::new(NumBits::ALL.to_vec()),
            selected_num_bits: NumBits::Bit96.into(),
        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Encode => {
                if !self.selected_bit.is_none() {
                    let bit = self.selected_num_bits.unwrap().get_num();
                    let encoder = self.selected_bit.unwrap().get_encode(bit);
                    let _result = encoder.encode(&self.ascii_string);
                    println!("{:?}", _result);
                    match _result {
                        ascii_encoder::AsciiResult::OK(hex) => {
                            self.hex_string = hex;
                        }
                        ascii_encoder::AsciiResult::OKAdded(hex) => {
                            self.hex_string = hex;
                        }
                        ascii_encoder::AsciiResult::OKRemoved(hex) => {
                            self.hex_string = hex;
                        }
                        _ => {}
                        
                    }
    
                }
                
            }
            Message::Decode => {
                if !self.selected_bit.is_none() {
                    let bit = self.selected_num_bits.unwrap().get_num();

                    let encoder = self.selected_bit.unwrap().get_encode(bit);
                    let _result = encoder.decode(&self.hex_string);
                    println!("{:?}", _result);
                    match _result {
                        ascii_encoder::AsciiResult::OK(hex) => {
                            self.ascii_string = hex;
                        }
                        ascii_encoder::AsciiResult::OKAdded(hex) => {
                            self.ascii_string = hex;
                        }
                        ascii_encoder::AsciiResult::OKRemoved(hex) => {
                            self.ascii_string = hex;
                        }
                        _ => {}
                        
                    }
    
                }

            }
            Message::SelectedBit(bit) => {
                self.selected_bit = Some(bit);
            }
            Message::AsciiChanged(ascii) => {
                self.ascii_string = ascii;
            }
            Message::HexChanged(hex) => {
                self.hex_string = hex;
            }
            Message::SelectedNumBits(num_bits) => {
                self.selected_num_bits = Some(num_bits);
            }
        
        }
    }
    fn view(&self) -> iced::Element<Message> {
        container(
            column![
                button("Encode").on_press(Message::Encode),
                combo_box(&self.bit, "bit", self.selected_bit.as_ref(),Message::SelectedBit)
                ,
                combo_box(&self.num_bits, "num_bits", self.selected_num_bits.as_ref(),Message::SelectedNumBits) ,
                text_input("ASCII", &self.ascii_string).on_input(Message::AsciiChanged),
                text_input("HEX", &self.hex_string).on_input(Message::HexChanged),
                button("Decode").on_press(Message::Decode),
            ].spacing(10)
        ).padding(10).height(300).width(300)

            .into()
    }
}


impl Default for EpcManager {
    fn default() -> Self {
        Self::new()
    }
}


pub fn main() {
    if let Err(e) = iced::run("EPC Manager", EpcManager::update, EpcManager::view) {
        eprintln!("Error: {:?}", e);
    }
}
