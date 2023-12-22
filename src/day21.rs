use crate::{Config, AOCProblem};

pub struct Day21 {
    board: Vec<Vec<char>>,
}

fn _print_board(board: &Vec<Vec<char>>) {
    println!("Board:\n");
    for i in 0..board.len() {
        let foo = &board[i].iter().collect::<String>();
        println!("{:?}", foo);
    }
}

impl Day21 {
    pub fn new() -> Day21 {
        Day21 {
            board: Vec::new(),
        }
    }

    fn update_board(&mut self) {
        // Update positions we can step on
        let mut new_stepped: Vec<(usize, usize)> = Vec::new();

        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] == 'O' || self.board[i][j] == 'S' {
                    if i > 0 && self.board[i-1][j] != '#' { 
                        new_stepped.push((i-1,j));
                    }
                    if i < self.board.len() - 1 && self.board[i+1][j] != '#' {
                        new_stepped.push((i+1,j));
                    }
                    if j > 0 && self.board[i][j-1] != '#' {
                        new_stepped.push((i, j-1));
                    }
                    if j < self.board[i].len() - 1 && self.board[i][j+1] != '#' {
                        new_stepped.push((i, j+1));
                    }
                    self.board[i][j] = '.';
                }
            }
        }

        for new_pos in new_stepped {
            self.board[new_pos.0][new_pos.1] = 'O';
        }
    }

    fn count_stepped(&self) -> usize {
        let mut count = 0;
        for i in 0..self.board.len() {
            count += (&self.board[i]).iter().filter(|&n| *n == 'O').count();
        }
        count
    }

}

impl AOCProblem for Day21 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let row: Vec<char> = line.chars().collect();
        self.board.push(row);
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        //print_board(&self.board);
        for i in 0..350 {
            self.update_board();
            let count = self.count_stepped();
            //print_board(&self.board);
            println!("Step: {} Squares: {}", i, count);
        }
        self.count_stepped().to_string()
    }

    fn compute_b(&mut self) -> String {
        // Will need to experiment with the details of the input
        // After Step 131 we enter a cycle on the single board, alternating between 7509 and 7566 squares
        // There's no obstacle straight from S in any direction
        //   So after X steps, we stepped on a square X out in each direction from S
        //   S is in the middle, 65 steps from any end
        // (26501365 - 65) / 131 = 202,300
        //   We've walked 202,300 boards in each direction
        //     This forms a diamond
        //            O
        //           OOO
        //          OOOOO 
        //           OOO
        //            O
        //  Size of the diamond is (2N - 1) + 2(2N - 3) + 2(2N - 5) ...
        // Could lookup the formula or solve, but lets just compute
        let n = 202301;
        let mut boards: i64 = (2*n) - 1;
        let mut odd = 1;
        for _ in 0..n-1 {
            odd += 2;
            boards += 2 * ((2*n) - odd);
        }
        // 81850984601 boards in the diamond
        // How many boards are in one state vs the other?
        // Parity flips when we move to the next board
        //               O
        //        E     OEO
        //       EOE   OEOEO
        //        E     OEO
        //               O
        // N^2 odd, (N-1)^2 even
        
        // Some boards are incomplete though
        // Test board on the perimeter
        // Board on the left, we started at the right most center position (straight from S)
        // How far do we get in 131 steps
        let orig_board = self.board.clone();
        self.board[65][65] = '.';
        self.board[65][130] = 'S';
        for _ in 0..131 {
            self.update_board();
        }

        println!("Left board in {} steps = {} count", 131, self.count_stepped());
        self.board[65][65] = '.';
        self.board[130][65] = 'S';
        self.board[65][130] = 'S';
        for _ in 0..131 {
            self.update_board();
        }
        println!("Left board 2 in {} steps = {} count", 131, self.count_stepped() / 2);

        // Now Right board
        self.board = orig_board.clone();
        self.board[65][65] = '.';
        self.board[65][0] = 'S';
        for _ in 0..131 {
            self.update_board();
        }
        println!("Right board in {} steps = {} count", 131, self.count_stepped());

        // Down
        self.board = orig_board.clone();
        self.board[65][65] = '.';
        self.board[0][65] = 'S';
        for _ in 0..131 {
            self.update_board();
        }
        println!("Down board in {} steps = {} count", 131, self.count_stepped());

        // Up
        self.board = orig_board.clone();
        self.board[65][65] = '.';
        self.board[130][65] = 'S';
        for _ in 0..131 {
            self.update_board();
        }
        println!("Up board in {} steps = {} count", 131, self.count_stepped());

        // Corners, we take half the steps to get to the corner board, then only have half the steps
        self.board = orig_board.clone();
        self.board[65][65] = '.';
        self.board[0][0] = 'S';
        for _ in 0..65 {
            self.update_board();
        }
        let se_count = self.count_stepped();
        println!("SE board in {} steps = {} count", 65, se_count);

        self.board = orig_board.clone();
        self.board[65][65] = '.';
        self.board[0][130] = 'S';
        for _ in 0..65 {
            self.update_board();
        }
        let sw_count = self.count_stepped();
        println!("SW board in {} steps = {} count", 65, sw_count);
        self.board = orig_board.clone();
        self.board[65][65] = '.';
        self.board[130][0] = 'S';
        for _ in 0..65 {
            self.update_board();
        }
        let ne_count = self.count_stepped();
        println!("NE board in {} steps = {} count", 65, ne_count);

        self.board = orig_board.clone();
        self.board[65][65] = '.';
        self.board[130][130] = 'S';
        for _ in 0..65 {
            self.update_board();
        }
        let nw_count = self.count_stepped();
        println!("NW board in {} steps = {} count", 65, nw_count);

        // Full boards, ignore the perimeter
        let odd_boards = (n - 1) * (n - 1);
        let even_boards = (n - 2) * (n - 2);

        let total_interior_boards = odd_boards + even_boards;
        let perimeter_boards = (boards - total_interior_boards) / 4;
        let b1 = even_boards * 7509;
        let b2: i64 = odd_boards * 7566;

        // These counts are all off by some: TBD figure out
        //  The simulations above are likely wrong
        //  Instead, simulate entirely the n=2 version and check each square

        let b3 = (perimeter_boards * 5781) + (perimeter_boards * 5793) + (perimeter_boards * 5793) + (perimeter_boards * 5781);
        // Finally add the corner boards, looks like there's n-1 corner boards of each type
        let corner_count = n - 1;

        let b4: i64 = corner_count * se_count as i64;
        let b5: i64 = corner_count * sw_count as i64;
        let b6: i64 = corner_count * ne_count as i64;
        let b7: i64 = corner_count * nw_count as i64;

        println!("Boards: {}  Odd Boards {} Even Boards {} Total Interior Boards {} Total Perimeter {} Total {}",
            boards, odd_boards, even_boards, total_interior_boards, perimeter_boards, total_interior_boards + perimeter_boards);
        println!("Corners: {} SE {} SW {} NE {} NW {}", corner_count, b4,b5,b6,b7);
        let corners = b4+b5+b6+b7;
        println!("Corners: {}", corners);

        let ans = b1 + b2 + b3 + b4 + b5 + b6 + b7;
        let diff: i64 = ans - 616951804315987;
        println!("diff: {}", diff);
        ans.to_string()
    }
}
