use crate::{Config, AOCProblem};
use core::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType { High, OnePair, TwoPair, Three, FullHouse, Four, Five }

#[derive(Debug, Eq, PartialEq, Ord)]
struct PokerHand {
    hand: String,
    bid: u32,
    hand_type: HandType,
}

impl PokerHand {
    fn new(hand_str: &str, bid: u32) -> PokerHand {
        let mut p = PokerHand {
            hand: String::from(hand_str),
            bid,
            hand_type: HandType::High,
        };
        p.compute_hand_type();
        p
    }

    // We compute these in order (try five first, then four, etc)
    //  So when checking for four of a kind, we can be sure it's not five of a kind

    fn is_five_of_a_kind(&self, counts: &HashMap<char, u8>) -> bool {
        if counts.len() == 1 { return true; } // All the same card
        // If we have 2 card types and one is Wild card (O), that's 5 too
        if counts.len() == 2 && counts.contains_key(&'O') { return true; }
        false
    }

    fn is_four_of_a_kind(&self, counts: &HashMap<char, u8>) -> bool {
        let wild_cards = *counts.get(&'O').unwrap_or(&0);
        for count in counts {
            if count.0 == &'O' { continue; }
            if *count.1 + wild_cards == 4 { return true; }
        }
        return false;
    }

    fn is_full_house(&self, counts: &HashMap<char, u8>) -> bool {
        let wild_cards = *counts.get(&'O').unwrap_or(&0);
        // FH consists of two non wild cards
        // Then we can add wild cards to either/or
        let mut card_a = None;
        let mut card_b = None;
        for count in counts {
            if count.0 == &'O' {
                continue;
            }
            if card_a == None {
                card_a = Some(count);
            } else if card_b == None {
                card_b = Some(count);
            } else {
                return false; // More than 2 non-wild cards
            }
        }
        if card_a == None || card_b == None {
            return false;
        }
        let some_card_a = card_a.unwrap();
        let some_card_b = card_b.unwrap();
        if *some_card_a.1 + wild_cards == 3 && *some_card_b.1 == 2 {
            return true;
        }
        if *some_card_b.1 + wild_cards == 3 && *some_card_a.1 == 2 {
            return true;
        }
        return false;
    }

    fn is_three_of_a_kind(&self, counts: &HashMap<char, u8>) -> bool {
        let wild_cards = *counts.get(&'O').unwrap_or(&0);
        for count in counts {
            if *count.1 + wild_cards == 3 { return true; }
        }
        return false;
    }

    fn is_two_pair(&self, counts: &HashMap<char, u8>) -> bool {
        // Can't have 2 pair with any wild cards!
        // If we have 2 wild cards, it's 3 of a kind at least
        // if we have 1 wild card, we need at least 1 real pair to get 2 pair
        //    And that real pair + wild card will be 3
        let wild_cards = *counts.get(&'O').unwrap_or(&0);
        if wild_cards > 0 {
            return false;
        }
        if counts.len() != 3 { return false; }
        for count in counts {
            // If one count is 2 and we have 3 different card types
            // Then the others have to be 2 and 1
            // Actually, we're done if count.len is 3, since it has to be 2/2/1
            //   since we already looked for 3/1/1
            if *count.1 == 2 { return true; }
        }
        return false;
    }
    fn is_one_pair(&self, counts: &HashMap<char, u8>) -> bool {
        let wild_cards = *counts.get(&'O').unwrap_or(&0);
        if wild_cards == 0 {
            // no wild cards, 4 different types means 2/1/1/1
            if counts.len() != 4 { return false; }
            return true;
        }
        // With a wild card, we need 1/1/1/1/1 or it's something better
        if wild_cards == 1 && counts.len() == 5 { return true; }
        return false;
    }

    fn compute_hand_type(&mut self) {
        let cards: Vec<char> = self.hand.chars().collect();
        let mut map = HashMap::new();
        for c in cards {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        if self.is_five_of_a_kind(&map) { 
            self.hand_type = HandType::Five;
        } else if self.is_four_of_a_kind(&map) { 
            self.hand_type = HandType::Four;
        } else if self.is_full_house(&map) {
            self.hand_type = HandType::FullHouse;
        } else if self.is_three_of_a_kind(&map) {
            self.hand_type = HandType::Three;
        } else if self.is_two_pair(&map) {
            self.hand_type = HandType::TwoPair;
        } else if self.is_one_pair(&map) {
            self.hand_type = HandType::OnePair;
        }
    }

    fn get_card_val(&self, c: &char) -> u8 {
        match c {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            'O' => 0,
            _ => 0,
        }
    }

    fn card_order(&self, c1: &char, c2: &char) -> Ordering {
        return self.get_card_val(c1).cmp(&self.get_card_val(c2));
    }

}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type != other.hand_type {
            return Some(self.hand_type.cmp(&other.hand_type));
        }
        // Same type, compare cards one by one
        let cards1: Vec<char> = self.hand.chars().collect();
        let cards2: Vec<char> = other.hand.chars().collect();
        for i in 0..5 {
            let c1 = cards1.get(i).unwrap();
            let c2 = cards2.get(i).unwrap();
            if c1 != c2 {
                return Some(self.card_order(c1, c2));
            }
        }
        println!("These must be the same hand: {}, {}", self.hand, other.hand);
        return Some(Ordering::Equal);
    }
}

pub struct Day7 {
    hands: Vec<PokerHand>,
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 {
            hands: Vec::new(),
        }
    }
}

impl AOCProblem for Day7 {
    fn handle_line(&mut self, line: &str, config: &Config) {
        let mut line_str: String = String::from(line);
        if config.variant {
            // To make differetiating between part a and b easier
            //  We'll make Jokers the letter O instead of J
            // Since part A has no jokers, there will be no O's in any hand
            //  And we can use the same code
            line_str = line_str.replace("J", "O");
        }
        let mut line_iter = line_str.split_whitespace();

        let hand = line_iter.next().unwrap();
        let bid = line_iter.next().unwrap().parse::<u32>().unwrap();
        let hand = PokerHand::new(hand, bid);
        println!("Parsed hand: {:?}", hand);
        self.hands.push(hand);
    }
    
    // Sort the list
    fn compute_a(&mut self) -> String {
        self.hands.sort();
        let mut rank: u64 = 1;
        let mut winnings: u64 = 0;
        for hand in &self.hands {
            let hand_val: u64 = rank * hand.bid as u64;
            println!("Hand {:?} Rank {} = Winnings {}", hand, rank, hand_val);
            winnings = winnings + hand_val;
            rank += 1;
        }
        winnings.to_string()
    }

    fn compute_b(&mut self) -> String {
        return self.compute_a();
    }
}
