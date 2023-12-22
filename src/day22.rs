use crate::{Config, AOCProblem};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position {
    x: u32,
    y: u32,
    z: u32,
}

impl Position {
    fn new(cs_string: &str) -> Position {
        let mut s_split = cs_string.split(",");
        let x = s_split.next().unwrap().parse::<u32>().unwrap();
        let y = s_split.next().unwrap().parse::<u32>().unwrap();
        let z = s_split.next().unwrap().parse::<u32>().unwrap();
        Position {x,y,z}
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Brick {
    start: Position,
    end: Position,
    index: usize,
}

impl Brick {
    fn lowest_z(&self) -> u32 {
        if self.start.z < self.end.z {
            self.start.z
        } else {
            self.end.z
        }
    }

    // New brick down 1
    fn drop_1(&self) -> Brick {
        if self.start.z > 1 && self.end.z > 1 {
            Brick{start: Position{x: self.start.x, y: self.start.y, z: self.start.z - 1},
                  end: Position{x: self.end.x, y: self.end.y, z: self.end.z - 1}, index: self.index}
        } else {
            self.clone()
        }
    }

    fn does_overlap(&self, other: &Brick) -> bool {
        let lx = std::cmp::min(self.start.x, self.end.x);
        let ux = std::cmp::max(self.start.x, self.end.x);
        let ly = std::cmp::min(self.start.y, self.end.y);
        let uy = std::cmp::max(self.start.y, self.end.y);
        let lz = std::cmp::min(self.start.z, self.end.z);
        let uz = std::cmp::max(self.start.z, self.end.z);

        let lx2 = std::cmp::min(other.start.x, other.end.x);
        let ux2 = std::cmp::max(other.start.x, other.end.x);
        let ly2 = std::cmp::min(other.start.y, other.end.y);
        let uy2 = std::cmp::max(other.start.y, other.end.y);
        let lz2 = std::cmp::min(other.start.z, other.end.z);
        let uz2 = std::cmp::max(other.start.z, other.end.z);

        if (lx <= ux2 && ux >= lx2) && (ly <= uy2 && uy >= ly2) && (lz <= uz2 && uz >= lz2) {
            return true;
        }
        return false;
    }

    // Does this brick fit with these bricks, or do any of them overlap
    //  Optionally give a brick in the lower_brick vec to ignore
    fn does_fit(&self, lower_bricks: &Vec<Brick>, ignore: Option<&Brick>) -> bool {
        for brick in lower_bricks {
            if let Some(b) = ignore {
                if brick.eq(b) {
                    continue;
                }
            }
            if self.index == brick.index {
                // Don't overlap myself
                continue;
            }
            if self.does_overlap(brick) {
                return false;
            }
        }
        return true;
    }

    fn drop_brick(&self, lower_bricks: &Vec<Brick>) -> Brick {
        // Try to drop the brick one by one, until we can't
        let mut cur_brick = self.clone();
        loop {
            let lz = cur_brick.lowest_z();
            if lz == 1 {
                // We're at the bottom
                return cur_brick;
            }
            let new_brick = cur_brick.drop_1();
            if !new_brick.does_fit(&lower_bricks, None) {
                // Can't go any lower, this is the bottom
                return cur_brick;
            }
            cur_brick = new_brick;
        }
    }

    // If brick is gone, can any of the other drop?
    fn safe_to_disintegrate(&self, bricks: &Vec<Brick>) -> bool {
        for brick in bricks {
            if brick.eq(self) {
                continue;
            }
            let n_brick = brick.drop_1();
            if n_brick.eq(brick) { 
                // It didn't actually drop
                continue;
            }
            if n_brick.does_fit(bricks, Some(&self)) {
                // No this brick can drop
                println!("Brick {} can drop if {} is removed", brick.index, self.index);
                return false;
            }
        }
        println!("No bricks can drop if {} is removed", self.index);
        return true;
    }
}

pub struct Day22 {
    items: Vec<Brick>,
}

impl Day22 {
    pub fn new() -> Day22 {
        Day22 {
            items: Vec::new(),
        }
    }
}

impl AOCProblem for Day22 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let mut line_iter = line.split("~");
        let start_str = line_iter.next().unwrap();
        let start = Position::new(start_str);
        let end_str = line_iter.next().unwrap();
        let end = Position::new(end_str);
        self.items.push(Brick{start, end, index: self.items.len()});
    }
    
    fn compute_a(&mut self) -> String {
        // Sort by lowest, we can ignore higher up bricks
        self.items.sort_by(|a,b| a.lowest_z().cmp(&b.lowest_z()));
        let mut dropped = Vec::new();
        for brick in &self.items {
            // Lower the brick as far as it can go
            let n_brick = brick.drop_brick(&dropped);
            dropped.push(n_brick);
        }
        for brick in &dropped {
            println!("Dropped Brick: {:?}", brick);
        }
        dropped.sort_by(|a,b| a.lowest_z().cmp(&b.lowest_z()));

        // For each brick, test if it can be disintegrated
        let mut safe = 0;
        for brick in &dropped {
            println!("Testing brick: {:?}", brick);
            if brick.safe_to_disintegrate(&dropped) {
                safe += 1;
            }
        }
        safe.to_string()
    }

    // Cut-n-paste from A, should refactor...
    fn compute_b(&mut self) -> String {
        // Sort by lowest, we can ignore higher up bricks
        self.items.sort_by(|a,b| a.lowest_z().cmp(&b.lowest_z()));
        let mut dropped = Vec::new();
        for brick in &self.items {
            // Lower the brick as far as it can go
            let n_brick = brick.drop_brick(&dropped);
            dropped.push(n_brick);
        }
        for brick in &dropped {
            println!("Dropped Brick: {:?}", brick);
        }
        dropped.sort_by(|a,b| a.lowest_z().cmp(&b.lowest_z()));

        // For each brick, count how many bricks would drop
        let mut would_drop = 0;
        for brick in &dropped {
            let mut drop_count = 0;
            let mut dropped2: Vec<Brick> = Vec::new();
            for brick2 in &dropped {
                if brick2.index == brick.index {
                    continue;
                }
                // Lower the brick as far as it can go
                let n_brick = brick2.drop_brick(&dropped2);
                if !n_brick.eq(brick2) {
                    drop_count += 1;
                }
                dropped2.push(n_brick);
            }
            println!("For {}, {} bricks would drop", brick.index, drop_count);
            would_drop += drop_count;
        }
        would_drop.to_string()
    }
}

