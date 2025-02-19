use ascii_encoder::{AsciiEncoder, AsciiEncoderType, StringResult};
use eight_encoder::EightEncoder;
use seven_encoder::SevenEncoder;
use six_encoder::SixEncoder;
use iced::widget::{button, text_input,column,combo_box,container,Text,row};
mod ascii_encoder;
mod eight_encoder;
mod seven_encoder;
mod six_encoder;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default,Clone,Copy, PartialEq, Eq,Debug)]
enum BitEncoder {
    Eight,
    #[default]
    Seven,
    Six,
}
impl BitEncoder {
    const ALL: [BitEncoder; 3] = [BitEncoder::Eight, BitEncoder::Seven, BitEncoder::Six];
    fn str(&self) -> &str {
        match self {
            BitEncoder::Eight => "8",
            BitEncoder::Seven => "7",
            BitEncoder::Six => "6",
        }
    }
    fn get_encode(&self,bit: usize) -> AsciiEncoderType {
        match self {
            BitEncoder::Eight => AsciiEncoderType::Eight(EightEncoder::new(bit)),
            BitEncoder::Seven => AsciiEncoderType::Seven(SevenEncoder::new(bit)),
            BitEncoder::Six => AsciiEncoderType::Six(SixEncoder::new(bit)),
        }

    }
}


impl std::fmt::Display for BitEncoder {
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
    bit_encoder: combo_box::State<BitEncoder>,
    selected_bit_encoder: Option<BitEncoder>,
    num_bits: combo_box::State<NumBits>,
    selected_num_bits: Option<NumBits>,
    result_message: String,
    encoder: AsciiEncoderType,
    ascii_message: String,
    hex_message: String,
    ascii_result: StringResult,
    hex_result: StringResult,
}

#[derive(Debug, Clone)]
enum Message {
    Encode,
    Decode,
    SelectedBitEncoder(BitEncoder),
    SelectedNumBits(NumBits),
    AsciiChanged(String),
    HexChanged(String),
}

impl EpcManager {

    fn new() -> Self {
        Self {
            ascii_string: String::new(),
            hex_string: String::new(),
            bit_encoder: combo_box::State::new(BitEncoder::ALL.to_vec()),
            selected_bit_encoder: Some( BitEncoder::Eight),
            num_bits: combo_box::State::new(NumBits::ALL.to_vec()),
            selected_num_bits: Some( NumBits::Bit96),
            result_message: String::new(),
            encoder: AsciiEncoderType::Eight(EightEncoder::new(96)),
            ascii_result: StringResult::EmptyString,
            hex_result: StringResult::EmptyString,
            ascii_message: StringResult::EmptyString.get_message(),
            hex_message: StringResult::EmptyString.get_message(),

        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Encode => {
                let _result = self.encoder.encode(&self.ascii_string);
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
                self.hex_result = self.encoder.check_hex(&self.hex_string);
                self.hex_message = self.hex_result.get_message();

            }
            Message::Decode => {
                let _result = self.encoder.decode(&self.hex_string);
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
                self.ascii_result = self.encoder.check_ascii(&self.ascii_string);
                self.ascii_message = self.ascii_result.get_message();

            }
            Message::SelectedBitEncoder(bit_encoder) => {
                self.selected_bit_encoder = Some(bit_encoder);
                self.encoder = bit_encoder.get_encode(self.selected_num_bits.unwrap().get_num());
                self.ascii_result = self.encoder.check_ascii(&self.ascii_string);
                self.hex_result = self.encoder.check_hex(&self.hex_string);
                self.ascii_message = self.ascii_result.get_message();
                self.hex_message = self.hex_result.get_message();
            }
            Message::AsciiChanged(ascii) => {
                self.ascii_string = ascii;
                self.ascii_result = self.encoder.check_ascii(&self.ascii_string);
                self.hex_result = self.encoder.check_hex(&self.hex_string);
                self.ascii_message = self.ascii_result.get_message();
                self.hex_message = self.hex_result.get_message();
            }
            Message::HexChanged(hex) => {
                self.hex_string = hex;
                self.ascii_result = self.encoder.check_ascii(&self.ascii_string);
                self.hex_result = self.encoder.check_hex(&self.hex_string);
                self.ascii_message = self.ascii_result.get_message();
                self.hex_message = self.hex_result.get_message();
            }
            Message::SelectedNumBits(num_bits) => {
                self.selected_num_bits = Some(num_bits);
                self.encoder = self.selected_bit_encoder.unwrap().get_encode(num_bits.get_num());
                self.ascii_result = self.encoder.check_ascii(&self.ascii_string);
                self.hex_result = self.encoder.check_hex(&self.hex_string);
                self.ascii_message = self.ascii_result.get_message();
                self.hex_message = self.hex_result.get_message();
            }
        
        }
    }
    fn view(&self) -> iced::Element<Message> {
        let mut encode_button = button("Encode");
        if self.ascii_result.is_ok() {
            encode_button =  encode_button.on_press(Message::Encode);
        } 
        let mut decode_button = button("Decode");
        if self.hex_result.is_ok() {
            decode_button = decode_button.on_press(Message::Decode);
        }
        container(
            column![
                row![
                    Text::new("EPC Manager Version: ").size(20),
                    Text::new(VERSION).size(20),
                ].spacing(10),
                row![
                    Text::new("EPC size"),
                    combo_box(&self.num_bits, "num_bits", self.selected_num_bits.as_ref(),Message::SelectedNumBits) ,
                    ].spacing(10),
                row![
                    Text::new("Eoncoder"),
                    combo_box(&self.bit_encoder, "bit", self.selected_bit_encoder.as_ref(),Message::SelectedBitEncoder),
                    ].spacing(10),
                row![
                    Text::new("ASCII Text"),
                    text_input("ASCII Text", &self.ascii_string).on_input(Message::AsciiChanged),
                    Text::new(&self.ascii_message),
                ].spacing(10),
                row![
                    Text::new("HEX Text"),
                    text_input("HEX Text", &self.hex_string).on_input(Message::HexChanged),
                    Text::new(&self.hex_message),
                ].spacing(10),
                row![encode_button, decode_button].spacing(10),
                row![
                    Text::new("Result: "),
                    Text::new(&self.result_message)

                ].spacing(10)
                
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
