use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////

type Deck = Vec<usize>;
type Score = Option<usize>;

struct GameResult {
    winner: u32,
    score: Score,
}

struct CombatGame {
    deck1: Deck,
    deck2: Deck,
    rec: bool,
    previous_states: HashSet<(usize, usize)>,
}

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();
    
    let f = BufReader::new(File::open("../input/day22.txt").unwrap());
    let (deck1, deck2) = read_decks(f);

    let mut game1 = CombatGame::new(deck1.clone(), deck2.clone(), false);
    let sol_part_1 = game1.play_game(true).score.unwrap();
    
    let mut game2 = CombatGame::new(deck1, deck2, true);
    let sol_part_2 = game2.play_game(true).score.unwrap();

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

impl CombatGame {

    fn new(deck1: Deck, deck2: Deck, rec: bool) -> Self {
        CombatGame { deck1, deck2, rec, previous_states: HashSet::new() }
    }
    
    fn play_game(&mut self, return_score: bool) -> GameResult {
        let mut winner = 0;
        while !self.deck1.is_empty() && !self.deck2.is_empty() {
            if self.rec {
                let scores = (self.compute_score(1), self.compute_score(2));

                if self.previous_states.contains(&scores) {
                    return GameResult { winner: 1, score: None };
                } else {
                    self.previous_states.insert(scores);
                }
            }
            winner = self.play_round();
        }

        let score = if return_score {
            Some(self.compute_score(winner))
        } else {
            None
        };

        return GameResult { winner, score };
    }

    fn play_round(&mut self) -> u32 {
        let card1 = self.deck1.remove(0);
        let card2 = self.deck2.remove(0);

        let winner = if self.rec && self.deck1.len() >= card1 && self.deck2.len() >= card2 {
            let new_deck_1 = self.deck1[..card1].to_vec();
            let new_deck_2 = self.deck2[..card2].to_vec();
            CombatGame::new(new_deck_1, new_deck_2, true).play_game(false).winner
        } else if card1 > card2 {1} else {2};

        if winner == 1 {
            self.deck1.push(card1);
            self.deck1.push(card2);
        } else {
            self.deck2.push(card2);
            self.deck2.push(card1);
        }

        return winner;
    }

    fn compute_score(&self, winner: u32) -> usize {
        let winner_deck = match winner {
            1 => &self.deck1,
            2 => &self.deck2,
            _ => panic!(),
        };

        let n_cards = winner_deck.len();
        return winner_deck.iter().enumerate().map(|(i, val)| (n_cards - i) * *val).sum();
    }
}

///////////////////////////////////////////////////////////////////////////////

fn read_decks(f: BufReader<File>) -> (Deck, Deck) {
    let mut ls = f.lines();
    let mut line: String;

    let mut deck1 = Vec::new();
    let mut deck2 = Vec::new();

    ls.next();

    while {line = ls.next().unwrap().unwrap(); !line.is_empty()} {
        deck1.push(line.parse().unwrap());
    }

    ls.next();

    for line in ls {
        deck2.push(line.unwrap().parse().unwrap());
    }

    return (deck1, deck2);
}
