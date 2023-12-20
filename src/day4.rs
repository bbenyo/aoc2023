use crate::{Config, AOCProblem};

pub struct ScratchCard {
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
    score: i32,
    win_count: u32,
    index: u32,
}

pub struct Day4 {
    cards: Vec<ScratchCard>,
}

impl ScratchCard {
    fn compute_score(&mut self) {
        let mut wins = 0;
        for num in &self.winning_numbers {
            if self.card_numbers.contains(&num) {
                println!("Found winning number for {}: {}", self.index, num);
                wins += 1;
            }
        }
        self.win_count = wins;
        if wins == 0 {
            self.score = 0;
        } else {
            let base: i32 = 2;
            self.score = base.pow(wins - 1);
        }
    }
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {
            cards: Vec::new(),
        }
    }
}

use std::collections::HashMap;

impl AOCProblem for Day4 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let mut line_iter = line.split_whitespace();

        line_iter.next(); // Card
        let card_id;

        if let Some(id) = line_iter.next() {
            let id_num = &id[0..id.len() - 1]; // Strip off the :
            card_id = id_num.parse::<u32>().unwrap_or(0);
        } else {
            eprintln!("Card id not found in {}", line);
            return;
        }

        let mut winning_numbers: Vec<u32> = Vec::new();
        let mut card_numbers: Vec<u32> = Vec::new();
        let mut winning = true; // We're parsing winning numbers until we hit the |
        for token in line_iter {
            if token.len() == 0 {
                continue;
            }
            if token == "|" {
                winning = false;
                continue;
            }

            let number = token.parse::<u32>().unwrap_or(0);
            if winning {
                winning_numbers.push(number);
            } else {
                card_numbers.push(number);
            }
        }

        let mut new_card = ScratchCard {
            winning_numbers,
            card_numbers,
            score: 0,
            win_count: 0,
            index: card_id,
        };

        new_card.compute_score();
        self.cards.push(new_card);
    }

    fn compute_a(&mut self) -> String {
        let mut sum = 0;
        for card in &self.cards {
            println!("Card Score for {} = {}", card.index, card.score);
            sum += card.score;
        }
        sum.to_string()
    }

    fn compute_b(&mut self) -> String {
        // How many card of each type do I have?
        let mut card_counts: HashMap<u32, u32> = HashMap::new();
        let mut i: u32 = 1;
        let mut total_cards = 0;
        for card in &self.cards {
            let cur_count = card_counts.entry(i).or_insert(0);
            *cur_count += 1; // Our real, non-copied card
            let i_count = *cur_count;
            // We can't get any more of this card, so add it to the total
            total_cards += i_count;
            println!("We have {} cards of id {}.  Win_count: {}", i_count, card.index, card.win_count);
            for j in i..(i + card.win_count) {
                if j < self.cards.len() as u32 {
                    // Add copies for the next N cards
                    let j_count = card_counts.entry(j+1).or_insert(0);
                    *j_count += i_count;
                    println!("Adding {} cards of type {}", i_count, j)
                }
            }
            i += 1;
        }
        total_cards.to_string()
    }
}