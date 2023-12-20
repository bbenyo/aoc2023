use crate::{Config, AOCProblem};
use pathfinding::prelude::astar;

#[derive(Debug, Eq, Hash, Clone, Copy)]
enum Cardinal { EAST, WEST, NORTH, SOUTH, NONE, ANY }

// ANY should equal any direction
//  We use this to match the goal state, the goal state doesn't care which direction we came from
//  Nodes need to remember which direction they came from, since you can't go backwards
impl PartialEq for Cardinal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Cardinal::ANY, _) => true,
            (_, &Cardinal::ANY) => true,
            (&Cardinal::NORTH, &Cardinal::NORTH) => true,
            (&Cardinal::SOUTH, &Cardinal::SOUTH) => true,
            (&Cardinal::EAST, &Cardinal::EAST) => true,
            (&Cardinal::WEST, &Cardinal::WEST) => true,
            (&Cardinal::NONE, &Cardinal::NONE) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
    came_from: Cardinal,
}

// A* node
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Node {
    pos: Location,
    cost: i32,
}

pub struct Day17 {
    board: Vec<Vec<u8>>,
    variant: bool,
}

fn print_board(board: &Vec<Vec<u8>>) {
    println!("Board:\n");
    for i in 0..board.len() {
        let mut board_str = String::new();
        let row = &board[i];
        for j in 0..row.len() {
            board_str.push_str(&row[j].to_string());
        } 
        println!("{:?}", board_str);
    }
}

impl Day17 {
    pub fn new() -> Day17 {
        Day17 {
            board: Vec::new(),
            variant: false,
        }
    }

    // A*: Get nodes we can move to from here, and where we came from
    fn get_successors(&self, pos: &Location) -> Vec<Node> {
        let mut next_nodes: Vec<Node> = Vec::new();
        
        let min_steps: usize;
        let max_steps: usize;
        // Part a: we can move 1-3
        // part b: we can move 4-10
        match self.variant {
            false => { min_steps = 1; max_steps = 3},
            true => { min_steps = 4; max_steps = 10},
        }

        // Can go up 1-3
        // Can't go back, and can't go any farther in the old direction
        if pos.came_from != Cardinal::NORTH && pos.came_from != Cardinal::SOUTH {
            let mut cost: i32 = 0;
            let mut cost2: i32 = 0;

            let len = self.board.len();
            
            // Update cost for any spaces we have to move before we get to a possible next position
            if pos.y > min_steps-1 {
                for k in 0..min_steps-1 {
                    cost += self.board[pos.y - 1 - k][pos.x] as i32;
                }
            }

            if pos.y < len - 1 - (min_steps - 1) {
                for k in 0..min_steps-1 {
                    cost2 += self.board[pos.y + 1 + k][pos.x] as i32;
                }
            }

            for k in min_steps-1..max_steps {
                if pos.y > k {
                    let pos = Location{x: pos.x, y: pos.y - 1 - k, came_from: Cardinal::SOUTH};
                    cost += self.board[pos.y][pos.x] as i32;
                    next_nodes.push(Node{pos, cost});
                };
                if k + 1 < len && pos.y < len - (1 + k) {
                    let pos = Location{x: pos.x, y: pos.y + 1 + k, came_from: Cardinal::NORTH};
                    cost2 += self.board[pos.y][pos.x] as i32;
                    next_nodes.push(Node{pos, cost: cost2});
                }
            }
        }

        if pos.came_from != Cardinal::EAST && pos.came_from != Cardinal::WEST {
            let mut cost: i32 = 0;
            let mut cost2: i32 = 0;
            let rlen = self.board[0].len();

             // Update cost for any spaces we have to move before we get to a possible next position
             if pos.x > min_steps-1 {
                for k in 0..min_steps-1 {
                    cost += self.board[pos.y][pos.x - 1 - k] as i32;
                }
            }

            if pos.x < rlen - 1 - (min_steps - 1) {
                for k in 0..min_steps-1 {
                    cost2 += self.board[pos.y][pos.x + 1 + k] as i32;
                }
            }

            for k in min_steps-1..max_steps {
                if pos.x > k {
                    let pos = Location{x: pos.x - 1 - k, y: pos.y, came_from: Cardinal::EAST};
                    cost += self.board[pos.y][pos.x] as i32;
                    next_nodes.push(Node{pos, cost});
                };
                if k + 1 < rlen && pos.x < rlen - (1 + k) {
                    let pos = Location{x: pos.x + 1 + k, y: pos.y, came_from: Cardinal::WEST};
                    cost2 += self.board[pos.y][pos.x] as i32;
                    next_nodes.push(Node{pos, cost: cost2});
                }
            } 
        }
        next_nodes
    }

    fn search(&self) -> i32 {
        let start = Location{x: 0, y: 0, came_from: Cardinal::NONE};
        let row_count = self.board.len();
        let goal = Location{x: self.board[row_count-1].len() - 1, y: row_count - 1, came_from: Cardinal::ANY};
        let result = astar(
            &start,
            |p| self.get_successors(p).iter().map(|s| (s.pos, s.cost)).collect::<Vec<_>>(),
            |p| ((p.x as i32 - goal.x as i32).abs() + (p.y as i32 - goal.y as i32).abs()),
            |p| *p==goal);
        let result = result.expect("Can't get to the goal!!");
        for path in result.0 {
            println!("{:?}", path);
        }
        println!("total cost: {:}", result.1);
        result.1
    }
}

impl AOCProblem for Day17 {
    fn handle_line(&mut self, line: &str, config: &Config) {
        self.variant = config.variant;
        let c_row: Vec<char> = line.chars().collect();
        let mut row: Vec<u8> = Vec::new();
        for c in c_row {
            row.push(c.to_string().parse::<u8>().unwrap());
        }
        self.board.push(row);
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        print_board(&self.board);
        self.search().to_string()
    }

    fn compute_b(&mut self) -> String {
        return self.compute_a();
    }
}
