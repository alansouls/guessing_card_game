use bevy::{
    ecs::component::Component,
    time::Timer,
};

use super::MatchState;

#[derive(Component)]
pub struct CurrentPlayer(pub usize);

#[derive(Component)]
pub struct PlayerInfo{
    pub player_id: usize,
    pub card_count: usize,
    pub guess: usize,
    pub wins: usize,
}

#[derive(Component)]
pub struct Card {
    pub player_id: Option<usize>,
    pub card: card_game_logic::game_logic::common::Card,
}

#[derive(Component)]
pub struct Guess(pub usize);

#[derive(Component)]
pub struct MaxGuess(pub usize);

#[derive(Component)]
pub struct TopPlayedCard;

#[derive(Component)]
pub struct DisplayPlayedCardTimer {
    pub timer: Timer,
    pub match_state: MatchState,
    pub next_player_id: usize,
}
