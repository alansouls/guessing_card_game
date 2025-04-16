use std::{
    net::{SocketAddr, UdpSocket},
    ops::DerefMut,
    str::FromStr,
    sync::Arc,
    thread,
};

use card_game_logic::game_message::{GameMessage, MessageType};
use futures::{executor::block_on, lock::Mutex};
use game_info::{GameInfo, GameInfoLookup, PlayerInfo};

pub mod game_info;

fn main() -> std::io::Result<()> {
    let game_info_lookup = GameInfoLookup::new();
    let game_info_lookup = Arc::new(Mutex::new(game_info_lookup));

    let socket = Arc::new(UdpSocket::bind("127.0.0.1:34254")?);
    loop {
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;

        let lookup_clone = game_info_lookup.clone();
        let socket_clone = socket.clone();
        thread::spawn(move || {
            let message_string = String::from_utf8_lossy(&buf[..amt]);
            match GameMessage::from_str(&message_string) {
                Ok(message) => {
                    handle_message(socket_clone.as_ref(), &src, &lookup_clone, message);
                }
                Err(_) => (),
            }
        });
    }
}

fn handle_message(
    socket: &UdpSocket,
    response_address: &SocketAddr,
    game_info_lookup: &Mutex<GameInfoLookup>,
    message: GameMessage,
) {
    let result: Result<(), String> = match message.message_type {
        MessageType::PlayerJoin => {
            add_player(message, response_address, game_info_lookup).map(|player_id| {
                let response_message = GameMessage {
                    player_id,
                    message_type: MessageType::PlayerJoined,
                    message_params: vec![],
                };
                let response_string = response_message.to_string();
                let _ = socket.send_to(response_string.as_bytes(), response_address);
            })
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

fn add_player(
    message: GameMessage,
    response_address: &SocketAddr,
    game_info_lookup: &Mutex<GameInfoLookup>,
) -> Result<usize, String> {
    let player_name_param = message
        .message_params
        .iter()
        .find(|param| param.key == "player_name");
    let room_name_param = message
        .message_params
        .iter()
        .find(|param| param.key == "room_name");
    match (player_name_param, room_name_param) {
        (Some(player_name_param), Some(room_name_param)) => {
            let mut game_info_lookup = block_on(game_info_lookup.lock());
            add_player_with_name(
                &room_name_param.value,
                &player_name_param.value,
                response_address,
                game_info_lookup.deref_mut(),
            )
        }
        _ => Err("You must provide a player name and a room name".to_string()),
    }
}

fn add_player_with_name(
    room_name: &String,
    player_name: &String,
    player_address: &SocketAddr,
    game_info_lookup: &mut GameInfoLookup,
) -> Result<usize, String> {
    match game_info_lookup.get_game_info_by_socket_addr(player_address) {
        Some(_) => Err("Player already in game".to_string()),
        None => {
            on_player_socket_addr_valid(room_name, player_name, player_address, game_info_lookup)
        }
    }
}

fn on_player_socket_addr_valid(
    room_name: &String,
    player_name: &String,
    player_address: &SocketAddr,
    game_info_lookup: &mut GameInfoLookup,
) -> Result<usize, String> {
    match game_info_lookup.get_game_info_by_name(&room_name) {
        Some(game_info) => on_room_exists(player_name, player_address, &game_info),
        None => game_info_lookup
            .create_game_info(room_name.clone(), player_name.clone(), *player_address)
            .map(|_| 0),
    }
}

fn on_room_exists(
    player_name: &String,
    player_address: &SocketAddr,
    game_info: &Mutex<GameInfo>,
) -> Result<usize, String> {
    let mut game_info = block_on(game_info.lock());
    match game_info
        .player_info_map
        .iter()
        .find(|p| p.1.player_ip == *player_address)
    {
        Some(_) => {
            return Err("Player already in room".to_string());
        }
        None => {
            let player_id = game_info.player_info_map.len() + 1;
            game_info.player_info_map.insert(
                player_id,
                PlayerInfo {
                    player_id,
                    player_name: player_name.clone(),
                    player_ip: *player_address,
                },
            );
            return Ok(player_id);
        }
    }
}
