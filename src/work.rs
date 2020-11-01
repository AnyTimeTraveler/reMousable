use websocket::{ClientBuilder, OwnedMessage};
use enigo::{Enigo, MouseButton, MouseControllable};
use iced::futures;

// Just a little utility function
pub fn connect<T: ToString>(url: T) -> iced::Subscription<Progress> {
    iced::Subscription::from_recipe(Worker {
        url: url.to_string(),
    })
}

pub struct Worker {
    url: String,
}


impl<H, I> iced_native::subscription::Recipe<H, I> for Worker
    where
        H: std::hash::Hasher,
{
    type Output = String;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.url.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            State::Ready(self.url),
            |state| async move {
                match state {
                    State::Ready(url) => {
                        let response = reqwest::get(&url).await;

                        match response {
                            Ok(response) => {
                                if let Some(total) = response.content_length() {
                                    Some((
                                        Progress::Started,
                                        State::Downloading {
                                            response,
                                            total,
                                            downloaded: 0,
                                        },
                                    ))
                                } else {
                                    Some((Progress::Errored, State::Finished))
                                }
                            }
                            Err(_) => {
                                Some((Progress::Errored, State::Finished))
                            }
                        }
                    }
                    State::Downloading {
                        mut response,
                        total,
                        downloaded,
                    } => match response.chunk().await {
                        Ok(Some(chunk)) => {
                            let downloaded = downloaded + chunk.len() as u64;

                            let percentage =
                                (downloaded as f32 / total as f32) * 100.0;

                            Some((
                                Progress::Advanced(percentage),
                                State::Downloading {
                                    response,
                                    total,
                                    downloaded,
                                },
                            ))
                        }
                        Ok(None) => Some((Progress::Finished, State::Finished)),
                        Err(_) => Some((Progress::Errored, State::Finished)),
                    },
                    State::Finished => {
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
pub enum Progress {
    Started,
    Advanced(f32),
    Finished,
    Errored,
}

pub enum State {
    Ready,
    Connected,
    Disconnected,
}



pub fn start_client(address: String) {
    let mut client = ClientBuilder::new(address.as_str()).unwrap()
        .connect_insecure().unwrap();

    let mut enigo = Enigo::new();

    let mut is_down = false;

    while let Ok(packet) = client.recv_message() {
        println!("Incomming: {:?}", packet.clone());
        if let OwnedMessage::Text(text) = packet {
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
}
