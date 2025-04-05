use bevy::prelude::Component;

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Clone, Copy, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Card(pub Suit, pub Rank);

#[derive(Component)]
pub struct Deck { pub cards: Vec<Card> }