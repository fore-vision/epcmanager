use ascii_encoder::{AsciiEncoder, AsciiEncoderType};
use eight_encoder::EightEncoder;
use seven_encoder::SevenEncoder;
use six_encoder::SixEncoder;
use iced::widget::{button, text_input,column,combo_box};
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
    fn getEncode(&self,bit: usize) -> AsciiEncoderType {
        match self {
            Bit::Eight => AsciiEncoderType::Eight(EightEncoder::new(bit)),
            Bit::Seven => AsciiEncoderType::Seven(SevenEncoder::new(bit)),
            Bit::Six => AsciiEncoderType::Six(SixEncoder::new(bit)),
        }

    }
}


impl std::fmt::Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
}




struct EpcManager {
    ascii_string: String,
    hex_string: String,
    bit: combo_box::State<Bit>,
    selected_bit: Option<Bit>,

}

#[derive(Debug, Clone)]
enum Message {
    Encode,
    Decode,
    SelectedBit(Bit),
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
        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Encode => {
                if !self.selected_bit.is_none() {
                    let encoder = self.selected_bit.unwrap().getEncode(96);
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
                    let encoder = self.selected_bit.unwrap().getEncode(96);
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
        
        }
    }
    fn view(&self) -> iced::Element<Message> {

        column![
            button("Encode").on_press(Message::Encode),
            combo_box(&self.bit, "bit", self.selected_bit.as_ref(),Message::SelectedBit)
            ,
            text_input("ASCII", &self.ascii_string).on_input(Message::AsciiChanged),
            text_input("HEX", &self.hex_string).on_input(Message::HexChanged),
            button("Decode").on_press(Message::Decode),
        ].width(Fill).spacing(10)
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
