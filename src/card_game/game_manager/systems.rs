use bevy::prelude::Commands;
use bevy::prelude::Query;
use super::components::{Card, Deck, Rank, Suit};

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

    commands.spawn(deck);
}

pub fn print_cards(query: Query<&Deck>) {
    let deck = query.single();

    for card in &deck.cards {
        println!("Suit: {:?} Rank: {:?}", card.0, card.1);
    }
}