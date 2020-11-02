use websocket::{ClientBuilder, OwnedMessage};
use enigo::{Enigo, MouseButton, MouseControllable};
use iced::futures;
use websocket::client::sync::Client;
use websocket::websocket_base::stream::sync::TcpStream;

pub fn connect<T: ToString>(url: T) -> iced::Subscription<State> {
    iced::Subscription::from_recipe(Worker {
        url: url.to_string(),
    })
}

pub struct Worker {
    url: String,
}

const TABLET_MAX_X: i64 = 20966;
const TABLET_MAX_Y: i64 = 15725;

const SCREEN_MAX_X: i64 = 1920;
const SCREEN_MAX_Y: i64 = 1080;

const RATIO_X: i64 = TABLET_MAX_X / SCREEN_MAX_X;
const RATIO_Y: i64 = TABLET_MAX_Y / SCREEN_MAX_Y;


impl<H, I> iced_futures::subscription::Recipe<H, I> for Worker
    where
        H: std::hash::Hasher,
{
    type Output = State;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.url.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(
            futures::stream::unfold(
            State::Ready(self.url),
            |state| async move {
                match state {
                    State::Ready(address) => {
                        let client_builder = ClientBuilder::new(address.as_str());
                        if let Err(error) = client_builder {
                            return Some(State::Error(error.to_string()));
                        }

                        Some(match client_builder.unwrap().connect_insecure(){
                            Ok(client) => State::Connected(client),
                            Err(error) => State::Error(error.to_string())
                        })
                    },
                    State::Connected(mut client) => {
                        let mut enigo = Enigo::new();
                        let mut is_down = false;


                        loop {
                            let packet = client.recv_message();
                            if let Err(error) = packet {
                                return Some(State::Disconnected(error.to_string()));
                            }

                            if let OwnedMessage::Text(text) = packet.unwrap() {
                                let dots: [i64; 3] = serde_json::from_str(text.as_str()).unwrap();
                                if !is_down && dots[2] > 0 {
                                    enigo.mouse_down(MouseButton::Left);
                                    is_down = true;
                                } else if is_down && dots[2] == 0 {
                                    enigo.mouse_up(MouseButton::Left);
                                    is_down = false;
                                }
                                enigo.mouse_move_to((dots[0] / RATIO_X) as i32, (dots[1] / RATIO_Y) as i32);
                            }
                        }
                    },
                    State::Disconnected(_) | State::Error(_) => {
                        // We do not let the stream die, as it would start a
                        // new download repeatedly if the user is not careful
                        // in case of errors.
                        let _: () = iced::futures::future::pending().await;

                        None
                    }
                }
            },
        ))
    }
}


#[derive(Debug, Clone)]
pub enum State {
    Ready(String),
    Connected(Client<TcpStream>),
    Disconnected(String),
    Error(String),
}
