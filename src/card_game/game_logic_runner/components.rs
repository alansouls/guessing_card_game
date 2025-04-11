use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CurrentPlayer(pub usize);

#[derive(Component)]
pub struct Card {
    pub player_id: Option<usize>,
    pub card: super::super::game_logic::common::Card,
}

#[derive(Component)]
pub struct Guess(pub usize);

#[derive(Component)]
pub struct MaxGuess(pub usize);

#[derive(Component)]
pub struct TopPlayedCard;
