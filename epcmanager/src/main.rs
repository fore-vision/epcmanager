use iced::widget::{button, text,column,combo_box};
use iced::Element;
use iced::Fill;

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
}

impl std::fmt::Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
}


struct EpcManager {
    epcs: Vec<String>,
    bit: combo_box::State<Bit>,
    selected_bit: Option<Bit>,

}

#[derive(Debug, Clone,Copy)]
enum Message {
    Encode,
    Decode,
    SelectedBit(Bit),
}

impl EpcManager {
    fn new() -> Self {
        Self {
            epcs: Vec::new(),
            bit: combo_box::State::new(Bit::ALL.to_vec()),
            selected_bit: Bit::Eight.into(),
        }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Encode => {
                self.epcs.push("Hello".to_string());
            }
            Message::Decode => {
                self.epcs.pop();
            }
            Message::SelectedBit(bit) => {
                self.selected_bit = Some(bit);
            }
        
        }
    }
    fn view(&self) -> iced::Element<Message> {

        column![
            button("Encode").on_press(Message::Encode),
            combo_box(&self.bit, "bit", self.selected_bit.as_ref(),Message::SelectedBit)
            ,
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
