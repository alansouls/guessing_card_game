use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    game_message::{GameMessage, MessageParam, MessageType},
    server_options,
};

use super::{
    common::{Card, CardPlayedResult, PlayedCard}, GameLogic
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
    guessing_round: bool,
    game_over: bool,
    udp_socket: Arc<UdpSocket>,
    server_address: SocketAddr,
    message_queue: Arc<Mutex<Vec<GameMessage>>>,
    listener_thread: Option<thread::JoinHandle<()>>,
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

        let udp_socket = Arc::new(udp_socket);

        let mut game_logic = OnlineGameLogic {
            player_id: None,
            is_host: false,
            player_cards: vec![],
            played_cards: vec![],
            player_infos: vec![],
            player_turn: 0,
            guessing_round: false,
            game_over: false,
            udp_socket: udp_socket.clone(),
            server_address: server_options::get_server_addr(),
            message_queue: Arc::new(Mutex::new(vec![])),
            listener_thread: None,
        };

        let udp_socket_for_listener = udp_socket;

        let message_queue_for_listener = game_logic.message_queue.clone();
        game_logic.listener_thread = Some(thread::spawn(move || {
            loop {
                let mut buf = [0 as u8; 1024];

                let (size, _) = udp_socket_for_listener
                    .recv_from(&mut buf)
                    .expect("Failed to receive message");

                let message_str = String::from_utf8_lossy(&buf[..size]);
                match GameMessage::from_str(message_str.as_ref()) {
                    Ok(message) => match message_queue_for_listener.lock() {
                        Ok(mut message_queue) => {
                            message_queue.push(message);
                        }
                        Err(e) => println!("Failed to lock message queue: {}", e),
                    },
                    Err(_) => println!("Failed to parse message {}", message_str),
                };
            }
        }));

        return game_logic;
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

        result
    }
}

impl GameLogic for OnlineGameLogic {
    fn start_match(&mut self, inital_card_count: usize) -> super::common::CardPlayedResult {
        if !self.is_host {
            return CardPlayedResult::NextPlayer;
        }

        let game_message = GameMessage {
            player_id: self.player_id.unwrap(),
            message_type: MessageType::StartMatch,
            message_params: vec![MessageParam {
                key: "card_count".to_string(),
                value: inital_card_count.to_string(),
            }],
        };

        let message = game_message.to_string();

        let _ = self
            .udp_socket
            .send_to(message.as_bytes(), self.server_address);

        CardPlayedResult::WaitUpdate
    }

    fn set_guess(&mut self, player_id: usize, guess: usize) -> Result<(), String> {
        if self.player_turn != player_id {
            return Err("Not your turn".to_string());
        }

        let game_message = GameMessage {
            player_id,
            message_type: MessageType::Guess,
            message_params: vec![MessageParam {
                key: "guess".to_string(),
                value: guess.to_string(),
            }],
        };

        let message = game_message.to_string();

        let _ = self
            .udp_socket
            .send_to(message.as_bytes(), self.server_address);

        Ok(())
    }

    fn play_card(
        &mut self,
        player_id: usize,
        card: &Card,
    ) -> Result<super::common::CardPlayedResult, String> {
        if self.player_turn != player_id {
            return Err("Not your turn".to_string());
        }

        let game_message = GameMessage {
            player_id,
            message_type: MessageType::PlayCard,
            message_params: vec![MessageParam {
                key: "card_suit".to_string(),
                value: format!("{:?}", card.0),
            }, 
            MessageParam {
                key: "card_rank".to_string(),
                value: format!("{:?}", card.1),
            }],
        };

        let message = game_message.to_string();

        let _ = self
            .udp_socket
            .send_to(message.as_bytes(), self.server_address);

        Ok(CardPlayedResult::WaitUpdate)
    }

    fn get_player_cards(&self, player_id: usize) -> &Vec<Card> {
        if player_id == self.player_id.unwrap() {
            return &self.player_cards;
        }

        panic!("Should not access other players cards");
    }

    fn get_player_card_count(&self, player_id: usize) -> usize {
        let player_info = &self.player_infos[player_id];

        player_info.player_card_count
    }

    fn get_player_turn(&self) -> usize {
        self.player_turn
    }

    fn get_player_guess(&self, player_id: usize) -> usize {
        let player_info = &self.player_infos[player_id];

        player_info.player_guess
    }

    fn get_player_wins(&self, player_id: usize) -> usize {
        let player_info = &self.player_infos[player_id];

        player_info.player_wins
    }

    fn get_winner(&self) -> usize {
        todo!();
    }

    fn get_game_over(&self) -> bool {
        self.game_over
    }

    fn get_played_cards(&self) -> &Vec<PlayedCard> {
        &self.played_cards
    }

    fn get_guessing_round(&self) -> bool {
        self.guessing_round
    }

    fn get_player_count(&self) -> usize {
        self.player_infos.len()
    }
}
