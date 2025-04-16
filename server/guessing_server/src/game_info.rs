use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use card_game_logic::game_logic::{local::LocalGameLogic, GameSettings};

use futures::lock::Mutex;

pub struct PlayerInfo {
    pub player_id: usize,
    pub player_name: String,
    pub player_ip: SocketAddr,
}

pub struct GameInfo {
    pub game_settings: GameSettings,
    pub game_logic: LocalGameLogic,
    pub player_info_map: HashMap<usize, PlayerInfo>,
}

pub struct GameInfoLookup {
    game_infos: Vec<Arc<Mutex<GameInfo>>>,
    game_info_by_name: HashMap<String, Arc<Mutex<GameInfo>>>,
    game_info_by_socket_addr: HashMap<SocketAddr, Arc<Mutex<GameInfo>>>,
}

impl GameInfoLookup {
    pub fn new() -> Self {
        GameInfoLookup {
            game_infos: Vec::new(),
            game_info_by_name: HashMap::new(),
            game_info_by_socket_addr: HashMap::new(),
        }
    }

    pub fn create_game_info(&mut self, name: String, player_name: String, host_socket_addr: SocketAddr) -> Result<Arc<Mutex<GameInfo>>, String> {
        if self.game_info_by_socket_addr.contains_key(&host_socket_addr) {
            return Err(format!("Game already exists at socket address: {}", host_socket_addr));
        }
        if self.game_info_by_name.contains_key(&name) {
            return Err(format!("Game already exists with name: {}", name));
        }
        let mut game_info = GameInfo {
            game_settings: GameSettings::default(),
            game_logic: LocalGameLogic::default(),
            player_info_map: HashMap::new(),
        };

        game_info.player_info_map.insert(0, PlayerInfo {
            player_id: 0,
            player_name,
            player_ip: host_socket_addr,
        });
        
        let game_info = Arc::new(Mutex::new(game_info));
        self.game_infos.push(game_info.clone());
        self.game_info_by_name.insert(name.clone(), game_info.clone());
        self.game_info_by_socket_addr.insert(host_socket_addr, game_info.clone());

        Ok(game_info)
    }

    pub fn get_game_info_by_name(&self, name: &str) -> Option<Arc<Mutex<GameInfo>>> {
        self.game_info_by_name.get(name).cloned()
    }

    pub fn get_game_info_by_socket_addr(&self, socket_addr: &SocketAddr) -> Option<Arc<Mutex<GameInfo>>> {
        self.game_info_by_socket_addr.get(socket_addr).cloned()
    }
}