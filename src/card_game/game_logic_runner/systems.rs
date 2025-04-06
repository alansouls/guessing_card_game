use bevy::ecs::{event::EventReader, system::{Commands, Res, ResMut}};

use crate::card_game::{game_logic::GameLogic, GameSettings, LocalGameLogicRes};

use super::events::GameStarted;


pub fn handle_game_start(
    mut game_logic: ResMut<LocalGameLogicRes>,
    game_settings: Res<GameSettings>,
    mut game_started_events: EventReader<GameStarted>,
) {
    for _ in game_started_events.read() {
        game_logic.0.init(game_settings.player_count, game_settings.inital_card_count);
    }
}