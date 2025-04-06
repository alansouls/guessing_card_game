use bevy::prelude::Component;

#[derive(Component)]
pub struct Deck { pub cards: Vec<Card> }

#[derive(Component)]
pub struct GameState { pub player_turn: i8, pub player_card_count: Vec<usize>, pub game_over: bool }

#[derive(Component)]
pub struct Player { pub player_id: i8, pub name: String }

#[derive(Component)]
pub struct PlayerHand { pub cards: Vec<Card> }

#[derive(Component)]
pub struct PlayerCount(pub i8);

#[derive(Component)]
pub struct CardsPlayed { pub cards: Vec<Card> }