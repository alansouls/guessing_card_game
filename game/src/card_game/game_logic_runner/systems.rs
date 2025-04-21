use bevy::{
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res, ResMut, Single},
    },
    hierarchy::DespawnRecursiveExt,
    state::state::NextState,
    time::{Time, Timer, TimerMode},
};

use crate::card_game::{GameLogicRes, GameSettings, GameState};

use card_game_logic::game_logic::{
    GameLogic,
    common::{Card as CardStruct, CardPlayedResult},
};

use super::{
    MatchState,
    components::{self, CurrentPlayer, DisplayPlayedCardTimer, MaxGuess, TopPlayedCard},
    events::{CardPlayed, GameEnded, PlayerGuessed, PlayerInfoUpdated},
    game_logic_facade::GameLogicFacade,
};

pub fn local_game_init(mut game_logic: ResMut<GameLogicRes>, game_settings: Res<GameSettings>) {
    game_logic.0.init_local(game_settings.inital_card_count);
}

pub fn online_game_init(mut game_logic: ResMut<GameLogicRes>, game_settings: Res<GameSettings>) {
    game_logic.0.init_online(
        &game_settings.online_player_name,
        &game_settings.online_room_name,
    );
}

pub fn handle_game_start(
    mut commands: Commands,
    mut game_logic: ResMut<GameLogicRes>,
    game_settings: Res<GameSettings>,
    mut match_state: ResMut<NextState<MatchState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_logic.0.start_match(game_settings.inital_card_count);

    commands.spawn(MaxGuess(game_settings.inital_card_count));

    commands.spawn(CurrentPlayer(game_logic.0.get_player_turn()));
    game_state.set(GameState::LocalGame);
    match_state.set(MatchState::Guessing);
}

pub fn handle_player_guess(
    mut commands: Commands,
    mut game_logic: ResMut<GameLogicRes>,
    mut event: EventReader<PlayerGuessed>,
    mut current_player: Single<&mut CurrentPlayer>,
    mut match_state: ResMut<NextState<MatchState>>,
    mut player_info_event: EventWriter<PlayerInfoUpdated>,
) {
    for event in event.read() {
        match game_logic.0.set_guess(event.player_id, event.guess) {
            Ok(_) => {
                update_current_player(
                    false,
                    &mut commands,
                    &game_logic,
                    current_player.as_mut(),
                    match_state.as_mut(),
                );

                player_info_event.send(PlayerInfoUpdated);
            }
            Err(_) => (),
        }
    }
}

pub fn handle_card_played(
    mut commands: Commands,
    mut game_logic: ResMut<GameLogicRes>,
    mut event: EventReader<CardPlayed>,
    mut current_player: Single<&mut CurrentPlayer>,
    mut game_ended_writer: EventWriter<GameEnded>,
    mut match_state: ResMut<NextState<MatchState>>,
    mut cards: Query<(Entity, &mut components::Card)>,
    top_card: Option<Single<Entity, With<TopPlayedCard>>>,
    mut player_info_event: EventWriter<PlayerInfoUpdated>,
) {
    let mapped_top_card = top_card.map(|t| *t);
    for event in event.read() {
        match game_logic.0.play_card(event.player_id, &event.card) {
            Ok(CardPlayedResult::NextPlayer) => {
                define_card_as_played(
                    &mut commands,
                    &game_logic.0,
                    event.player_id,
                    event.card,
                    &mut cards,
                    &mapped_top_card,
                );

                update_current_player(
                    false,
                    &mut commands,
                    &game_logic,
                    current_player.as_mut(),
                    match_state.as_mut(),
                );
                player_info_event.send(PlayerInfoUpdated);
            }
            Ok(CardPlayedResult::NextTurn) | Ok(CardPlayedResult::NextMatch) => {
                define_card_as_played(
                    &mut commands,
                    &game_logic.0,
                    event.player_id,
                    event.card,
                    &mut cards,
                    &mapped_top_card,
                );

                update_current_player(
                    true,
                    &mut commands,
                    &game_logic,
                    current_player.as_mut(),
                    match_state.as_mut(),
                );
                player_info_event.send(PlayerInfoUpdated);
            }
            Ok(CardPlayedResult::GameOver) => {
                define_card_as_played(
                    &mut commands,
                    &game_logic.0,
                    event.player_id,
                    event.card,
                    &mut cards,
                    &mapped_top_card,
                );
                player_info_event.send(PlayerInfoUpdated);

                let winner = game_logic.0.get_winner();
                game_ended_writer.send(GameEnded { winner });
            }
            Err(err) => {
                println!("Error playing card: {}", err);
            }
        }
    }
}

pub fn spawn_cards(mut commands: Commands, game_logic: Res<GameLogicRes>) {
    for player_id in 0..game_logic.0.get_player_card_counts().len() {
        let cards = game_logic.0.get_player_cards(player_id as usize);
        for card in cards.iter() {
            commands.spawn(components::Card {
                player_id: Some(player_id as usize),
                card: *card,
            });
        }
    }
}

pub fn clear_cards(
    mut commands: Commands,
    time: Res<Time>,
    display_played_card_timer: Option<Single<(Entity, &mut DisplayPlayedCardTimer)>>,
    mut cards: Query<Entity, With<components::Card>>,
    mut match_state: ResMut<NextState<MatchState>>,
    mut current_player: Single<&mut CurrentPlayer>,
) {
    if display_played_card_timer.is_none() {
        return;
    }

    let timer_entity = display_played_card_timer.as_ref().unwrap().0;
    let timer_component = &mut (display_played_card_timer.unwrap().1);

    timer_component.timer.tick(time.delta());

    if timer_component.timer.finished() {
        for card_entity in cards.iter_mut() {
            commands.entity(card_entity).despawn_recursive();
        }

        commands.entity(timer_entity).despawn_recursive();
        match_state.set(timer_component.match_state);
        current_player.0 = timer_component.next_player_id;
    }
}

fn update_current_player(
    clear_played_cards: bool,
    commands: &mut Commands,
    game_logic: &GameLogicRes,
    current_player: &mut CurrentPlayer,
    match_state: &mut NextState<MatchState>,
) {
    let next_state = if game_logic.0.get_guessing_round() {
        MatchState::Guessing
    } else if game_logic.0.get_game_over() {
        MatchState::Finished
    } else {
        MatchState::Playing
    };

    let current_player_id = game_logic.0.get_player_turn();

    if clear_played_cards {
        commands.spawn(DisplayPlayedCardTimer {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
            match_state: next_state,
            next_player_id: current_player_id,
        });
        match_state.set(MatchState::DisplayingPlayedCard);
    } else {
        if current_player_id != current_player.0 {
            current_player.0 = current_player_id;
        }

        match_state.set(next_state);
    }
}

fn define_card_as_played(
    commands: &mut Commands,
    game_logic: &GameLogicFacade,
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

        if let Some(last_card) = game_logic.get_played_cards().last() {
            if last_card.card == card.card {
                commands.entity(card_entity).insert(TopPlayedCard);
                if let Some(top_card_entity) = top_card {
                    commands.entity(*top_card_entity).remove::<TopPlayedCard>();
                }
            }
        }
    }
}

pub fn setup_player_infos(mut commands: Commands, game_logic: Res<GameLogicRes>) {
    for player_id in 0..game_logic.0.get_player_card_counts().len() {
        let card_count = game_logic.0.get_player_cards(player_id).len();
        let guess = game_logic.0.get_player_guess(player_id);
        let wins = game_logic.0.get_player_wins(player_id);

        commands.spawn(components::PlayerInfo {
            player_id,
            card_count,
            guess,
            wins,
        });
    }
}

pub fn update_player_infos(
    mut event: EventReader<PlayerInfoUpdated>,
    game_logic: Res<GameLogicRes>,
    mut player_info_query: Query<&mut components::PlayerInfo>,
) {
    for _ in event.read() {
        for mut player_info in player_info_query.iter_mut() {
            let player_id = player_info.player_id;

            let card_count = game_logic.0.get_player_cards(player_id).len();
            let guess = game_logic.0.get_player_guess(player_id);
            let wins = game_logic.0.get_player_wins(player_id);

            if player_info.card_count != card_count
                || player_info.guess != guess
                || player_info.wins != wins
            {
                player_info.card_count = card_count;
                player_info.guess = guess;
                player_info.wins = wins;
            }
        }
    }
}
