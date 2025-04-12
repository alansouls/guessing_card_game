

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Rank {
    Ace,
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card(pub Suit, pub Rank);

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.1 == other.1 {
            self.0.partial_cmp(&other.0)
        } else {
            self.1.partial_cmp(&other.1)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PlayedCard{
    pub player_id: usize,
    pub card: Card,
}