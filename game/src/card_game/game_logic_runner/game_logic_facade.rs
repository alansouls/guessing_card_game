use card_game_logic::game_logic::{GameLogic, local::LocalGameLogic};

pub struct GameLogicFacade {
    local_game_logic: Option<LocalGameLogic>,
}

fn new_local() -> GameLogicFacade {
    GameLogicFacade {
        local_game_logic: Some(LocalGameLogic::default()),
    }
}

fn panic_not_initialized() -> ! {
    panic!("GameLogicFacade not initialized properly")
}

impl GameLogic for GameLogicFacade {
    fn init(&mut self, player_count: usize, initial_card_count: usize) {
        match self.local_game_logic {
            Some(ref mut game_logic) => {
                return game_logic.init(player_count, initial_card_count);
            }
            None => (),
        }

        panic_not_initialized();
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
}
