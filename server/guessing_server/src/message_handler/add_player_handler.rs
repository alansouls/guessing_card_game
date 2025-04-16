use std::{net::SocketAddr, ops::DerefMut};

use card_game_logic::game_message::GameMessage;
use futures::{executor::block_on, lock::Mutex};

use crate::game_info::{GameInfo, GameInfoLookup, PlayerInfo};

pub fn add_player(
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
            on_valid_params(
                &room_name_param.value,
                &player_name_param.value,
                response_address,
                game_info_lookup.deref_mut(),
            )
        }
        _ => Err("You must provide a player name and a room name".to_string()),
    }
}

fn on_valid_params(
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
