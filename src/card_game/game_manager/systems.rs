use std::io;

use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventWriter;
use bevy::prelude::Commands;
use bevy::prelude::Query;
use rand::prelude::*;

use super::components::CardsPlayed;
use super::components::GameState;
use super::components::Player;
use super::components::PlayerCount;
use super::components::PlayerHand;
use super::components::{Card, Deck, Rank, Suit};

const INITAL_HAND_SIZE: usize = 3; // Number of cards each player receives

pub fn create_deck(mut commands: Commands) {
    let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
    let ranks = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];

    // Collect all combinations of suits and ranks into an array
    let cards: Vec<Card> = suits
        .iter()
        .flat_map(|&suit| ranks.iter().map(move |&rank| Card(suit, rank)))
        .collect();

    let deck = Deck { cards };

    let game_state = GameState {
        player_turn: 0,
        player_card_count: vec![INITAL_HAND_SIZE; 4],
        game_over: false,
    };

    let player_count = PlayerCount(4);
    let cards_played = CardsPlayed { cards: vec![] };
    commands.spawn((game_state, player_count, deck, cards_played));
}

pub fn print_cards(query: Query<&Deck>) {
    let deck = query.single();

    for card in &deck.cards {
        println!("Suit: {:?} Rank: {:?}", card.0, card.1);
    }
}

pub fn shuffle_deck(mut query: Query<&mut Deck>) {
    for mut deck in query.iter_mut() {
        deck.cards.shuffle(&mut rand::rng());
    }
}

pub fn create_players(mut commands: Commands, query: Query<&mut PlayerCount>) {
    let player_count = query.single().0;

    for i in 0..player_count {
        commands.spawn(Player {
            player_id: i,
            name: format!("Player {}", i + 1),
        });
    }
}

pub fn distribute_cards(
    mut commands: Commands,
    mut deck_query: Query<(&GameState, &mut Deck)>,
    mut query: Query<(Entity, &mut Player)>,
) {
    let (game_state, mut deck) = deck_query.single_mut();

    for player in query.iter_mut() {
        let deck_lenght = deck.cards.len();
        let hand_size = game_state.player_card_count[player.1.player_id as usize];
        let cards = deck.cards.split_off(deck_lenght - hand_size);

        let mut player_entity = commands.entity(player.0);
        player_entity.insert(PlayerHand { cards });
    }
}

pub fn print_player_hands(query: Query<(&Player, &PlayerHand)>) {
    for (player, hand) in query.iter() {
        println!("{}'s hand:", player.name);
        for card in &hand.cards {
            println!("Suit: {:?} Rank: {:?}", card.0, card.1);
        }
    }
}

pub fn print_deck_count(query: Query<&Deck>) {
    let deck = query.single();
    println!("Deck count: {}", deck.cards.len());
}

pub fn execute_turn(
    mut game_state_query: Query<(&GameState, &mut CardsPlayed)>,
    mut player_query: Query<(&Player, &mut PlayerHand)>,
) {
    let (game_state, mut cards_played) = game_state_query.single_mut();

    let (player, mut player_hand) = player_query
        .iter_mut()
        .filter(|p| p.0.player_id == game_state.player_turn)
        .next()
        .unwrap_or_else(|| panic!("Player with ID {} not found", game_state.player_turn));

    println!("It's {}'s turn!", player.name);

    println!("Select a card to play (0-{})", player_hand.cards.len() - 1);

    let card_to_play: usize = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(num) if num < player_hand.cards.len() => break num,
            _ => println!(
                "Invalid input. Please enter a number between 0 and {}.",
                player_hand.cards.len() - 1
            ),
        };
    };

    let played_card = player_hand.cards.remove(card_to_play);
    println!(
        "{} played card: Suit: {:?} Rank: {:?}",
        player.name, played_card.0, played_card.1
    );

    cards_played.cards.push(played_card.clone());
}

pub fn handle_after_turn(
    mut exit: EventWriter<bevy::app::AppExit>,
    mut game_state_query: Query<(&mut GameState, &PlayerCount)>,
    mut player_query: Query<(&Player, &PlayerHand)>,
) {
    let (mut game_state, player_count) = game_state_query.single_mut();

    game_state.player_turn = (game_state.player_turn + 1) % player_count.0;

    let mut player_tried = 0;

    while player_tried < player_count.0 {
        let player_hand = player_query
            .iter_mut()
            .filter(|p| p.0.player_id == game_state.player_turn)
            .next()
            .unwrap_or_else(|| panic!("Player with ID {} not found", game_state.player_turn)).1;

        if player_hand.cards.len() == 0 {
            game_state.player_turn = (game_state.player_turn + 1) % player_count.0;
            player_tried += 1;
        }
        else {
            break;
        }
    }

    if player_tried == player_count.0 {
        println!("All players have played all their cards. Game over!");
        game_state.game_over = true;
    }

    if game_state.game_over {
        exit.send(bevy::app::AppExit::Success);
        return;
    }
}

pub fn print_cards_played(query: Query<&CardsPlayed>) {
    let cards_played = query.single();

    for card in &cards_played.cards {
        println!("Played card: Suit: {:?} Rank: {:?}", card.0, card.1);
    }
}
