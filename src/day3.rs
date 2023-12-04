use crate::{Config, AOCProblem};

pub struct Part {
    start: usize,  // index in the row of the start digit of the part 
    end: usize, // index in the row of the last digit
    number: u32, // part number
}

pub struct Day3 {
    part_numbers_valid: Vec<u32>,
    part_numbers_invalid: Vec<u32>,
    prev_row: String,
    cur_row: String,
    next_row: String,
    prev_row_parts: Vec<Part>,
    cur_row_parts: Vec<Part>,
    next_row_parts: Vec<Part>,
    gear_ratio_sum: u32,
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            part_numbers_valid: Vec::new(),
            part_numbers_invalid: Vec::new(),

            // Raw characters from the previous, current, and next row
            prev_row: String::from(""),
            cur_row: String::from(""),
            next_row: String::from(""),

            // Parsed out parts from the prev, current, and next row
            prev_row_parts: Vec::new(),
            cur_row_parts: Vec::new(),
            next_row_parts: Vec::new(),

            gear_ratio_sum: 0,
        }
    }

    // Is the part number cur_row[start..end] valid?
    // Part A: Valid means there's a symbol adjacent somewhere
    fn is_valid_part(&self, start: usize, end: usize) -> bool {
        // First check current row, start-1 and end + 1
        let valid_start = if start == 0 { 0 } else { start -1 };
        let valid_end = if end == (self.cur_row.len() - 1) { end } else {end + 1};
       
        let chr = &self.cur_row[valid_start..valid_start+1].parse::<char>().unwrap();
        if part_symbol(*chr) { return true };
        
        let chr = &self.cur_row[(valid_end-1)..valid_end].parse::<char>().unwrap();
        if part_symbol(*chr) { return true };

        if self.prev_row.len() > 0 {
            // Previous row, start-1 .. end+1
            let prev_chars = &self.prev_row[valid_start..valid_end];
            for chr in prev_chars.chars() {
                if part_symbol(chr) { return true; }
            }
        }

        if self.next_row.len() > 0 {
            let next_chars = &self.next_row[valid_start..valid_end];
            for chr in next_chars.chars() {
                if part_symbol(chr) { return true; }
            }
        }

        false
    }

}

fn part_symbol(chr: char) -> bool {
    return !(chr.is_ascii_digit() || chr == '.');
}

impl Day3 {
    // Parse parts from the given row
    fn parse_parts(&self, row: &String) -> Vec<Part> {
        let mut start_index: usize = 0;
        let mut started_part: bool = false;
        let mut i: usize = 0;
        let mut part_vec: Vec<Part> = Vec::new();
        for char in row.chars() {
            if char.is_ascii_digit() {
                if started_part == false {
                    start_index = i;
                    started_part = true;
                }
            } else {
                if started_part == true {
                    let end_index = i;
                    let part_str = &row[(start_index as usize)..end_index];
                    let new_part = Part {
                        start: start_index,
                        end: end_index - 1,
                        number: part_str.parse::<u32>().unwrap(),
                    };
                    part_vec.push(new_part);
                    started_part = false;
                }
            }
            i += 1;
        }
        // Get any part we started and ended at the last char
        if started_part == true {
            let part_str = &row[(start_index as usize)..];
            let new_part = Part {
                start: start_index,
                end: row.len() - 1,
                number: part_str.parse::<u32>().unwrap(),
            };
            part_vec.push(new_part);
        }
        part_vec
    }

    fn get_gear_ratio(&self, index: usize) -> u32 {
        // Count the number of parts we're adjacent to
        let mut part1 = 0;
        let mut part2 = 0;
        for part in &self.prev_row_parts {
            if part.start <= index + 1 && part.end >= index - 1 {
                // Overlap!
                if part1 == 0 { part1 = part.number } 
                else if part2 == 0 { part2 = part.number } 
                // Too many parts adjacent
                else { return 0; }
            }
        }
        for part in &self.cur_row_parts {
            if part.start <= index + 1 && part.end >= index - 1 {
                // Overlap!
                if part1 == 0 { part1 = part.number } 
                else if part2 == 0 { part2 = part.number } 
                // Too many parts adjacent
                else { return 0; }
            }
        }
        for part in &self.next_row_parts {
            if part.start <= index + 1 && part.end >= index - 1 {
                // Overlap!
                if part1 == 0 { part1 = part.number } 
                else if part2 == 0 { part2 = part.number } 
                // Too many parts adjacent
                else { return 0; }
            }
        }
        return part1 * part2
    }

    fn handle_current_row(&mut self) {
        // println!("Handling current row: {}", self.cur_row);
        // println!("\tPrev row: {}", self.prev_row);
        // println!("\tNext row: {}", self.next_row);

        for part in &self.cur_row_parts {
            if self.is_valid_part(part.start, part.end) {            
                println!("Found VALID part: {}", part.number);
                self.part_numbers_valid.push(part.number);
            } else {
                println!("Found INVALID part!: {}", part.number);
                self.part_numbers_invalid.push(part.number);
            }
        }

        // Look for Gears for part b
        let mut i = 0;
        for char in self.cur_row.chars() {
            if char == '*' {
                let ratio = self.get_gear_ratio(i);
                println!("Gear found at {} Ratio: {}", i, ratio);
                self.gear_ratio_sum += ratio;
            }
            i += 1;
        }
        
    }
}


impl AOCProblem for Day3 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        if self.cur_row.len() == 0 {
            // This is the first row
            self.cur_row = String::from(line);
            self.cur_row_parts = self.parse_parts(&self.cur_row);
            return;
        }
        self.next_row = String::from(line);
        self.next_row_parts = self.parse_parts(&self.next_row);
        self.handle_current_row();
        self.prev_row = self.cur_row.clone();
        self.prev_row_parts.clear();
        self.prev_row_parts.append(&mut self.cur_row_parts);
        self.cur_row = self.next_row.clone();
        self.cur_row_parts.clear();
        self.cur_row_parts.append(&mut self.next_row_parts);
    }

    fn compute_a(&mut self) -> String {
        self.next_row = String::from("");
        self.handle_current_row();
        let mut sum = 0;
        for part in self.part_numbers_valid.iter() {
            println!("Valid part: {}", part);
            sum += part;
        }   
        sum.to_string()
    }

    fn compute_b(&mut self) -> String {
        self.next_row = String::from("");
        self.handle_current_row();
        self.gear_ratio_sum.to_string()
    }
}
