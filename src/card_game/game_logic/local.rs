use super::{
    GameLogic,
    common::{Card, CardPlayedResult, PlayedCard, Rank, Suit},
};

pub struct LocalGameLogic {
    pub player_turn: usize,
    pub player_cards: Vec<Vec<Card>>,
    pub player_card_count: Vec<usize>,
    pub game_over: bool,
    pub cards_played: Vec<PlayedCard>,
    pub deck: Vec<Card>,
    pub guessing_round: bool,
    pub guesses: Vec<usize>,
    last_to_guess: usize,
    starting_turn: usize,
    wins: Vec<usize>,
}

impl Default for LocalGameLogic {
    fn default() -> Self {
        Self {
            player_turn: 0,
            player_cards: Vec::new(),
            player_card_count: Vec::new(),
            game_over: false,
            cards_played: Vec::new(),
            deck: Vec::new(),
            guessing_round: false,
            guesses: Vec::new(),
            last_to_guess: 0,
            starting_turn: 0,
            wins: Vec::new(),
        }
    }
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

    while game_logic.player_card_count[game_logic.player_turn] == 0
    {
        game_logic.player_turn =
            (game_logic.player_turn + 1) % game_logic.player_card_count.len() as usize;
    }
    
    game_logic.starting_turn = game_logic.player_turn;
}

fn next_player_turn(game_logic: &mut LocalGameLogic) -> CardPlayedResult {
    game_logic.player_turn =
        (game_logic.player_turn + 1) % game_logic.player_card_count.len() as usize;

    while game_logic.player_card_count[game_logic.player_turn as usize] == 0
        && game_logic.player_turn != game_logic.starting_turn
    {
        game_logic.player_turn =
            (game_logic.player_turn + 1) % game_logic.player_card_count.len() as usize;
    }

    if game_logic.player_turn == game_logic.starting_turn {
        return check_turn_winner(game_logic);
    }

    CardPlayedResult::NextPlayer
}

fn check_turn_winner(game_logic: &mut LocalGameLogic) -> CardPlayedResult {
    // Find the winning card - should be the highest card of the lead suit
    let lead_suit = game_logic.cards_played[0].card.0;
    
    // Find the highest card of the lead suit
    let winning_card_index = game_logic.cards_played
        .iter()
        .enumerate()
        .filter(|(_, card)| card.card.0 == lead_suit) // Only consider cards of the lead suit
        .max_by(|(_, a), (_, b)| a.card.1.partial_cmp(&b.card.1).unwrap())
        .map(|(index, _)| index)
        .unwrap_or(0); // Default to first card if something goes wrong
    
    let winning_player = game_logic.cards_played[winning_card_index].player_id;
    
    game_logic.wins[winning_player] += 1;
    game_logic.player_turn = winning_player;
    
    game_logic.cards_played.clear();
    check_match_finished(game_logic)
}

fn check_match_finished(game_logic: &mut LocalGameLogic) -> CardPlayedResult {
    let mut match_finished = true;
    for cards in &game_logic.player_cards {
        if cards.len() > 0 {
            match_finished = false;
            break;
        }
    }

    if match_finished {
        remove_cards_from_players(game_logic);
        return start_match(game_logic);
    } else {
        start_playing_round(game_logic);
        return CardPlayedResult::NextTurn;
    }
}

fn remove_cards_from_players(game_logic: &mut LocalGameLogic) {
    for player_id in 0..game_logic.player_cards.len() {
        let wins = game_logic.wins[player_id];
        let guess = game_logic.guesses[player_id];
        if wins != guess {
            game_logic.player_card_count[player_id] -= 1;
        }
    }

    game_logic.cards_played.clear();
}

fn start_match(game_logic: &mut LocalGameLogic) -> CardPlayedResult {
    if game_logic
        .player_card_count
        .iter()
        .filter(|c| **c > 0)
        .count()
        == 1
    {
        game_logic.game_over = true;
        return CardPlayedResult::GameOver;
    }
    game_logic.deck = create_deck();
    shuffle_deck(&mut game_logic.deck);
    distribute_cards(game_logic);
    game_logic.guessing_round = true;
    game_logic.guesses = vec![0; game_logic.player_card_count.len()];

    if game_logic.player_turn == 0 {
        game_logic.last_to_guess = game_logic.player_card_count.len() - 1;
    } else {
        game_logic.last_to_guess = game_logic.player_turn - 1;
    }

    return CardPlayedResult::NextMatch;
}

fn distribute_cards(game_logic: &mut LocalGameLogic) {
    for (index, &cards_count) in game_logic.player_card_count.iter().enumerate() {
        if cards_count > 0 {
            // Only distribute cards to players who should have cards
            game_logic.player_cards[index] = if game_logic.deck.len() >= cards_count {
                game_logic.deck.split_off(game_logic.deck.len() - cards_count)
            } else {
                // Handle case where there might not be enough cards
                Vec::new()
            };
        } else {
            // Empty vector for players with no cards
            game_logic.player_cards[index] = Vec::new();
        }
    }
}

fn push_played_card(game_logic: &mut LocalGameLogic, card: &PlayedCard) {
    // In a trick-taking game, we need to track the highest card of the same suit as the first card
    if game_logic.cards_played.is_empty() {
        // First card of the trick
        game_logic.cards_played.push(*card);
    } else {
        let lead_card = game_logic.cards_played[0].card;
        let lead_suit = lead_card.0;
        
        // If same suit, compare by rank; if different suit, lead suit always wins
        if card.card.0 == lead_suit {
            // Same suit, compare ranks
            game_logic.cards_played.push(*card);
        } else {
            // Different suit, cannot win the trick
            game_logic.cards_played.push(*card);
        }
    }
}

impl GameLogic for LocalGameLogic {
    fn init(&mut self, player_count: usize, initial_card_count: usize) {
        self.player_turn = 0;
        self.player_card_count = vec![initial_card_count; player_count];
        self.game_over = false;
        self.wins = vec![0; player_count];
        self.player_cards = vec![Vec::new(); player_count];
        self.starting_turn = 0;
        self.cards_played = Vec::new();

        start_match(self);
    }

    fn set_guess(&mut self, player_id: usize, guess: usize) -> Result<(), String> {
        if self.game_over {
            return Err(String::from("Game is over"));
        }

        let total_guesses = self.guesses.iter().sum::<usize>();
        let max_cards = self.player_card_count.iter().max().unwrap_or(&0);
        let next_player = (self.player_turn + 1) % self.player_card_count.len() as usize;
        if self.player_turn == player_id {
            if total_guesses + guess == *max_cards && next_player == self.starting_turn {
                return Err(String::from("You cannot guess the same number of cards as the maximum cards in hand"));
            }

            self.guesses[player_id as usize] = guess;
            self.player_turn = next_player;

            if self.player_turn == self.starting_turn {
                start_playing_round(self);
            }
        }

        return Ok(());
    }

    fn play_card(&mut self, player_id: usize, card: &Card) -> Result<CardPlayedResult, String> {
        if self.game_over {
            return Err(String::from("Game is over"));
        }

        if self.guessing_round || self.player_turn != player_id {
            return Err(String::from("Guessing round or not your turn"));
        }

        match self.player_cards[player_id as usize]
            .iter()
            .position(|c| *c == *card)
        {
            Some(index) => {
                self.player_cards[player_id as usize].remove(index);

                push_played_card(
                    self,
                    &PlayedCard {
                        player_id: player_id as usize,
                        card: *card,
                    },
                );

                Ok(next_player_turn(self))
            }
            None => return Err(String::from("Player does not have this card")),
        }
    }

    fn get_player_cards(&self, player_id: usize) -> &Vec<Card> {
        return &self.player_cards[player_id as usize];
    }

    fn get_player_card_count(&self, player_id: usize) -> usize {
        self.player_card_count[player_id as usize]
    }

    fn get_player_turn(&self) -> usize {
        self.player_turn
    }

    fn get_player_guess(&self, player_id: usize) -> usize {
        self.guesses[player_id as usize]
    }

    fn get_winner(&self) -> usize {
        return self
            .player_card_count
            .iter()
            .enumerate()
            .filter(|(_, c)| **c > 0)  // Fixed reference pattern
            .map(|(id, _)| id)
            .next()
            .expect("No winner found");
    }
}
