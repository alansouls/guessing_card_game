use bevy::{
    ecs::{
        entity::Entity, event::{EventReader, EventWriter}, query::With, system::{Commands, Query, Res, ResMut, Single}
    },
    state::state::NextState,
};

use crate::card_game::{
    GameSettings, GameState, LocalGameLogicRes,
    game_logic::{GameLogic, common::Card as CardStruct, local::LocalGameLogic},
};

use super::{
    MatchState,
    components::{self, CurrentPlayer, MaxGuess, TopPlayedCard},
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
    mut commands: Commands,
    mut game_logic: ResMut<LocalGameLogicRes>,
    mut event: EventReader<CardPlayed>,
    mut current_player: Single<&mut CurrentPlayer>,
    mut game_ended_writer: EventWriter<GameEnded>,
    mut match_state: ResMut<NextState<MatchState>>,
    mut cards: Query<(Entity, &mut components::Card)>,
    top_card: Option<Single<Entity, With<TopPlayedCard>>>,
) {
    let mapped_top_card = top_card.map(|t| *t);
    for event in event.read() {
        match game_logic.0.play_card(event.player_id, &event.card) {
            Ok(_) => {
                define_card_as_played(
                    &mut commands,
                    &game_logic.0,
                    event.player_id,
                    event.card,
                    &mut cards,
                    &mapped_top_card,
                );

                if game_logic.0.game_over {
                    game_ended_writer.send(GameEnded {
                        winner: game_logic.0.get_winner(),
                    });
                }

                update_current_player(&game_logic, current_player.as_mut(), match_state.as_mut());
            }
            Err(err) => {
                println!("Error playing card: {}", err);
            }
        }
    }
}

fn update_current_player(
    game_logic: &LocalGameLogicRes,
    current_player: &mut CurrentPlayer,
    match_state: &mut NextState<MatchState>,
) {
    let current_player_id = game_logic.0.get_player_turn();

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
        for card in cards.iter() {
            commands.spawn(components::Card {
                player_id: Some(player_id as usize),
                card: *card,
            });
        }
    }
}

fn define_card_as_played(
    commands: &mut Commands,
    game_logic: &LocalGameLogic,
    player_id: usize,
    card_value: CardStruct,
    cards: &mut Query<(Entity, &mut components::Card)>,
    top_card: &Option<Entity>,
) {
    for (card_entity, mut card) in cards.iter_mut() {
        if card.player_id != Some(player_id) {
            continue;
        }

        if card_value == card.card {
            card.player_id = None;
        }

        if let Some(last_card) = game_logic.cards_played.last() {
            if last_card.card == card.card {
                commands.entity(card_entity).insert(TopPlayedCard);
                if let Some(top_card_entity) = top_card {
                    commands.entity(*top_card_entity).remove::<TopPlayedCard>();
                }
            }
        }
    }
}
