use bevy::ecs::component::Component;

use crate::card_game::game_logic;

#[derive(Component)]
pub struct CurrentPlayer(pub usize);

#[derive(Component)]
pub struct Card{
    pub player_id: Option<usize>,
    pub card: game_logic::common::Card
}

#[derive(Component)]
pub struct Guess(pub usize);