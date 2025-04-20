use bevy::ecs::event::Event;

use card_game_logic::game_logic::common::Card;

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

#[derive(Event)]
pub struct PlayerInfoUpdated;
