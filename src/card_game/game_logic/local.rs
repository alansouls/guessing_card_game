use super::{
    GameLogic,
    common::{Card, PlayedCard, Rank, Suit},
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
    game_logic.starting_turn = game_logic.player_turn;
}

fn next_player_turn(game_logic: &mut LocalGameLogic) {
    let starting_turn = game_logic.player_turn;
    game_logic.player_turn =
        (game_logic.player_turn + 1) % game_logic.player_card_count.len() as usize;

    while game_logic.player_card_count[game_logic.player_turn as usize] == 0
        && game_logic.player_turn != starting_turn
    {
        game_logic.player_turn =
            (game_logic.player_turn + 1) % game_logic.player_card_count.len() as usize;
    }

    if game_logic.player_turn == starting_turn {
        check_turn_winner(game_logic);
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

    remove_cards_from_players(game_logic);
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
        start_match(game_logic);
    } else {
        start_playing_round(game_logic);
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

fn start_match(game_logic: &mut LocalGameLogic) {
    if game_logic
        .player_card_count
        .iter()
        .filter(|c| **c > 0)
        .count()
        == 1
    {
        game_logic.game_over = true;
        return;
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
}

fn distribute_cards(game_logic: &mut LocalGameLogic) {
    let mut i = 0;
    for cards_count in &game_logic.player_card_count {
        game_logic.player_cards[i] = game_logic
            .deck
            .split_off(game_logic.deck.len() - cards_count);
        i += 1;
    }
}

impl GameLogic for LocalGameLogic {
    fn init(&mut self, player_count: usize, initial_card_count: usize) {
        self.player_turn = 0;
        self.player_card_count = vec![initial_card_count; player_count]; // Assuming 4 players
        self.game_over = false;
        self.wins = vec![0; player_count];
        self.player_cards = vec![Vec::new(); player_count];
        self.starting_turn = 0;
        self.cards_played = Vec::new();

        start_match(self);
    }

    fn set_guess(&mut self, player_id: usize, guess: usize) {
        if self.game_over {
            return;
        }

        let total_guesses = self.guesses.iter().sum::<usize>();
        if self.player_turn == player_id {
            if total_guesses + guess == self.player_card_count.len() {
                return;
            }

            self.guesses[player_id as usize] = guess;
            self.player_turn = (self.player_turn + 1) % self.player_card_count.len() as usize;

            if self.player_turn == self.starting_turn {
                start_playing_round(self);
            }
        }
    }

    fn play_card(&mut self, player_id: usize, card_index: usize) {
        if self.game_over {
            return;
        }

        if self.guessing_round || self.player_turn != player_id {
            return;
        }

        if card_index >= self.player_cards[player_id as usize].len() {
            return;
        }

        if card_index < self.player_cards[player_id as usize].len() {
            let card = self.player_cards[player_id as usize].remove(card_index);
            self.cards_played.push(PlayedCard { player_id, card });

            next_player_turn(self);
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
            .filter(|c| **c > 0)
            .map(|c| *c)
            .collect::<Vec<usize>>()
            .pop()
            .expect("No winner found");
    }
}
