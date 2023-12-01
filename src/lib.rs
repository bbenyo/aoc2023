// TODO: Split this into a separate Crate that each day can use
// Cut-n-Paste until we do that
// TODO: Refactor into a trait, so we can call methods on the AOCProblem trait here
//   Then each day, we can add an AOCProblem implementation only

use std::{fs, error::Error};

pub struct Config {
    // Which day we're doing (1-25)
    // We'll generate the filenames from the day
    pub day: u8,

    // True if we're doing part b, false if we're doing part a
    pub variant: bool,

    // True if we're running the test inputs, false if we're running the final input
    pub test_input: bool,
}

fn usage() {
    eprintln!("Usage: day variant test_input");
    eprintln!("  day: 1-25 (which AOC day to run)");
    eprintln!("  variant: a | b");
    eprintln!("  test_input: test | final");
    eprintln!("    test input is expected at testX.txt");
    eprintln!("    final input is expected at finalX.txt");
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 4 {
            usage();
            return Err("Incorrect number of command line parameters");
        }

        let day = args[1].parse::<u8>().unwrap_or(0);
        if day < 1 || day > 25 {
            usage();
            return Err("Expecting a day value from 1-25");
        }

        // TODO: Instead of defaulting to false, throw an Err
        let variant = match args[2].as_str() {
            "a" => false,
            "b" => true,
            _ => return Err("Expecting a or b for variant"),
        };

        let test_input = match args[3].as_str() {
            "test" => true,
            "final" => false,
            _ => return Err("Expecting test or final for test_input"),
        };
            
        Ok(Config { day, variant, test_input })
    }
}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let mut e_filename = String::new();
    match config.test_input {
        true => e_filename.push_str("test_"),
        false => e_filename.push_str("final_"),
    }
    e_filename.push_str(config.day.to_string().as_str());
    e_filename.push_str(".txt");

    let contents = fs::read_to_string(e_filename)?;

    let mut day: Day1 = Day1::new();

    // TODO: Make an AOC algorithm trait
    for line in contents.lines() {
        handle_line(line, &config, &mut day);
    }

    // TODO: Convert to functions on day trait
    match config.variant {
        false => compute_a(&mut day),
        true => compute_b(&mut day),
    };

    Ok(get_result(&day))
}

// TODO: Make this a trait
//   Day1 can implement the AOCProblem trait
struct Day1 {
    calibration_values: Vec<u32>,
    sum: u32,
}

impl Day1 {
    fn new() -> Day1 {
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

fn handle_line(line: &str, config: &Config, day: &mut Day1) {    
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

fn compute_a(day: &mut Day1) {
    println!("Computing variant a");
    day.sum = day.calibration_values.iter().sum();
}

fn compute_b(day: &mut Day1) {
    println!("Computing variant b");
    // Computation here is the same as part a
    compute_a(day);
}

fn get_result(day: &Day1) -> String {
    return day.sum.to_string()
}
