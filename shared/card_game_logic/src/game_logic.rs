use common::{Card, CardPlayedResult};

pub mod common;
pub mod local;

pub struct GameSettings {
    pub player_count: usize,
    pub initial_card_count: usize,
}

pub trait GameLogic {
    fn init(&mut self, player_count: usize, initial_card_count: usize);
    fn set_guess(&mut self, player_id: usize, guess: usize) -> Result<(), String>;
    fn play_card(&mut self, player_id: usize, card: &Card) -> Result<CardPlayedResult, String>;
    fn get_player_cards(&self, player_id: usize) -> &Vec<Card>;
    fn get_player_card_count(&self, player_id: usize) -> usize;
    fn get_player_turn(&self) -> usize;
    fn get_player_guess(&self, player_id: usize) -> usize;
    fn get_player_wins(&self, player_id: usize) -> usize;
    fn get_winner(&self) -> usize;
}
