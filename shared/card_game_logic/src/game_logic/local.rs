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

    while game_logic.player_card_count[game_logic.player_turn] == 0 {
        game_logic.player_turn =
            (game_logic.player_turn + 1) % game_logic.player_card_count.len() as usize;
    }

    game_logic.starting_turn = game_logic.player_turn;
}

fn next_player_turn(game_logic: &mut LocalGameLogic) -> CardPlayedResult {
    game_logic.player_turn =
        (game_logic.player_turn + 1) % game_logic.player_card_count.len() as usize;

    while game_logic.player_cards[game_logic.player_turn as usize].len() == 0
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
    let winning_player = game_logic.cards_played.last().unwrap().player_id;

    game_logic.wins[winning_player] += 1;
    game_logic.player_turn = winning_player;

    let has_cards_to_play = game_logic.player_cards.iter().any(|c| c.len() > 0);
    while game_logic.player_cards[game_logic.player_turn].len() == 0 && has_cards_to_play {
        game_logic.player_turn = (game_logic.player_turn + 1) % game_logic.player_card_count.len();
    }

    println!("Next player is {}", game_logic.player_turn);

    game_logic.cards_played.clear();
    check_match_finished(game_logic)
}

fn check_match_finished(game_logic: &mut LocalGameLogic) -> CardPlayedResult {
    let match_finished = game_logic.player_cards.iter().all(|c| c.len() == 0);

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
    game_logic.starting_turn = game_logic.player_turn;
    game_logic.deck = create_deck();
    shuffle_deck(&mut game_logic.deck);
    distribute_cards(game_logic);
    game_logic.guessing_round = true;
    game_logic.guesses = vec![0; game_logic.player_card_count.len()];
    game_logic.wins = vec![0; game_logic.player_card_count.len()];

    if game_logic.player_turn == 0 {
        game_logic.last_to_guess = game_logic.player_card_count.len() - 1;
    } else {
        game_logic.last_to_guess = game_logic.player_turn - 1;
    }

    return CardPlayedResult::NextMatch;
}

fn distribute_cards(game_logic: &mut LocalGameLogic) {
    for (index, cards_count) in game_logic.player_card_count.iter().enumerate() {
        game_logic.player_cards[index] = game_logic
            .deck
            .split_off(game_logic.deck.len() - cards_count);
    }
}

fn push_played_card(game_logic: &mut LocalGameLogic, card: &PlayedCard) {
    match game_logic.cards_played.pop() {
        Some(last_card) => {
            if last_card.card > card.card {
                game_logic.cards_played.push(*card);
                game_logic.cards_played.push(last_card);
            } else {
                game_logic.cards_played.push(last_card);
                game_logic.cards_played.push(*card);
            }
        }
        _ => game_logic.cards_played.push(*card),
    }
}

impl GameLogic for LocalGameLogic {
    fn init(&mut self, player_count: usize, initial_card_count: usize) {
        self.player_turn = 0;
        self.player_card_count = vec![initial_card_count; player_count];
        self.game_over = false;
        self.wins = vec![0; player_count];
        self.player_cards = vec![Vec::new(); player_count];
        self.cards_played = Vec::new();

        start_match(self);
    }

    fn set_guess(&mut self, player_id: usize, guess: usize) -> Result<(), String> {
        if self.game_over {
            return Err(String::from("Game is over"));
        }

        let total_guesses = self.guesses.iter().sum::<usize>();
        let max_cards = self.player_card_count.iter().max().unwrap_or(&0);
        let next_player = (self.player_turn + 1) % self.player_card_count.len();
        if self.player_turn == player_id {
            if total_guesses + guess == *max_cards && next_player == self.starting_turn {
                return Err(String::from(
                    "You cannot guess the same number of cards as the maximum cards in hand",
                ));
            }

            self.guesses[player_id] = guess;
            self.player_turn = next_player;

            while self.player_cards[self.player_turn].len() == 0 {
                self.player_turn = (self.player_turn + 1) % self.player_card_count.len();
            }

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

    fn get_player_wins(&self, player_id: usize) -> usize {
        self.wins[player_id]
    }

    fn get_winner(&self) -> usize {
        return self
            .player_card_count
            .iter()
            .filter(|c| **c > 0)
            .map(|c| *c)
            .collect::<Vec<usize>>()
            .pop()
            .expect("No winner found");
    }
}
