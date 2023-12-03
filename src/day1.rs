// TODO: Make this a trait
//   Day1 can implement the AOCProblem trait

use crate::Config;
pub struct Day1 {
    pub calibration_values: Vec<u32>,
    pub sum: u32,
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            calibration_values: Vec::new(),
            sum: 0,
        }
    }
}

struct ReplacePair<'a> {
    from: &'a str,
    to: &'a str,
}

pub fn handle_line(line: &str, config: &Config, day: &mut Day1) {    
    // First and last digits, -1 is uninitialized
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    // May need to edit the line
    let mut line_str = String::from(line);

    if config.variant {
        // Part B, we need to substitute some strings for numbers
        // Of course they'll be tricky with something like "twone"
        // It's not too bad though, we just need to keep the first and last letters to match with other numbers
        let mut replace_map: Vec<ReplacePair> = Vec::new();
        replace_map.push(ReplacePair {from: "one", to: "o1e"});
        replace_map.push(ReplacePair {from: "two", to: "t2o"});
        replace_map.push(ReplacePair {from: "three", to: "t3e"});
        replace_map.push(ReplacePair {from: "four", to: "f4r"});
        replace_map.push(ReplacePair {from: "five", to: "f5e"});
        replace_map.push(ReplacePair {from: "six", to: "s6x"});
        replace_map.push(ReplacePair {from: "seven", to: "s7n"});
        replace_map.push(ReplacePair {from: "eight", to: "e8t"});
        replace_map.push(ReplacePair {from: "nine", to: "n9e"});
       
        replace_map.iter().for_each(|pair| line_str = line_str.replace(pair.from, pair.to));
    }
    
    println!("Line: {}", line_str);

    for char in line_str.chars() {
        if char.is_ascii_digit() {
            let digit = char.to_digit(10).unwrap();
            if first.is_none() {
                first = Some(digit);
                last = Some(digit);
            } else {
                last = Some(digit);
            }
        }
    }

    if first == None || last == None {
        eprintln!("Error on line: {} No digits", line);
        return;
    }
    let cal_value: u32 = first.unwrap() * 10 + last.unwrap();
    day.calibration_values.push(cal_value);
}

pub fn compute_a(day: &mut Day1) {
    println!("Computing variant a");
    day.sum = day.calibration_values.iter().sum();
}

pub fn compute_b(day: &mut Day1) {
    println!("Computing variant b");
    // Computation here is the same as part a
    compute_a(day);
}

pub fn get_result(day: &Day1) -> String {
    return day.sum.to_string()
}
