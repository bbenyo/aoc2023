use crate::{Config, AOCProblem};

#[derive(Debug)]
struct Star {
    x: usize,
    y: usize,
    index: usize,
}

impl Star {
    fn distance_to(&self, star: &Star) -> i32 {
        // Manhattan distance = |x1 - x2| + |y1 - y2|
        return (self.x as i32 - star.x as i32).abs() + (self.y as i32 - star.y as i32).abs();
    }
}

pub struct Day11 {
    board: Vec<Vec<char>>,
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>,
    stars: Vec<Star>,
    expansion_coefficient: usize,
}

impl Day11 {
    pub fn new() -> Day11 {
        Day11 {
            board: Vec::new(),
            expanded_rows: Vec::new(),
            expanded_cols: Vec::new(),
            stars: Vec::new(),
            expansion_coefficient: 1,
        }
    }

    // Given x/y, get the position in the expanded galaxy
    // Count how many expanded rows/cols we pass going from 0 to x and 0 to y
    fn get_expanded_star(&self, star: &Star) -> Star {
        let mut e_x = star.x;
        let mut e_y = star.y;
        for e_row in &self.expanded_rows {
            // Sorted order, so once we hit x, we can stop
            if e_row < &star.y { e_y += &self.expansion_coefficient } else { break; }
        }

        for e_col in &self.expanded_cols {
            if e_col < &star.x { e_x += &self.expansion_coefficient } else { break; }
        }

        Star { x: e_x, y: e_y, index: star.index }
    }

    fn expand_galaxy(&mut self) {
        // Expand every column with all . into two columns with all .
        // Expand every row with all . into two rows with all .
        for i in 0..self.board.len() {
            let mut all_empty = true;
            for j in 0..self.board[i].len() {
                if self.board[i][j] != '.' {
                    all_empty = false;
                    break;
                }
            }
            if all_empty {
                self.expanded_rows.push(i);
            }
        }

        for i in 0..self.board[0].len() {
            let mut all_empty = true;
            for j in 0..self.board.len() {
                if self.board[j][i] != '.' {
                    all_empty = false;
                    break;
                }
            }
            if all_empty {
                self.expanded_cols.push(i);
            }
        }

        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] == '#' {
                    let star = Star {x: j, y: i, index: self.stars.len() + 1 };
                    let e_star = self.get_expanded_star(&star);
                    self.stars.push(e_star);
                }
            }
        }
    }

    fn print_expanded_galaxy(&self) {
        println!("Expanded Rows: {:?}", self.expanded_rows);
        println!("Expanded Cols: {:?}", self.expanded_cols);
    }

    fn print_galaxy(&self) {
        println!("Galaxy:\n");
        for i in 0..self.board.len() {
            let mut board_str = String::new();
            let row = &self.board[i];
            for j in 0..row.len() {
                board_str.push(row[j]);
            }   
            println!("{:?}", board_str);
        }
    }
}

impl AOCProblem for Day11 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let line_str: String = String::from(line);
        let row: Vec<char> = line_str.chars().collect();
        self.board.push(row);
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        self.expand_galaxy();
        self.print_galaxy();
        self.print_expanded_galaxy();
        let mut dist_sum: i64 = 0;
        for i in 0..self.stars.len() {
            let star = self.stars.get(i).unwrap();
            for j in (i+1)..self.stars.len() {
                let star2 = self.stars.get(j).unwrap();
                println!("Star1 {:?} Star2 {:?}", &star, &star2);
                let dist = star.distance_to(star2);
                println!("Shortest distance between {} and {} = {}", i, j, dist);
                dist_sum += dist as i64;
            }
        }
        dist_sum.to_string()
    }

    fn compute_b(&mut self) -> String {
        // Coefficient is number of rows/cols to add - 1, since we keep the original        
        self.expansion_coefficient = 999999;
        return self.compute_a();
    }
}
