use confy::ConfyError;
use iced::{Application, Settings};
use serde_derive::{Deserialize, Serialize};
use gui::Gui;

mod gui;
mod work;

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
    Gui::run(settings);
}
