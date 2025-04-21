use card_game_logic::game_logic::{
    GameLogic, common::PlayedCard, local::LocalGameLogic, online::OnlineGameLogic,
};

pub struct GameLogicFacade {
    local_game_logic: Option<LocalGameLogic>,
    online_game_logic: Option<OnlineGameLogic>,
}

impl GameLogicFacade {
    pub fn new() -> Self {
        GameLogicFacade {
            local_game_logic: None,
            online_game_logic: None,
        }
    }

    pub fn init_local(&mut self, player_count: usize) -> &GameLogicFacade {
        let mut game_logic = LocalGameLogic::default();
        game_logic.init(player_count);
        self.local_game_logic = Some(game_logic);
        self.online_game_logic = None;

        self
    }

    pub fn init_online(&mut self, player_name: &String, room_name: &String) -> &GameLogicFacade {
        let mut game_logic = OnlineGameLogic::new();

        match game_logic.join(player_name.clone(), room_name.clone()) {
            Ok(_) => (),
            Err(err) => {
                panic!("Failed to join online game: {}", err);
            }
        }

        self.local_game_logic = None;
        self.online_game_logic = Some(OnlineGameLogic::new());
        self
    }
}

fn panic_not_initialized() -> ! {
    panic!("GameLogicFacade not initialized properly")
}

impl GameLogic for GameLogicFacade {
    fn start_match(
        &mut self,
        inital_card_count: usize,
    ) -> card_game_logic::game_logic::common::CardPlayedResult {
        match self.local_game_logic {
            Some(ref mut game_logic) => {
                return game_logic.start_match(inital_card_count);
            }
            None => (),
        }

        panic_not_initialized()
    }

    fn set_guess(&mut self, player_id: usize, guess: usize) -> Result<(), String> {
        match self.local_game_logic {
            Some(ref mut game_logic) => {
                return game_logic.set_guess(player_id, guess);
            }
            None => (),
        }

        panic_not_initialized();
    }

    fn play_card(
        &mut self,
        player_id: usize,
        card: &card_game_logic::game_logic::common::Card,
    ) -> Result<card_game_logic::game_logic::common::CardPlayedResult, String> {
        match self.local_game_logic {
            Some(ref mut game_logic) => {
                return game_logic.play_card(player_id, card);
            }
            None => (),
        }

        panic_not_initialized();
    }

    fn get_player_cards(
        &self,
        player_id: usize,
    ) -> &Vec<card_game_logic::game_logic::common::Card> {
        match self.local_game_logic {
            Some(ref game_logic) => {
                return game_logic.get_player_cards(player_id);
            }
            None => (),
        }

        panic_not_initialized()
    }

    fn get_player_card_count(&self, player_id: usize) -> usize {
        match self.local_game_logic {
            Some(ref game_logic) => {
                return game_logic.get_player_card_count(player_id);
            }
            None => (),
        }

        panic_not_initialized()
    }

    fn get_player_turn(&self) -> usize {
        match self.local_game_logic {
            Some(ref game_logic) => {
                return game_logic.get_player_turn();
            }
            None => (),
        }

        panic_not_initialized()
    }

    fn get_player_guess(&self, player_id: usize) -> usize {
        match self.local_game_logic {
            Some(ref game_logic) => {
                return game_logic.get_player_guess(player_id);
            }
            None => (),
        }

        panic_not_initialized()
    }

    fn get_player_wins(&self, player_id: usize) -> usize {
        match self.local_game_logic {
            Some(ref game_logic) => {
                return game_logic.get_player_wins(player_id);
            }
            None => (),
        }

        panic_not_initialized()
    }

    fn get_winner(&self) -> usize {
        match self.local_game_logic {
            Some(ref game_logic) => {
                return game_logic.get_winner();
            }
            None => (),
        }

        panic_not_initialized()
    }

    fn get_game_over(&self) -> bool {
        match self.local_game_logic {
            Some(ref game_logic) => {
                return game_logic.get_game_over();
            }
            None => (),
        }

        panic_not_initialized()
    }

    fn get_played_cards(&self) -> &Vec<PlayedCard> {
        match self.local_game_logic {
            Some(ref game_logic) => {
                return game_logic.get_played_cards();
            }
            None => (),
        }

        panic_not_initialized()
    }

    fn get_guessing_round(&self) -> bool {
        match self.local_game_logic {
            Some(ref game_logic) => {
                return game_logic.get_guessing_round();
            }
            None => (),
        }

        panic_not_initialized()
    }

    fn get_player_card_counts(&self) -> &Vec<usize> {
        match self.local_game_logic {
            Some(ref game_logic) => {
                return &game_logic.get_player_card_counts();
            }
            None => (),
        }

        panic_not_initialized()
    }
}
