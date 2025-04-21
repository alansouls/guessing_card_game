use std::{net::UdpSocket, str::FromStr, sync::Arc, thread};

use card_game_logic::{game_message::GameMessage, server_options};
use futures::lock::Mutex;
use game_info::GameInfoLookup;

pub mod game_info;
pub mod message_handler;

fn main() -> std::io::Result<()> {
    let game_info_lookup = GameInfoLookup::new();
    let game_info_lookup = Arc::new(Mutex::new(game_info_lookup));

    let socket =
        Arc::new(UdpSocket::bind(server_options::get_server_bind_addr()).expect("Failed to bind"));
    loop {
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;

        let lookup_clone = game_info_lookup.clone();
        let socket_clone = socket.clone();
        thread::spawn(move || {
            println!("Received message from {:?}", src);
            let message_string = String::from_utf8_lossy(&buf[..amt]);
            match GameMessage::from_str(&message_string) {
                Ok(message) => {
                    message_handler::handle_message(
                        socket_clone.as_ref(),
                        &src,
                        &lookup_clone,
                        message,
                    );
                }
                Err(_) => (),
            }
        });
    }
}
