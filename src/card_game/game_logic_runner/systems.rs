use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        system::{Commands, Res, ResMut, Single},
    },
    state::{commands, state::NextState},
};

use crate::card_game::{GameSettings, LocalGameLogicRes, game_logic::GameLogic};

use super::{
    components::{self, CurrentPlayer}, events::{CardPlayed, GameEnded, PlayerGuessed}, MatchState
};

pub fn handle_game_start(
    mut game_logic: ResMut<LocalGameLogicRes>,
    game_settings: Res<GameSettings>,
    current_player: Option<Single<&mut CurrentPlayer>>,
    match_state: ResMut<NextState<MatchState>>,
) {
    game_logic
        .0
        .init(game_settings.player_count, game_settings.inital_card_count);

    update_current_player(game_logic, current_player, match_state);
}

pub fn handle_player_guess(
    mut game_logic: ResMut<LocalGameLogicRes>,
    mut event: EventReader<PlayerGuessed>,
    current_player: Option<Single<&mut CurrentPlayer>>,
    match_state: ResMut<NextState<MatchState>>,
) {
    for event in event.read() {
        game_logic.0.set_guess(event.player_id, event.guess);
    }

    update_current_player(game_logic, current_player, match_state);
}

pub fn handle_card_played(
    mut game_logic: ResMut<LocalGameLogicRes>,
    mut event: EventReader<CardPlayed>,
    current_player: Option<Single<&mut CurrentPlayer>>,
    mut game_ended_writer: EventWriter<GameEnded>,
    match_state: ResMut<NextState<MatchState>>,
) {
    for event in event.read() {
        game_logic.0.play_card(event.player_id, event.card_index);
    }

    if game_logic.0.game_over {
        game_ended_writer.send(GameEnded {
            winner: game_logic.0.get_winner(),
        });
    }

    update_current_player(game_logic, current_player, match_state);
}

fn update_current_player(
    game_logic: ResMut<'_, LocalGameLogicRes>,
    current_player: Option<Single<'_, &mut CurrentPlayer>>,
    mut match_state: ResMut<NextState<MatchState>>,
) {
    match current_player {
        Some(mut current_player) => {
            current_player.0 = game_logic.0.get_player_turn();
        }
        None => {}
    };

    if game_logic.0.guessing_round {
        match_state.set(MatchState::Guessing);
    } else if game_logic.0.game_over {
        match_state.set(MatchState::Finished);
    } else {
        match_state.set(MatchState::Playing);
    }
}

pub fn spawn_cards(
    mut commands: Commands,
    game_logic: Res<LocalGameLogicRes>,
) {
    for player_id in 0..game_logic.0.player_card_count.len() {
        let cards = game_logic.0.get_player_cards(player_id as usize);
        for card in cards {
            println!("Spawning card: {:?}", card);
            commands.spawn(components::Card{
                player_id: Some(player_id as usize),
                card: card.clone(),
            });
        }
    }
}
