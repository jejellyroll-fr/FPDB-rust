use iced::widget::{button, Button, Column, Text};
use iced::{Element, Sandbox, Alignment, Length};

pub struct FpdbApp {
    import_count: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ImportPressed,
}

impl Sandbox for FpdbApp {
    type Message = Message;

    fn new() -> Self {
        FpdbApp {
            import_count: 0,
        }
    }

    fn title(&self) -> String {
        String::from("FPDB Rust")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ImportPressed => {
                self.import_count += 1;
                // Here you would call your import function
            }
        }
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Button::new(Text::new("Import"))
                    .on_press(Message::ImportPressed)
            )
            .push(Text::new(format!("Imported {} times", self.import_count)))
            .into()
    }
}