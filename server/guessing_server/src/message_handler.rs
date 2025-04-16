use std::net::{SocketAddr, UdpSocket};

use card_game_logic::game_message::{GameMessage, MessageType};
use futures::lock::Mutex;

use crate::game_info::GameInfoLookup;

mod add_player_handler;

pub fn handle_message(
    socket: &UdpSocket,
    response_address: &SocketAddr,
    game_info_lookup: &Mutex<GameInfoLookup>,
    message: GameMessage,
) {
    let result: Result<(), String> = match message.message_type {
        MessageType::PlayerJoin => {
            add_player_handler::add_player(message, response_address, game_info_lookup).map(
                |player_id| {
                    let response_message = GameMessage {
                        player_id,
                        message_type: MessageType::PlayerJoined,
                        message_params: vec![],
                    };
                    let response_string = response_message.to_string();
                    let _ = socket.send_to(response_string.as_bytes(), response_address);
                },
            )
        }
        MessageType::Guess => {
            println!("Player {} guessed", message.player_id);
            Ok(())
        }
        MessageType::PlayCard => {
            println!("Player {} played a card", message.player_id);
            Ok(())
        }
        MessageType::PlayerJoined | MessageType::UpdateState => {
            Err("Server should not be getting this message".to_string())
        }
    };

    match result {
        Err(e) => {
            let _ = socket.send_to(e.as_bytes(), response_address);
        }
        _ => (),
    }
}
