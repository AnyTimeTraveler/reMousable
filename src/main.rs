use confy::ConfyError;
use enigo::*;
use iced::{Application, Settings};
use serde_derive::{Deserialize, Serialize};
use websocket::ClientBuilder;
use websocket::OwnedMessage;

use gui::Gui;
use std::thread;

mod gui;
mod work;

const TABLET_MAX_X: i64 = 20966;
const TABLET_MAX_Y: i64 = 15725;

const SCREEN_MAX_X: i64 = 1920;
const SCREEN_MAX_Y: i64 = 1080;

const RATIO_X: i64 = TABLET_MAX_X / SCREEN_MAX_X;
const RATIO_Y: i64 = TABLET_MAX_Y / SCREEN_MAX_Y;

#[derive(Serialize, Deserialize)]
struct Config {
    address: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            address: "ws://192.168.1.48:55555".to_string()
        }
    }
}

fn main() {
    let result: Result<Config, ConfyError> = confy::load("reMousable");
    let address = if let Ok(data) = result {
        data.address
    } else {
        "ws://192.168.1.48:55555".to_string()
    };

    let mut settings = Settings::with_flags(address.to_string());
    settings.antialiasing = false;
    // Gui::run(settings).unwrap()
    let handle = thread::spawn(|| Gui::run(settings));

}
