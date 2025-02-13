use ascii_encoder::{AsciiEncoder, AsciiEncoderType};
use eight_encoder::EightEncoder;
use seven_encoder::SevenEncoder;
use six_encoder::SixEncoder;
use iced::widget::{button, text_input,column,combo_box,container,Text,row};
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
    result_message: String,

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
            result_message: String::new(),
        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Encode => {
                if !self.selected_bit.is_none() {
                    let bit = self.selected_num_bits.unwrap().get_num();
                    let encoder = self.selected_bit.unwrap().get_encode(bit);
                    let _result = encoder.encode(&self.ascii_string);
                    dbg!(&_result);
                    match _result {
                        ascii_encoder::AsciiResult::OK(hex) => {
                            self.hex_string = hex;
                            self.result_message = String::from("OK");
                        }
                        ascii_encoder::AsciiResult::OKAdded(hex) => {
                            self.hex_string = hex;
                            self.result_message = String::from("Added padding");
                        }
                        ascii_encoder::AsciiResult::OKRemoved(hex) => {
                            self.hex_string = hex;
                            self.result_message = String::from("Removed text");
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
                    dbg!(&_result);
                    match _result {
                        ascii_encoder::AsciiResult::OK(hex) => {
                            self.ascii_string = hex;
                            self.result_message = String::from("OK");
                        }
                        ascii_encoder::AsciiResult::OKAdded(hex) => {
                            self.ascii_string = hex;
                            self.result_message = String::from("Added space");
                        }
                        ascii_encoder::AsciiResult::OKRemoved(hex) => {
                            self.ascii_string = hex;
                            self.result_message = String::from("Removed text");
                        }
                        ascii_encoder::AsciiResult::OKEnded(hex) => {
                            self.ascii_string = hex;
                            self.result_message = String::from("Text Ended ");
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
        let mut encode_button = button("Encode");
        if !self.ascii_string.is_empty() {
            encode_button =  encode_button.on_press(Message::Encode);
        } 
        let mut decode_button = button("Decode");
        if !self.hex_string.is_empty() {
            decode_button = decode_button.on_press(Message::Decode);
        }
        container(
            column![
                Text::new("EPC Manager").size(30),
                row![
                    Text::new("EPC size"),
                    combo_box(&self.num_bits, "num_bits", self.selected_num_bits.as_ref(),Message::SelectedNumBits) ,
                    ].spacing(10),
                row![
                    Text::new("Eoncoder"),
                    combo_box(&self.bit, "bit", self.selected_bit.as_ref(),Message::SelectedBit),
                    ].spacing(10),
                text_input("ASCII Text", &self.ascii_string).on_input(Message::AsciiChanged),
                text_input("HEX Text", &self.hex_string).on_input(Message::HexChanged),
                row![encode_button, decode_button].spacing(10),
                Text::new(&self.result_message)
                
            ].spacing(10)
        ).padding(10).height(300).width(500)

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
