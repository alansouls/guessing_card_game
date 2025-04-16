use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    ops::DerefMut,
    str::FromStr,
    sync::Arc,
    thread,
};

use futures::{executor::block_on, lock::Mutex};
use card_game_logic::{game_logic::local::LocalGameLogic, game_message::{GameMessage, MessageType}};

pub struct PlayerInfo {
    pub player_id: usize,
    pub player_name: String,
    pub player_ip: SocketAddr,
}

fn main() -> std::io::Result<()> {
    let game_logic = LocalGameLogic::default();
    let game_logic = Arc::new(Mutex::new(LocalGameLogic));
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:34254")?);

    let map: HashMap<usize, PlayerInfo> = HashMap::new();
    let map = Mutex::new(map);
    let map = Arc::new(map);
    loop {
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;

        let map_clone = map.clone();
        let socket_clone = socket.clone();
        thread::spawn(move || {
            let message_string = String::from_utf8_lossy(&buf[..amt]);
            match GameMessage::from_str(&message_string) {
                Ok(message) => {
                    handle_message(socket_clone.as_ref(), &src, &map_clone, message);
                }
                Err(_) => (),
            }
        });
    }
}

fn handle_message(
    socket: &UdpSocket,
    response_address: &SocketAddr,
    ip_map: &Mutex<HashMap<usize, PlayerInfo>>,
    message: GameMessage,
) {
    let result: Result<(), String> = match message.message_type {
        MessageType::PlayerJoin => {
            add_player(message, response_address, ip_map).map(|player_id| {
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
    ip_map: &Mutex<HashMap<usize, PlayerInfo>>,
) -> Result<usize, String> {
    match message
        .message_params
        .iter()
        .find(|param| param.key == "name")
    {
        Some(param) => {
            let mut map_lock = block_on(ip_map.lock());
            add_player_with_name(&param.value, response_address, map_lock.deref_mut())
        }
        None => Err("You must provide a player name".to_string()),
    }
}

fn add_player_with_name(
    player_name: &String,
    player_address: &SocketAddr,
    ip_map: &mut HashMap<usize, PlayerInfo>,
) -> Result<usize, String> {
    if ip_map
        .iter()
        .find(|(_, player_info)| {
            player_info.player_ip == *player_address || player_info.player_name == *player_name
        })
        .is_some()
    {
        return Err("Player already exists".to_string());
    }

    let player_id = ip_map.len();
    let player_info = PlayerInfo {
        player_id,
        player_name: player_name.clone(),
        player_ip: *player_address,
    };

    ip_map.insert(player_id, player_info);

    Ok(player_id)
}
