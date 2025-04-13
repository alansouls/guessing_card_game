use bevy::{ecs::{event::Event, system::Resource}, time::Timer};

use crate::card_game::game_logic::common::Card;

#[derive(Event)]
pub struct GameEnded {
    pub winner: usize,
}

#[derive(Event)]
pub struct PlayerGuessed {
    pub player_id: usize,
    pub guess: usize,
}

#[derive(Event)]
pub struct CardPlayed {
    pub player_id: usize,
    pub card: Card,
}