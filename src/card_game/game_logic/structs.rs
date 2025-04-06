

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Card(pub Suit, pub Rank);

#[derive(Clone, Copy, Debug)]
pub struct PlayedCard{
    pub player_id: u8,
    pub card: Card,
}