use common::{Card, CardPlayedResult, PlayedCard};

pub mod common;
pub mod local;
pub mod online;

pub struct GameSettings {
    pub player_count: usize,
    pub initial_card_count: usize,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            player_count: 3,
            initial_card_count: 3,
        }
    }
}

pub trait GameLogic {
    fn start_match(&mut self, inital_card_count: usize) -> CardPlayedResult;
    fn set_guess(&mut self, player_id: usize, guess: usize) -> Result<(), String>;
    fn play_card(&mut self, player_id: usize, card: &Card) -> Result<CardPlayedResult, String>;
    fn get_player_cards(&self, player_id: usize) -> &Vec<Card>;
    fn get_player_card_count(&self, player_id: usize) -> usize;
    fn get_player_turn(&self) -> usize;
    fn get_player_guess(&self, player_id: usize) -> usize;
    fn get_player_wins(&self, player_id: usize) -> usize;
    fn get_winner(&self) -> usize;
    fn get_game_over(&self) -> bool;
    fn get_played_cards(&self) -> &Vec<PlayedCard>;
    fn get_guessing_round(&self) -> bool;
    fn get_player_count(&self) -> usize;
}
