use rand::rng;
use structs::{Card, PlayedCard, Rank, Suit};

pub mod structs;

pub trait GameLogic {
    fn init(&mut self, player_count: usize, initial_card_count: usize);
    fn set_guess(&mut self, player_id: u8, guess: usize);
    fn play_card(&mut self, player_id: u8, card_index: usize);
}

pub struct LocalGameLogic {
    pub player_turn: u8,
    pub player_cards: Vec<Vec<Card>>,
    pub player_card_count: Vec<usize>,
    pub game_over: bool,
    pub cards_played: Vec<PlayedCard>,
    pub deck: Vec<Card>,
    pub guessing_round: bool,
    pub guesses: Vec<usize>,
    starting_turn: u8,
    wins: Vec<usize>,
}

fn create_deck() -> Vec<Card> {
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
    return suits
        .iter()
        .flat_map(|&suit| ranks.iter().map(move |&rank| Card(suit, rank)))
        .collect();
}

fn shuffle_deck(deck: &mut Vec<Card>) {
    use rand::rng;
    use rand::seq::SliceRandom;

    let mut rng = rng();
    deck.shuffle(&mut rng);
}

fn start_playing_round(game_logic: &mut LocalGameLogic) {
    game_logic.guessing_round = false;
    game_logic.starting_turn = game_logic.player_turn;
}

fn next_player_turn(game_logic: &mut LocalGameLogic) {
    game_logic.player_turn = (game_logic.player_turn + 1) % game_logic.player_card_count.len() as u8;

    if game_logic.player_turn == game_logic.starting_turn {
    }
}

fn check_turn_winner(game_logic: &mut LocalGameLogic) {
    let mut winning_card = game_logic.cards_played[0].card;
    let mut winning_player = game_logic.cards_played[0].player_id;

    for played_card in &game_logic.cards_played {
        if played_card.card > winning_card {
            winning_card = played_card.card;
            winning_player = played_card.player_id;
        }
    }

    game_logic.wins[winning_player as usize] += 1;
    game_logic.starting_turn = winning_player;

    check_match_finished(game_logic);
}

fn check_match_finished(game_logic: &mut LocalGameLogic) {
    let mut match_finished = true;
    for cards in &game_logic.player_cards {
        if cards.len() > 0 {
            match_finished = false;
            break;
        }
    }

    if match_finished {

    }
    else {
        start_playing_round(game_logic);
    }
}

fn remove_cards_from_players(game_logic: &mut LocalGameLogic) {
    for player_id in 0..game_logic.player_cards.len() {
        let player_cards = &mut game_logic.player_cards[player_id];
        let mut i = 0;
        while i < player_cards.len() {
            if game_logic.cards_played.iter().any(|played_card| played_card.card == player_cards[i]) {
                player_cards.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

impl GameLogic for LocalGameLogic {
    fn init(&mut self, player_count: usize, initial_card_count: usize) {
        self.player_turn = 0;
        self.player_card_count = vec![initial_card_count; player_count]; // Assuming 4 players
        self.game_over = false;
        self.guessing_round = true;
        self.guesses = vec![0; player_count];
        self.wins = vec![0; player_count];
        self.player_cards = vec![Vec::new(); player_count];
        self.starting_turn = 0;
        self.cards_played = Vec::new();
        self.deck = create_deck();

        shuffle_deck(&mut self.deck);
    }

    fn set_guess(&mut self, player_id: u8, guess: usize) {
        if self.player_turn == player_id {
            self.guesses[player_id as usize] = guess;
            self.player_turn = (self.player_turn + 1) % self.player_card_count.len() as u8;

            if self.player_turn == self.starting_turn {
                start_playing_round(self);
            }
        }
    }

    fn play_card(&mut self, player_id: u8, card_index: usize) {
        if self.guessing_round || self.player_turn != player_id {
            return;
        }

        if card_index >= self.player_cards[player_id as usize].len() {
            return;
        }

        if card_index < self.player_cards[player_id as usize].len() {
            let card = self.player_cards[player_id as usize].remove(card_index);
            self.cards_played.push(PlayedCard { player_id, card });
        }
    }
}
