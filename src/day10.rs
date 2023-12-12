use crate::{Config, AOCProblem};
use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Cardinal { EAST, WEST, NORTH, SOUTH }

// Position along a pipeline, we're at loc and we came into this pipe section from "came_from"
// Steps counts how long we've walked
#[derive(Debug)]
struct PipePath {
    loc: Location,
    came_from: Cardinal,
    steps: i32,
}

pub struct Day10 {
    board: Vec<Vec<char>>,  // The gameboard, chars are the pipes we read in
    scores: Vec<Vec<i16>>,  // Game board scores, values are the distances we compute
    start: Location,
    farthest_distance: i32,
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 {
            board: Vec::new(),
            scores: Vec::new(),
            start: Location {x: 0, y: 0},
            farthest_distance: 0,
        }
    }

    // What is at this location on the board?
    fn get_symbol(&self, loc: &Location) -> char {
        return self.board[loc.y][loc.x];
    }

    fn set_symbol(&mut self, loc: &Location, val: char) {
        self.board[loc.y][loc.x] = val;
    }

    fn get_score(&self, loc: &Location) -> i16 {
        return self.scores[loc.y][loc.x];
    }

    // Where do I go next from this pipe path?
    fn traverse(&self, cur: &PipePath) -> Option<PipePath> {
        let pipe = self.get_symbol(&cur.loc);
        match pipe {
            '|' => { 
                match cur.came_from {
                    Cardinal::NORTH => Some(PipePath{loc: Location{x: cur.loc.x, y: cur.loc.y + 1}, came_from: Cardinal::NORTH, steps: cur.steps + 1}),
                    Cardinal::SOUTH => Some(PipePath{loc: Location{x: cur.loc.x, y: cur.loc.y - 1}, came_from: Cardinal::SOUTH, steps: cur.steps + 1}),
                    _ => None,
                }
            }           
            '-' => { 
                match cur.came_from {
                    Cardinal::WEST => Some(PipePath{loc: Location{x: cur.loc.x + 1, y: cur.loc.y}, came_from: Cardinal::WEST, steps: cur.steps + 1}),
                    Cardinal::EAST => Some(PipePath{loc: Location{x: cur.loc.x - 1, y: cur.loc.y}, came_from: Cardinal::EAST, steps: cur.steps + 1}),
                    _ => None,
                }
            } 
            'L' => { 
                match cur.came_from {
                    Cardinal::NORTH => Some(PipePath{loc: Location{x: cur.loc.x + 1, y: cur.loc.y}, came_from: Cardinal::WEST, steps: cur.steps + 1}),
                    Cardinal::EAST => Some(PipePath{loc: Location{x: cur.loc.x, y: cur.loc.y - 1}, came_from: Cardinal::SOUTH, steps: cur.steps + 1}),
                    _ => None,
                }
            } 
            'J' => { 
                match cur.came_from {
                    Cardinal::NORTH => Some(PipePath{loc: Location{x: cur.loc.x - 1, y: cur.loc.y}, came_from: Cardinal::EAST, steps: cur.steps + 1}),
                    Cardinal::WEST => Some(PipePath{loc: Location{x: cur.loc.x, y: cur.loc.y - 1}, came_from: Cardinal::SOUTH, steps: cur.steps + 1}),
                    _ => None,
                }
            } 
            '7' => { 
                match cur.came_from {
                    Cardinal::SOUTH => Some(PipePath{loc: Location{x: cur.loc.x - 1, y: cur.loc.y}, came_from: Cardinal::EAST, steps: cur.steps + 1}),
                    Cardinal::WEST => Some(PipePath{loc: Location{x: cur.loc.x, y: cur.loc.y + 1}, came_from: Cardinal::NORTH, steps: cur.steps + 1}),
                    _ => None,
                }
            }
            'F' => { 
                match cur.came_from {
                    Cardinal::SOUTH => Some(PipePath{loc: Location{x: cur.loc.x + 1, y: cur.loc.y}, came_from: Cardinal::WEST, steps: cur.steps + 1}),
                    Cardinal::EAST => Some(PipePath{loc: Location{x: cur.loc.x, y: cur.loc.y + 1}, came_from: Cardinal::NORTH, steps: cur.steps + 1}),
                    _ => None,
                }
            }
            '.' => { println!("Found empty ground!"); None},
            'S' => { println!("Found Starting symbol"); None},
            _ => { println!("Found unknown symbol {}", pipe); None},
        }
    }

    fn initialize_scores(&mut self) {
        let row_len = self.board.get(0).unwrap().len();
        for _ in 0..self.board.len() {
            let score_row = vec![0; row_len];
            self.scores.push(score_row);
        }
    }

    fn set_score(&mut self, path: &PipePath) -> bool {
        let cur_score = self.scores[path.loc.y][path.loc.x];
        if cur_score > 0 && cur_score <= path.steps as i16 {
            return false;
        }
        if path.steps > self.farthest_distance {
            self.farthest_distance = path.steps;
        }
        self.scores[path.loc.y][path.loc.x] = path.steps as i16;
        true
    }

    fn print_scores(&self) {
        println!("Score Map:\n");
        for i in 0..self.scores.len() {
            println!("{:?}", self.scores[i]);
        }
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
    
    // Display only pipes on the main loop
    fn print_loop(&self) {
        println!("Loop\n");
        for i in 0..self.scores.len() {
            let mut loop_str = String::new();
            for j in 0..self.scores[i].len() {
                let loc = Location{x: j, y:i};
                if loc == self.start || self.get_score(&loc) > 0 {
                    loop_str.push(self.get_symbol(&loc));
                } else {
                    loop_str.push('.');
                }
            }
            println!("{:?}", loop_str);
        }
    }

    fn get_start_pipepaths(&mut self) -> VecDeque<PipePath> {
        let mut pipe_paths = VecDeque::new();
        // Flags to tell whether we can move north/south/east/west from start
        // Used later to decide what piece S is really
        let mut m_north: bool = false;
        let mut m_south: bool = false;
        let mut m_east: bool = false;
        let mut m_west: bool = false;
        // Can we move north from the start? (Does the pipe to the north connect south?)
        if let Some(loc_north) = self.go_north(&self.start) {
             match self.get_symbol(&loc_north) {
                '|' | '7' | 'F' => {
                    m_north = true;
                    pipe_paths.push_front(PipePath{loc: loc_north, came_from: Cardinal::SOUTH, steps: 1});
                }
                _ => (),
            }
        }
        // South?
        if let Some(loc_south) = self.go_south(&self.start) {
            match self.get_symbol(&loc_south) {
                '|' | 'L' | 'J' => { m_south = true; pipe_paths.push_front(PipePath{loc: loc_south, came_from: Cardinal::NORTH, steps: 1})},
                _ => (),
            }
        }
        // West?
        if let Some(loc_west) = self.go_west(&self.start) {
            match self.get_symbol(&loc_west) {
                '-' | 'L' | 'F' => { m_west = true; pipe_paths.push_front(PipePath{loc: loc_west, came_from: Cardinal::EAST, steps: 1})},
                _ => (),
            }
        }
        // And east
        if let Some(loc_east) = self.go_east(&self.start) {
            match self.get_symbol(&loc_east) {
                '-' | 'J' | '7' => {m_east = true; pipe_paths.push_front(PipePath{loc: loc_east, came_from: Cardinal::WEST, steps: 1})},
                _ => (),
            }
        }
        for path in &pipe_paths {
            self.set_score(&path);
        }
        // Figure out which pipe S is, put it on the board
        if m_north && m_south {
            self.board[self.start.y][self.start.x] = '|';
        } else if m_north && m_east {
            self.board[self.start.y][self.start.x] = 'L';
        } else if m_north && m_west {
            self.board[self.start.y][self.start.x] = 'J';
        } else if m_south && m_east {
            self.board[self.start.y][self.start.x] = 'F';
        } else if m_south && m_west {
            self.board[self.start.y][self.start.x] = '7';
        } else if m_east && m_west {
            self.board[self.start.y][self.start.x] = '-';
        } else {
            eprintln!("Can't figure out what S is!");
        }
        pipe_paths
    }

    fn go_north(&self, loc: &Location) -> Option<Location> { 
        if loc.y > 0 { Some(Location { x: loc.x, y: loc.y - 1 }) } else { None }
    }
    
    fn go_south(&self, loc: &Location) -> Option<Location> { 
        if loc.y < (self.board.len() - 1) { Some(Location { x: loc.x, y: loc.y + 1} ) } else { None }
    }
    
    fn go_east(&self, loc: &Location) -> Option<Location> { 
        let width = self.board[loc.y].len();
        if loc.x < width - 1 { Some(Location { x: loc.x + 1, y: loc.y} )} else { None }
    }

    fn go_west(&self, loc: &Location) -> Option<Location> { 
        if loc.x > 0 { Some(Location { x: loc.x - 1, y: loc.y} ) } else { None }
    }

    // If we're moving into loc from the west, does this cross a pipeline boundary
    //   This will be used by the line-crossing "is inside polygon" algorithm
    //   Draw a line from west to east, count how many times that line crosses the shape boundary
    // From the left, we need to decide if there's an intersection or not
    //   A pipe that is horizontal doesn't intersect a line from the left
    //   For a horizontal segment, it crosses the pipe boundery if the corners are opposite (north + south)
    //                                                      |
    // ->  +--+   Does not cross into the polygon    ->  +--+  Does cross the polygon boundary
    //     |  |                                          |
    //  
    //   Instead of remembering the last corner, we can pick one direction (north or south) to count
    //    If both corners are the same (north or south), then we'll either count 0 or 2 crossings
    //    Either way we end up the same (outside or inside), no crossing or cross and cross back
    //    If they're different, then we'll count 1 crossing.
    //   Here we picked south, so F and 7 count as crossing.  Should work fine the other way as well
    fn does_cross_boundary_from_west(&self, pipe: char) -> bool {
        if pipe == '|' || pipe == 'F' || pipe == '7' { return true; }
        return false;
    }

    // Replace board characters with I if its enclosed inside the loop.
    // Replace with O if it's outside the loop
    fn create_in_out_board(&mut self) {
        println!("Creating Inside Pipeline Board:\n");
        // Start by replacing all non-loop places with O
        for i in 0..self.scores.len() {
            for j in 0..self.scores[i].len() {
                let loc = Location{x: j, y:i};
                if loc != self.start && self.get_score(&loc) == 0 {
                    self.board[i][j] = 'O';
                }
            }
        }
        for i in 0..self.board.len() {
            let mut inside: bool = false;
            for j in 0..self.board[i].len() {
                let char = self.board[i][j];
                if self.does_cross_boundary_from_west(char) {
                    // Crossed a boundary, flip from inside to out, or vice versa
                    if inside { inside = false } else { inside = true };
                }
                if inside == true && self.scores[i][j] == 0 && (self.start.y != i || self.start.x != j) {
                    // Inside the pipeline, and not part of the pipeline
                    // Start position is part of the pipeline, and has score 0 so check for that explicitly
                    self.set_symbol(&Location{x: j, y: i}, 'I');
                }
            }
        }
    }

    fn count_inside_spaces(&mut self) -> i32 {
        let mut count = 0;
        for i in 0..self.board.len() {
            // Could do a filter/collect instead
            let row = &self.board[i];
            for j in 0..row.len() {
                if row[j] == 'I' {
                    count += 1;
                }   
            }
        }
        count
    }
    
}

impl AOCProblem for Day10 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let line_str: String = String::from(line);
        let row: Vec<char> = line_str.chars().collect();
        // Find is good here since we're assuming ascii strings only, no unicode
        //  So byte position = char index
        let spos = line.find('S');
        if let Some(sx) = spos {
            let sloc = Location {x: sx, y: self.board.len()};
            println!("Found start location at {:?}", sloc);
            self.start = sloc;
        }

        self.board.push(row);
    }
    
    fn compute_a(&mut self) -> String {
        self.initialize_scores();
        let mut paths: VecDeque<PipePath> = self.get_start_pipepaths();
        while let Some(path) = paths.pop_back() {
            println!("Current Pipe Path: {:?}", path);
            let next = self.traverse(&path);
            match next {
                None => { println!("Stuck at {:?}", &path); break; },
                Some(new_path) => {
                    if self.set_score(&new_path) {
                        paths.push_front(new_path);
                    } else {
                        println!("Done with path {:?}", path);
                    }
                }
            }
        }
        self.print_scores();
        return self.farthest_distance.to_string();
    }

    fn compute_b(&mut self) -> String {
        self.compute_a();
        self.print_loop();
        self.create_in_out_board();
        self.print_board();
        self.count_inside_spaces().to_string()
    }
}
