use enigo::*;
use websocket::ClientBuilder;
use websocket::OwnedMessage;

const TABLET_MAX_X: i64 = 20966;
const TABLET_MAX_Y: i64 = 15725;

const SCREEN_MAX_X: i64 = 1920;
const SCREEN_MAX_Y: i64 = 1080;

const RATIO_X: i64 = TABLET_MAX_X / SCREEN_MAX_X;
const RATIO_Y: i64 = TABLET_MAX_Y / SCREEN_MAX_Y;

fn main() {
    let mut client = ClientBuilder::new("ws://192.168.1.41:55555").unwrap()
        .connect_insecure().unwrap();

    let mut enigo = Enigo::new();

    let mut is_down = false;

    while let Ok(packet) = client.recv_message() {
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
