use crate::{Config, AOCProblem};

struct Pattern {
    board: Vec<Vec<char>>,
    horiz_reflect: Option<usize>,
    vert_reflect: Option<usize>,
}

impl Pattern {
    fn compute_vert_reflect(&mut self) -> bool {
        // Is there a vertical column reflection?
        for i in 0..self.board[0].len() - 1 {
            if let Some(orig) = self.vert_reflect {
                if orig == i {
                    // Already found a vert reflect here, must be on part b
                    // Keep looking
                    continue;
                }
            }
            if self.is_vert_reflect(i) {
                self.vert_reflect = Some(i);
                println!("Found Vertical reflection at {}", i);
                return true;
            }
        }
        return false;
    }

    // Is the line between col and col+1 a reflection?
    fn is_vert_reflect(&self, col: usize) -> bool {
        //println!("Checking vert reflect for {}", col);
        for i in 0..self.board[0].len() {
            if i > col {
                return true;
            }
            let left_col = col - i;
            let right_col = col + 1 + i;

            if right_col >= self.board[0].len() {
                return true;
            }
            for j in 0..self.board.len() {
                if self.board[j][left_col] != self.board[j][right_col] {
                    return false;
                }
            }
        }
        return true;
    }

    fn compute_horiz_reflect(&mut self) -> bool {
        // Is there a horizontal row reflection?
        for i in 0..self.board.len() - 1 {
            if let Some(orig) = self.horiz_reflect {
                if orig == i {
                    // already found this one, keep looking, must be on part b
                    continue;
                }
            }
            if self.is_horiz_reflect(i) {
                self.horiz_reflect = Some(i);
                println!("Found Horizontal reflection at {}", i);
                return true;
            }
        }
        return false;
    }

    // Is the line between col and col+1 a reflection?
    fn is_horiz_reflect(&self, row: usize) -> bool {
        //println!("Checking horiz_reflect for {}", row);
        for i in 0..self.board.len() {
            if i > row {
                return true;
            }
            let up_row: usize = row - i;
            let down_row = row + 1 + i;
            if down_row >= self.board.len() {
                return true;
            }
            for j in 0..self.board[0].len() {
                if self.board[up_row][j] != self.board[down_row][j] {
                    return false;
                }
            }
        }
        return true;
    }

    fn print_board(&self) {
        println!("Board:\n");
        for i in 0..self.board.len() {
            let mut board_str = String::new();
            let row = &self.board[i];
            for j in 0..row.len() {
                board_str.push(row[j]);
            }   
            println!("{:?}", board_str);
        }
    }

    // Part B, let's brute force it
    // Likely a way to be smarter about which rows/columns to try
    //    But brute forcing doesn't seem so bad, there's no exponential growth
    fn find_smudge(&mut self) -> bool {
        let orig_horiz_reflect = self.horiz_reflect;
        let orig_vert_reflect = self.vert_reflect;

        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                //println!("Trying smudge at {},{}", i,j);
                if self.board[i][j] == '.' {
                    self.board[i][j] = '#';
                } else {
                    self.board[i][j] = '.';
                }
                if self.compute_vert_reflect() {
                    if self.vert_reflect != orig_vert_reflect {
                        self.horiz_reflect = None;
                        println!("Found differnet vert reflection with smudge at {} {}: {:?} {:?}", i,j, self.vert_reflect, self.horiz_reflect);
                        return true;
                    } 
                }
                if self.compute_horiz_reflect() {
                    if self.horiz_reflect != orig_horiz_reflect && self.horiz_reflect != None {
                        self.vert_reflect = None;
                        println!("Found differnet horiz reflection with smudge at {} {}: {:?} {:?}", i,j, self.vert_reflect, self.horiz_reflect);
                        return true;
                    }
                }
                // Reset the changes
                self.horiz_reflect = orig_horiz_reflect;
                self.vert_reflect = orig_vert_reflect;
                if self.board[i][j] == '.' {
                    self.board[i][j] = '#';
                } else {
                    self.board[i][j] = '.';
                }
            }
        }
        return false;
    }
}

pub struct Day13 {
    patterns: Vec<Pattern>,
    cur_pattern: Pattern,
}

impl Day13 {
    pub fn new() -> Day13 {
        Day13 {
            patterns: Vec::new(),
            cur_pattern: Pattern {board: Vec::new(), horiz_reflect: None, vert_reflect: None}
        }
    }
    
    fn compute_score(&mut self) -> usize {
        let mut rows = 0;
        let mut cols = 0;
        for pattern in &mut self.patterns {
            if let Some(row_reflect) = pattern.horiz_reflect {
                rows += row_reflect + 1;
            } else if let Some(col_reflect) = pattern.vert_reflect {
                cols += col_reflect + 1;
            } else {
                eprintln!("NO reflection found!");
            }
        }
        cols + (100 * rows)
    }
    
}

impl AOCProblem for Day13 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let line_str: String = String::from(line);
        if line_str.len() == 0 {
            // Complete the latest pattern
            let new_pattern = Pattern {board: Vec::new(), horiz_reflect: None, vert_reflect: None};
            self.patterns.push(std::mem::replace(&mut self.cur_pattern, new_pattern));
            return;
        }
        let row: Vec<char> = line_str.chars().collect();
        self.cur_pattern.board.push(row);
    }

    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        let new_pattern = Pattern {board: Vec::new(), horiz_reflect: None, vert_reflect: None};
        self.patterns.push(std::mem::replace(&mut self.cur_pattern, new_pattern));
            
        let mut idx = 0;
        for pattern in &mut self.patterns {
            println!("Handling Pattern: {}", idx);
            pattern.print_board();
            pattern.compute_vert_reflect();
            if pattern.vert_reflect == None {
                pattern.compute_horiz_reflect();
            }
            idx += 1;
        }
        self.compute_score().to_string()
    }

    fn compute_b(&mut self) -> String {
        // Find the original reflections
        self.compute_a();
        let mut idx = 0;
        for pattern in &mut self.patterns {
            println!("Looking for smudge on pattern {}", idx);
            println!("\n Original reflection {:?} {:?}", pattern.horiz_reflect, pattern.vert_reflect);
            if !pattern.find_smudge() {
                pattern.print_board();
                panic!("Unable to find a smudge on this board!");
            }
            idx += 1;
        }
        self.compute_score().to_string()
    }
}
