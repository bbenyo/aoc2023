use crate::{Config, AOCProblem};

#[derive(Debug)]
enum Cardinal { EAST, WEST, NORTH, SOUTH }

/* Fill in the board with chars depicting how many beams in a square and what directions
 * . : No beam, empty
 * > : EAST
 * v : SOUTH
 * ^ : NORTH
 * < : WEST
 * J : WEST + NORTH
 * 7 : WEST + SOUTH
 * F : EAST + SOUTH
 * L : EAST + NORTH
 * - : EAST + WEST
 * | : SOUTH + NORTH
 * N : EAST + SOUTH + WEST
 * S : EAST + NORTH + WEST
 * E : WEST + NORTH + SOUTH
 * W : EAST + NORTH + SOUTH
 * + : ALL directions
 * 
 * If a beam going DIR enters a square with a beam already in that direction, we're done
 *   They add together, but we don't care, we've already calculated it
 * 
 * Energized fills in the board with the above chars
 */
pub struct Day16 {
    board: Vec<Vec<char>>,
    energized: Vec<Vec<char>>,
}

fn print_board(board: &Vec<Vec<char>>) {
    println!("Board:\n");
    for i in 0..board.len() {
        let foo = &board[i].iter().collect::<String>();
        println!("{:?}", foo);
    }
}

#[derive(Debug)]
struct Beam {
    x: usize,
    y: usize,
    dir: Cardinal,
}

impl Day16 {
    pub fn new() -> Day16 {
        Day16 {
            board: Vec::new(),
            energized: Vec::new(),
        }
    }

    fn init_energized(&mut self) {
        self.energized.clear();
        for i in 0..self.board.len() {
            let mut row = Vec::new();
            for _ in 0..self.board[i].len() {
                row.push('.');
            }
            self.energized.push(row);
        }
    }

    // NORTH, SOUTH, EAST, WEST (is there a beam) -> character
    fn get_symbol(&self, dirs: (bool, bool, bool, bool)) -> char {
        match dirs {
            (false, false, false, false) => '.',
            (true, false, false, false) => '^',
            (false, true, false, false) => 'v',
            (false, false, true, false) => '>',
            (false, false, false, true) => '<',
            (true, false, false, true) => 'J',
            (false, true, false, true) => '7',
            (false, true, true, false) => 'F',
            (true, false, true, false) => 'L',
            (true, true, false, false) => '|',
            (false, false, true, true) => '-',
            (false, true, true, true) => 'N',
            (true, false, true, true) => 'S',
            (true, true, false, true) => 'E',
            (true, true, true, false) => 'W',
            (true, true, true, true) => '+',
        }
    }

    // char -> NORTH, SOUTH, EAST, WEST (is there a beam)
    fn get_dirs(&self, beam: char) -> (bool, bool, bool, bool) {
        match beam {
            '.' => (false, false, false, false),
            '^' => (true, false, false, false),
            'v' => (false, true, false, false),
            '>' => (false, false, true, false),
            '<' => (false, false, false, true),
            'J' => (true, false, false, true),
            '7' => (false, true, false, true),
            'F' => (false, true, true, false),
            'L' => (true, false, true, false),
            '|' => (true, true, false, false),
            '-' => (false, false, true, true),
            'N' => (false, true, true, true),
            'S' => (true, false, true, true),
            'W' => (true, true, true, false),
            'E' => (true, true, false, true),
            '+' => (true, true, true, true),
            _ => { eprintln!("Unrecognized char {}", beam); (false, false, false, false)},
        }
    }

    fn forward_beam(&self, b: Beam) -> Option<Beam> {
        let mut by: usize = b.y;
        let mut bx: usize = b.x;
        match b.dir {
            Cardinal::EAST => bx = bx + 1,
            Cardinal::WEST => if bx > 0 { bx = bx - 1;} else {return None;},
            Cardinal::NORTH => if by > 0 { by = by - 1;} else {return None;},
            Cardinal::SOUTH => by = by + 1,
        }
        if by >= self.board.len() { return None; };
        if bx >= self.board[by].len() { return None; };

        Some(Beam{x: bx, y: by, dir: b.dir})
    }

    // Return the propagated beam, or NONE if we hit a wall or another beam in the same dir
    fn propagate_beam(&mut self, b: Beam) -> (Option<Beam>, Option<Beam>) {
        let bx = b.x;
        let by = b.y;
        let mirror = self.board[by][bx];
        let ener = self.energized[by][bx];
        // Did we merge with a beam already going this direction?
        let mydir: Vec<char>;
        match &b.dir {
            Cardinal::NORTH => mydir = vec!['^','J','L','|','S','E','W','+'],
            Cardinal::SOUTH => mydir = vec!['v','7','F','|','N','E','W','+'],
            Cardinal::EAST => mydir = vec!['>','F','L','-','S','N','W','+'],
            Cardinal::WEST => mydir = vec!['<','7','J','-','S','N','E','+'],
        }
        // Already had a beam going that way there
        if mydir.contains(&ener) { return (None, None); }

        let mut dirs = self.get_dirs(ener);
        match &b.dir {
            Cardinal::NORTH => dirs.0 = true,
            Cardinal::SOUTH => dirs.1 = true,
            Cardinal::EAST => dirs.2 = true,
            Cardinal::WEST => dirs.3 = true,
        }
        let new_ener = self.get_symbol(dirs);
        self.energized[by][bx] = new_ener;
        // Check what happens to the beam
        let mut b2 = Beam{x: bx, y: by, dir: b.dir};
        match mirror {
            '.' => (Some(b2), None),
            '/' => {
                match b2.dir {
                    Cardinal::EAST => { b2.dir = Cardinal::NORTH; (Some(b2), None)},
                    Cardinal::NORTH => { b2.dir = Cardinal::EAST; (Some(b2), None)},
                    Cardinal::SOUTH => { b2.dir = Cardinal::WEST; (Some(b2), None)},
                    Cardinal::WEST => { b2.dir = Cardinal::SOUTH; (Some(b2), None)},
                }
            },
            '\\' => {
                match b2.dir {
                    Cardinal::EAST => { b2.dir = Cardinal::SOUTH; (Some(b2), None)},
                    Cardinal::NORTH => { b2.dir = Cardinal::WEST; (Some(b2), None)},
                    Cardinal::SOUTH => { b2.dir = Cardinal::EAST; (Some(b2), None)},
                    Cardinal::WEST => { b2.dir = Cardinal::NORTH; (Some(b2), None)},
                }
            },
            '|' => {
                match b2.dir {
                    Cardinal::EAST | Cardinal::WEST => {
                        let b3 = Beam{x: bx, y: by, dir: Cardinal::NORTH};
                        b2.dir = Cardinal::SOUTH;
                        (Some(b2), Some(b3))
                    },
                    _ => (Some(b2), None),
                }
            },
            '-' => {
                match b2.dir {
                    Cardinal::NORTH | Cardinal::SOUTH => {
                        let b3 = Beam{x: bx, y: by, dir: Cardinal::WEST};
                        b2.dir = Cardinal::EAST;
                        (Some(b2), Some(b3))
                    },
                    _ => (Some(b2), None),
                }
            }
            _ => (None, None),
        }
    }

    fn count_energized(&self) -> usize {
        let mut count = 0;
        for i in 0..self.energized.len() {
            count += (&self.energized[i]).iter().filter(|&n| *n != '.').count();
        }
        count
    }

    fn propagate(&mut self, beam: Beam) {
        let mut work_list: Vec<Beam> = Vec::new();
        work_list.push(beam);
        while let Some(beam) = work_list.pop() {
            let nbeams = self.propagate_beam(beam);
            if let Some(b1) = nbeams.0 {
                let b2 = self.forward_beam(b1);
                if let Some(good_beam) = b2 {
                    work_list.push(good_beam);
                }
            }
            if let Some(b1) = nbeams.1 {
                let b2 = self.forward_beam(b1);
                if let Some(good_beam) = b2 {
                    work_list.push(good_beam);
                }
            }
        }
    }
}

impl AOCProblem for Day16 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let line_str: String = String::from(line);
        let row: Vec<char> = line_str.chars().collect();
        self.board.push(row);
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        self.init_energized();
        print_board(&self.board);
        let start_beam = Beam{x: 0, y: 0, dir: Cardinal::EAST};
        self.propagate(start_beam);        
        print_board(&self.energized);        
        self.count_energized().to_string()
    }

    fn compute_b(&mut self) -> String {
        // Start EAST
        let mut max = 0;
        for i in 0..self.board.len() {
            self.init_energized();
            let start_beam = Beam{x: 0, y: i, dir: Cardinal::EAST};
            self.propagate(start_beam);
            let e = self.count_energized();
            if e > max { println!("New Max {} from row {} EAST", e, i); max = e; };

            // And WEST
            self.init_energized();
            let start_beam = Beam{x: self.board[i].len() - 1, y: i, dir: Cardinal::WEST};
            self.propagate(start_beam);
            let e = self.count_energized();
            if e > max { println!("New Max {} from row {} WEST", e, i); max = e; };
        }

        // SOUTH
        for i in 0..self.board[0].len() {
            self.init_energized();
            let start_beam = Beam{x: i, y: 0, dir: Cardinal::SOUTH};
            self.propagate(start_beam);
            let e = self.count_energized();
            if e > max { println!("New Max {} from col {} SOUTH", e, i); max = e; };

            // And NORTH
            self.init_energized();
            let start_beam = Beam{x: i, y: self.board[i].len() - 1, dir: Cardinal::NORTH};
            self.propagate(start_beam);
            let e = self.count_energized();
            if e > max { println!("New Max {} from col {} NORTH", e, i); max = e; };
        }
        return max.to_string();
    }
}
