use iced::*;

use crate::{start_client, work};

pub struct Gui {
    address_field: text_input::State,
    connect_button: button::State,
    status: String,
    address: String,
    do_connect: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ConnectPressed,
    StatusChange(String),
    TextChange,
}

impl Application for Gui {
    // type Executor = executor::Null;
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = String;

    fn new(address: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                address_field: Default::default(),
                connect_button: Default::default(),
                status: "Ready to connect".to_string(),
                address,
                do_connect: false,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        format!("reMousable - {}", self.status)
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.do_connect {
            work::connect(self.address.clone()).map(Message::StatusChange)
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ConnectPressed => {
                self.do_connect = true;
                self.status = "Connecting...".to_string();
            }
            Message::StatusChange(status) => {
                self.status = status;
            }
            Message::TextChange => {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout
        Container::new(
            Column::with_children(
                vec![
                    Text::new("Address:").into(),
                    TextInput::new(&mut self.address_field, "Address", &mut self.address, |_| { Message::TextChange }).into(),
                    Button::new(&mut self.connect_button, Text::new("Connect"))
                        .on_press(Message::ConnectPressed).into(),
                    Row::with_children(
                        vec![
                            Text::new("Status:").into(),
                            Text::new(self.status.clone()).into()
                        ]
                    ).into()
                ]
            )
        )
            .width(Length::Shrink)
            .height(Length::Shrink)
            .center_x()
            .center_y()
            .into()
    }
}
