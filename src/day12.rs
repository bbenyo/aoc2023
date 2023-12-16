use crate::{Config, AOCProblem};
use std::collections::HashMap;

#[derive(Debug)]
// For part B we need to cache.
//  If we're trying to place block N, and the earliest place we can put it is location M
//  We can remember (M,N,#combos), so the next time we're trying to place N at M we just look up combos
struct PicrossRow {
    row: Vec<char>,
    counts: Vec<usize>,
    cache: HashMap<u64, u64>,
}

// Search item, where in the row are we, and which block are we trying to place
// These will be entered into a work queue for the search process
#[derive(Debug)]
struct WorkItem {
    index: usize,
    block: usize,
    placements: Vec<usize>, // Where we put each block (starting)
    combos: u64,
}

impl PicrossRow {
    // Can we place block # block_index starting at position start?
    // If so, where's the first place we can put the next block (1 beyond the end of this block)
    fn place_block(&mut self, block_size: usize, start: usize) -> Option<usize> {
        // Find the first spot starting at i that we can put block (block_index)
        let mut block_pos = None;
        for i in start..self.row.len() {
           if self.fit_block_here(i, block_size) {
                block_pos = Some(i);
                break;
           }
           if self.row[i] == '#' {
                // We don't have a choice, next block has to start here
                // But we couldn't fit it, so this fails
                return None;
           }
        }
        match block_pos {
            None => None,
            Some(pos) => Some(pos),
        }
    }

    fn fit_block_here(&mut self, index: usize, size: usize) -> bool {
        if index+size > self.row.len() { return false; }
        for i in 0..size {
            if self.row[index+i] == '.' { return false; }
        }
        // Next space has to be empty or the end
        let after_end = index+size;
        if after_end < self.row.len() {
            if self.row[after_end] == '#' {
                return false;
            }
        }
        return true;
    }

    fn _print_solution(&self, item: &WorkItem) {
        let mut soln: Vec<char> = vec!['.'; self.row.len()];
        println!("Solution: {:?}", item);
        for i in 0..self.counts.len() {
            let size = self.counts[i];
            let start = item.placements[i];
            for j in start..(start+size) {
                soln[j] = '#';
            }
        }
        let mut board_str = String::new();
        for k in 0..soln.len() {
            board_str.push(soln[k]);
        }
        println!("{:?}", board_str);
    }

    // How many combos can work for this row?
    fn count_all_combos(&mut self) -> u64 {
        let mut count = 0;
        let first_block_earliest = self.place_block(self.counts[0], 0);
        let first_block;
        match first_block_earliest {
            None => return 0,
            Some(f) => first_block = f,
        }
        let mut w1 = WorkItem {index: first_block, block: 0, placements: Vec::new(), combos: 0};
        count += w1.count_combos(self);
        if self.row[first_block] == '?' {
            // We don't have to start here, could start later
            for k in first_block+1..(self.row.len() - self.counts[0] + 1) {
                if self.row[k] == '?' || self.row[k] == '#' {
                    // We could start here too, maybe
                    let mut new_item = WorkItem { index: k, block: 0, placements: Vec::new(), combos: 0 };
                    count += new_item.count_combos(self);                    
                    if self.row[k] == '#' {
                        // This was the last place we can start
                        break;
                    }
                }
            }
        }
        count
    }
}

impl WorkItem {
    fn get_hash_key(&self) -> u64 {
        // Hash this workItem using block and index only
        // Since we only have 2 numbers, a simple x*1000 + y will work, as long as y will be less than 1000
        return (self.index * 1000 + self.block) as u64;
    }
    fn count_combos(&mut self, row: &mut PicrossRow) -> u64 {
        let key = self.get_hash_key();
        if let Some(cached_combos) = row.cache.get(&key) {
            //println!("\tCache Hit: {:?}/{} = {}", self, key, cached_combos);
            return *cached_combos;
        }
        
        //println!("Working on {:?}", self);
        let mut count = 0;
        let next_block_size = row.counts[self.block];
        let valid = row.fit_block_here(self.index, next_block_size);
        if valid {
            let block_start = self.index;
            self.placements.push(block_start);
            let s_block = self.block + 1;
            if s_block >= row.counts.len() {
                // We placed the last block, this is a complete answer if there are no more #'s
                let mut no_extra = true;
                for k in block_start+next_block_size..row.row.len() {
                    if row.row[k] == '#' {
                        // There's another block! this doesn't work
                        no_extra = false;
                        break;
                    }
                }
                if no_extra {
                    count += 1;
                    // row.print_solution(&self);
                }
            } else {
                // Need to place another block, earliest it can go is 2 spaces after the end of this one
                //  (leave 1 empty space between blocks)
                let next_start = block_start + next_block_size + 1;
                for k in next_start..row.row.len() {
                    if row.row[k] == '.' { continue; } // Can't start on a known empty
                    let mut new_work_item = WorkItem {index: k, block: s_block, placements: self.placements.clone(), combos: 0};
                    count += new_work_item.count_combos(row);
                    if row.row[k] == '#' { break; } // Have to start the next one here at least
                }
            }
        }
        //println!("\t Combos for {:?} = {}", self, count);
        row.cache.insert(key, count);
        self.combos = count;
        return count;
    }
}

pub struct Day12 {
    rows: Vec<PicrossRow>,
}

impl Day12 {
    pub fn new() -> Day12 {
        Day12 {
            rows: Vec::new(),
        }
    }
}

impl AOCProblem for Day12 {
    fn handle_line(&mut self, line: &str, config: &Config) {
        let line_str: String = String::from(line);
        let mut line_iter = line_str.split_whitespace();

        // Two components, initial row and counts
        let mut init_row = line_iter.next().unwrap();
        let init_counts = line_iter.next().unwrap();
        let mut counts: Vec<usize> = init_counts.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        let mut row_str = String::from(init_row);

        if config.variant {
            for _i in 0..4 {
                row_str.push('?');
                row_str.push_str(init_row);

            }
            init_row = &row_str;

            let orig_counts = counts.clone();
            for _i in 0..4 {
                let mut c2 = orig_counts.clone();
                counts.append(&mut c2);
            }
        }

        let puzzle_row = PicrossRow {
            row: init_row.chars().collect(),
            counts,
            cache: HashMap::new(),
        };
        println!("Picross Row: {:?}", puzzle_row);
        self.rows.push(puzzle_row);
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        let mut val: u64 = 0;
        for item in &mut self.rows {
            println!("Counting Combos for {:?}", item);
            let combos = item.count_all_combos();
            println!("Combos: {}", combos);
            val += combos;
        }
        val.to_string()
    }

    fn compute_b(&mut self) -> String {
        return self.compute_a();
    }
}
