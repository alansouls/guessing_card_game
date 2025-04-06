use bevy::ecs::event::Event;

#[derive(Event)]
pub struct GameStarted;

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
    pub card_index: usize,
}