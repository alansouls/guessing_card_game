use std::net::{SocketAddr, UdpSocket};

use crate::{
    game_message::{GameMessage, MessageParam, MessageType},
    server_options,
};

use super::{
    GameLogic,
    common::{Card, PlayedCard},
};

pub struct OnlinePlayerInfo {
    pub player_id: usize,
    pub player_name: String,
    pub player_card_count: usize,
    pub player_wins: usize,
    pub player_guess: usize,
}

pub struct OnlineGameLogic {
    player_id: Option<usize>,
    is_host: bool,
    player_cards: Vec<Card>,
    played_cards: Vec<PlayedCard>,
    player_infos: Vec<OnlinePlayerInfo>,
    player_turn: usize,
    game_over: bool,
    udp_socket: UdpSocket,
    server_address: SocketAddr,
}

impl OnlineGameLogic {
    pub fn new() -> Self {
        let mut port = 4000;

        let udp_socket = loop {
            let udp_socket = UdpSocket::bind(format!("127.0.0.1:{}", port));
            if (udp_socket.is_ok()) {
                break udp_socket.unwrap();
            } else {
                port += 1;
            }

            if port > 5000 {
                panic!("Could not bind to any port between 4000 and 5000");
            }
        };

        OnlineGameLogic {
            player_id: None,
            is_host: false,
            player_cards: vec![],
            played_cards: vec![],
            player_infos: vec![],
            player_turn: 0,
            game_over: false,
            udp_socket: udp_socket,
            server_address: server_options::get_server_addr(),
        }
    }

    pub fn join(&mut self, player_name: String, room_name: String) -> Result<(), String> {
        let game_message = GameMessage {
            player_id: 0,
            message_type: MessageType::PlayerJoin,
            message_params: vec![
                MessageParam {
                    key: "player_name".to_string(),
                    value: player_name,
                },
                MessageParam {
                    key: "room_name".to_string(),
                    value: room_name,
                },
            ],
        };

        let message = game_message.to_string();

        let result = self
            .udp_socket
            .send_to(message.as_bytes(), self.server_address)
            .map_err(|e| e.to_string())
            .map(|_| ());

        if result.is_err() {
            return result;
        }

        let mut buf = [0 as u8; 1024];
        self.udp_socket
            .recv_from(&mut buf)
            .expect("Fail to receive message");

        println!("Received message: {}", String::from_utf8_lossy(&buf));

        result
    }
}

impl GameLogic for OnlineGameLogic {
    fn start_match(&mut self, inital_card_count: usize) -> super::common::CardPlayedResult {
        todo!()
    }

    fn set_guess(&mut self, player_id: usize, guess: usize) -> Result<(), String> {
        todo!()
    }

    fn play_card(
        &mut self,
        player_id: usize,
        card: &Card,
    ) -> Result<super::common::CardPlayedResult, String> {
        todo!()
    }

    fn get_player_cards(&self, player_id: usize) -> &Vec<Card> {
        todo!()
    }

    fn get_player_card_count(&self, player_id: usize) -> usize {
        todo!()
    }

    fn get_player_turn(&self) -> usize {
        todo!()
    }

    fn get_player_guess(&self, player_id: usize) -> usize {
        todo!()
    }

    fn get_player_wins(&self, player_id: usize) -> usize {
        todo!()
    }

    fn get_winner(&self) -> usize {
        todo!()
    }

    fn get_game_over(&self) -> bool {
        todo!()
    }

    fn get_played_cards(&self) -> &Vec<PlayedCard> {
        todo!()
    }

    fn get_guessing_round(&self) -> bool {
        todo!()
    }

    fn get_player_card_counts(&self) -> &Vec<usize> {
        todo!()
    }
}
