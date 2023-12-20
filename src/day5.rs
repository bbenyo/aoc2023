use crate::{Config, AOCProblem};

#[derive(Debug)]
struct Mapping {
    dest_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}

impl Mapping {
    // Use the mapping to convert a source to a dest object, if possible
    fn convert_source_to_dest(&self, source: u64) -> Option<u64> {
        if source >= self.src_range_start && source < (self.src_range_start + self.range_len) {
            // We're in the source range, we can map
            let range_delta = source - self.src_range_start;
            Some(self.dest_range_start + range_delta)
        } else {
            None
        }
    }

    fn convert_dest_to_source(&self, dest: u64) -> Option<u64> {
        if dest >= self.dest_range_start && dest < (self.dest_range_start + self.range_len) {
            let range_delta = dest - self.dest_range_start;
            Some(self.src_range_start + range_delta)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct XYMap {
    to: String,
    mappings: Vec<Mapping>,
}

impl XYMap {
    fn add_mapping(&mut self, line_str: String) {
        let mut line_iter = line_str.split_whitespace();
        // Parse a mapping, let it panic if there are any parsing errors
        // TODO: Error handling
        let dest_range_start = line_iter.next().unwrap_or("").parse::<u64>().unwrap();
        let src_range_start = line_iter.next().unwrap_or("").parse::<u64>().unwrap();
        let range_len = line_iter.next().unwrap_or("").parse::<u64>().unwrap();

        let new_mapping = Mapping { dest_range_start, src_range_start, range_len };
        self.mappings.push(new_mapping);        
    }

    fn map_source_to_dest(&self, source: u64) -> u64 {
        for mapping in &self.mappings {
            if let Some(dest) = mapping.convert_source_to_dest(source) {
                return dest
            }
        }
        // No special mapping, dest = source
        source
    }

    // revert map
    fn map_dest_to_source(&self, dest: u64) -> u64 {
        for mapping in &self.mappings {
            if let Some(src) = mapping.convert_dest_to_source(dest) {
                return src
            }
        }
        dest
    }
}

struct SeedRange {
    start: u64,
    end: u64,
}

use std::collections::HashMap;

pub struct Day5 {
    seeds: Vec<u64>,
    seed_ranges: Vec<SeedRange>,
    maps: HashMap<String, XYMap>, // Map of the from value to an XY map
    current_map: String, // Name of the map we're currently parsing in
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5 {
            seeds: Vec::new(),
            seed_ranges: Vec::new(),
            maps: HashMap::new(),
            current_map: String::from(""),
        }
    }

    fn map_seed_to_location(&self, seed: u64) -> u64 {
        let next_to = String::from("seed");
        let mut next_map = self.maps.get(&next_to);
        let mut source = seed;
        while next_map.is_some() {
            let map = next_map.unwrap();
            let dest = map.map_source_to_dest(source);
            // println!("{} {} -> {} {}", &map.from, source, &map.to, dest);
            next_map = self.maps.get(&map.to);
            source = dest;
        }
        source
    }

    // Reverse map for part b
    fn map_location_to_seed(&self, location: u64) -> u64 {
        // Hardcode the reverse map, could add another hashmap instead
        let reverse_map = vec!["humidity", "temperature", "light", "water", "fertilizer", "soil", "seed"];
        let mut dest = location;
        for next_from in reverse_map {
            let next_map = self.maps.get(next_from);
            let map = next_map.unwrap();
            let src = map.map_dest_to_source(dest);
            dest = src;
        };
        dest
    }

    fn is_valid_seed(&self, seed: u64) -> bool {
        for seed_range in &self.seed_ranges {
            if seed >= seed_range.start && seed < seed_range.end {
                return true;
            }        
        }
        return false;
    }
}

impl AOCProblem for Day5 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        if line.len() == 0 {
            return;  // Skip empty lines
        }
        let line_str: String = String::from(line);
        let first = &line[..1].chars().next().unwrap();
        if first.is_ascii_digit() {
            // Add another mapping to the current map
            if let Some(c_map) = self.maps.get_mut(&self.current_map) {
                c_map.add_mapping(line_str);
            } else {
                return;
            }
        } else {
            let mut line_iter = line_str.split_whitespace();
            let token = line_iter.next().unwrap();
            if "seeds:" == token {
                while let Some(token) = line_iter.next() {
                    let token2 = line_iter.next().unwrap();
                    let start = token.parse::<u64>().unwrap();
                    let range = token2.parse::<u64>().unwrap();
                    let end = start + range;
                    let seed_range = SeedRange {start, end};
                    // For part B, 2 numbers are a start and range
                    self.seed_ranges.push(seed_range);
                    // For part A, each number is a seed
                    self.seeds.push(start);
                    self.seeds.push(range);
                }
                println!("Parsed seeds: {:?}", self.seeds);
            } else {
                let mut token_split = token.split("-to-");
                let from = token_split.next().unwrap();
                let from_str = String::from(from);
                let to = token_split.next().unwrap();
                let new_map = XYMap {
                    to: String::from(to), mappings: Vec::new(),
                };
                println!("Added new mapping: {:?}", new_map);
                self.maps.insert(from_str, new_map);
                self.current_map = String::from(from);
            }
        }        
    }

    fn compute_a(&mut self) -> String {
        let mut smallest_loc: Option<u64> = None;
        for seed in &self.seeds {
            println!("Seed: {}", seed);
            // Start mapping seeds until we get to location
            let seed_loc = self.map_seed_to_location(*seed);
            println!("\t Final Location: {}", seed_loc);
            match smallest_loc {
                None => smallest_loc = Some(seed_loc),
                Some(loc) => if loc > seed_loc { smallest_loc = Some(seed_loc) }
            }
        }
        smallest_loc.unwrap().to_string()
    }

    // Reverse search, start the the smallest loc, and find the seed that corresponds to it, etc.
    fn compute_b(&mut self) -> String {
        // Break out via return when we found it
        // Start at location 0, go backwards to seed, once we find a valid seed, we're done
        let mut loc = 0;
        loop {
            let seed = self.map_location_to_seed(loc);
            if self.is_valid_seed(seed) {
                println!("VALID seed found {} at location {}", seed, loc);
                return loc.to_string();
            }
            loc += 1;
            if loc % 1000000 == 0 {
                println!("Trying location {}", loc);
            }
        }
    }
}
