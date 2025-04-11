use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        system::{Commands, Res, ResMut, Single},
    },
    state::{commands, state::NextState},
};

use crate::card_game::{game_logic::GameLogic, GameSettings, GameState, LocalGameLogicRes};

use super::{
    MatchState,
    components::{self, CurrentPlayer, MaxGuess},
    events::{CardPlayed, GameEnded, PlayerGuessed},
};

pub fn handle_game_start(
    mut commands: Commands,
    mut game_logic: ResMut<LocalGameLogicRes>,
    game_settings: Res<GameSettings>,
    mut match_state: ResMut<NextState<MatchState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_logic
        .0
        .init(game_settings.player_count, game_settings.inital_card_count);

    commands.spawn(MaxGuess(game_settings.inital_card_count));
    
    commands.spawn(CurrentPlayer(game_logic.0.get_player_turn()));
    game_state.set(GameState::LocalGame);
    match_state.set(MatchState::Guessing);
}

pub fn handle_player_guess(
    mut game_logic: ResMut<LocalGameLogicRes>,
    mut event: EventReader<PlayerGuessed>,
    mut current_player: Single<&mut CurrentPlayer>,
    mut match_state: ResMut<NextState<MatchState>>,
) {
    for event in event.read() {
        game_logic.0.set_guess(event.player_id, event.guess);
        update_current_player(&game_logic, current_player.as_mut(), match_state.as_mut());
    }
}

pub fn handle_card_played(
    mut game_logic: ResMut<LocalGameLogicRes>,
    mut event: EventReader<CardPlayed>,
    mut current_player: Single<&mut CurrentPlayer>,
    mut game_ended_writer: EventWriter<GameEnded>,
    mut match_state: ResMut<NextState<MatchState>>,
) {
    for event in event.read() {
        game_logic.0.play_card(event.player_id, event.card_index);

        if game_logic.0.game_over {
            game_ended_writer.send(GameEnded {
                winner: game_logic.0.get_winner(),
            });
        }

        update_current_player(
            &game_logic,
            current_player.as_mut(),
            match_state.as_mut(),
        );
    }
}

fn update_current_player(
    game_logic: &LocalGameLogicRes,
    current_player: &mut CurrentPlayer,
    match_state: &mut NextState<MatchState>,
) {
    let current_player_id = game_logic.0.get_player_turn();

    println!("Current player: {:?}", current_player_id);

    if current_player_id != current_player.0 {
        current_player.0 = current_player_id;
    }

    if game_logic.0.guessing_round {
        match_state.set(MatchState::Guessing);
    } else if game_logic.0.game_over {
        match_state.set(MatchState::Finished);
    } else {
        match_state.set(MatchState::Playing);
    }
}

pub fn spawn_cards(mut commands: Commands, game_logic: Res<LocalGameLogicRes>) {
    for player_id in 0..game_logic.0.player_card_count.len() {
        let cards = game_logic.0.get_player_cards(player_id as usize);
        for (i, card) in cards.iter().enumerate() {
            commands.spawn(components::Card {
                player_id: Some(player_id as usize),
                card_index: Some(i),
                card: card.clone(),
            });
        }
    }
}
