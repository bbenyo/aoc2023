use crate::{Config, AOCProblem};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

pub struct Day14 {
    board: Vec<Vec<char>>,
}

fn print_board(board: &Vec<Vec<char>>) {
    println!("Board:\n");
    for i in 0..board.len() {
        let foo = &board[i].iter().collect::<String>();
        println!("{:?}", foo);
    }
}

// Could hash the board state as n bit integers for each row where 1 bits are positions of the O
//  But of course, we have a 100x100 grid, which is too long
//  Could try a smarter hash function, but we'll try just creating the full state string
fn board_hash_str(board: &Vec<Vec<char>>) -> u64 {
    let mut str = String::new();
    for i in 0..board.len() {
        let foo = &board[i].iter().collect::<String>();
        str.push_str(foo);
    }
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);
    hasher.finish()
}

fn tilt_north(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted = board.clone();
    for i in 1..tilted.len() {
        for j in 0..tilted[i].len() {
            if tilted[i][j] != 'O' { continue; } // Only move rocks
            let mut moved = i;
            for k in 1..i+1 {
                // move up k spots max until we hit a O or #
                if tilted[i-k][j] == '.' { moved = i-k } else { break; }
            }
            tilted[i][j] = '.';
            tilted[moved][j] = 'O';
        }
    }
    tilted
}

fn tilt_west(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted = board.clone();
    for j in 1..tilted[0].len() {
        for i in 0..tilted.len() {
            if tilted[i][j] != 'O' { continue; } // Only move rocks
            let mut moved = j;
            for k in 1..j+1 {
                // move up k spots max until we hit a O or #
                if tilted[i][j-k] == '.' { moved = j-k } else { break; }
            }
            tilted[i][j] = '.';
            tilted[i][moved] = 'O';
        }
    }
    tilted
}

fn tilt_south(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted = board.clone();
    let tlen = tilted.len();
    for i in 1..tlen {
        for j in 0..tilted[i].len() {
            if tilted[tlen-i-1][j] != 'O' { continue; } // Only move rocks
            let mut moved = tlen-i-1;
            for k in 1..i+1 {
                // move up k spots max until we hit a O or #
                if tilted[tlen-i-1+k][j] == '.' { moved = tlen-i-1+k } else { break; }
            }
            tilted[tlen-i-1][j] = '.';
            tilted[moved][j] = 'O';
        }
    }
    tilted
}

fn tilt_east(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted = board.clone();
    let tlen = tilted[0].len();
    for j in 1..tlen {
        for i in 0..tilted.len() {
            if tilted[i][tlen-j-1] != 'O' { continue; } // Only move rocks
            let mut moved = tlen-j-1;
            for k in 1..j+1 {
                // move up k spots max until we hit a O or #
                if tilted[i][tlen-j-1+k] == '.' { moved = tlen-j-1+k } else { break; }
            }
            tilted[i][tlen-j-1] = '.';
            tilted[i][moved] = 'O';
        }
    }
    tilted
}


impl Day14 {
    pub fn new() -> Day14 {
        Day14 {
            board: Vec::new(),
        }
    }

    fn score_north(&self, tilted: Vec<Vec<char>>) -> usize {
        let mut score: usize = 0;
        for i in 0..tilted.len() {
            let row_score: usize = tilted.len() - i;
            for j in 0..tilted[i].len() {
                if tilted[i][j] == 'O' {
                    score += row_score;
                }
            }
        }
        score
    }

    fn tilt_cycle(&self, tilted: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut cycle: Vec<Vec<char>> = tilt_north(&tilted);
        cycle = tilt_west(&cycle);
        cycle = tilt_south(&cycle);
        cycle = tilt_east(&cycle);
        cycle
    }

}

impl AOCProblem for Day14 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let row: Vec<char> = line.chars().collect();
        self.board.push(row);
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        print_board(&self.board);
        let tilted = tilt_north(&self.board);
        print_board(&tilted);
        println!("Board Size: {} x {}", self.board.len(), self.board[0].len());
        self.score_north(tilted).to_string()
    }

    fn compute_b(&mut self) -> String {
        print_board(&self.board);
        // Board state hash -> cycle number when we saw that board
        let mut cache: HashMap<u64, u64> = HashMap::new();
        let mut tilted = self.board.clone();
        let mut after_cycle: u64 = 0;
        let mut i = 0;
        while after_cycle == 0 {
            tilted = self.tilt_cycle(tilted);
            i += 1;
            let h = board_hash_str(&tilted);
            if let Some(first_cycle) = cache.get(&h) {
                println!("{} FOUND CYCLE at {}", i, *first_cycle);
                print_board(&tilted);
                let cycle_len = i - *first_cycle;
                let togo = 1000000000 - i;
                let rem = togo / cycle_len;
                println!("Cycle len {} Iterations to go after cycles {}", cycle_len, rem);
                after_cycle = togo - (rem * cycle_len);
            } else {
                cache.insert(h, i);
            }
            if i % 1000 == 0 {
                println!("Cycle: {}", i);
            }
            //print_board(&tilted);
        }
        println!("After Cycle left to go {}", after_cycle);
        for _ in 0..after_cycle {
            tilted = self.tilt_cycle(tilted);
        }
        print_board(&tilted);
        self.score_north(tilted).to_string()
    }

}
