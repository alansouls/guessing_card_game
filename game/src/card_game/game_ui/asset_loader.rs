use bevy::{asset::AssetServer, sprite::Sprite};

use card_game_logic::game_logic::common::{Card, Rank, Suit};

pub trait AssetLoader {
    fn load_card_sprite(&self, card: &Card) -> Sprite;
}

impl AssetLoader for AssetServer {
    fn load_card_sprite(&self, card: &Card) -> Sprite {
        let suit: &str = match card.0 {
            Suit::Hearts => "h",
            Suit::Diamonds => "d",
            Suit::Clubs => "c",
            Suit::Spades => "s",
        };

        let rank: &str = match card.1 {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "j",
            Rank::Queen => "q",
            Rank::King => "k",
            Rank::Ace => "a",
        };

        let path = format!("sprites/cards/card_b_{}{}.png", suit, rank);

        Sprite::from_image(self.load(path))
    }
}