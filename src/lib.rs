// TODO: Refactor Days into a trait (AOCProblem), so we can call methods on the AOCProblem trait here
//   e.g. AOCProblem::handle_line
//   Add a builder factory that takes the day and returns the proper AOCProblem
//   Then each day, we can add a new AOCProblem implementation only
//  Until we do that refactor, we'll just add another line to the matches

use std::{fs, error::Error};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;

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

pub trait AOCProblem {
    fn handle_line(&mut self, line: &str, config: &Config);
    fn compute_a(&mut self) -> String;
    fn compute_b(&mut self) -> String;
}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let mut e_filename = String::new();
    match config.test_input {
        true => e_filename.push_str("data/test/test_"),
        false => e_filename.push_str("data/final/final_"),
    }
    e_filename.push_str(config.day.to_string().as_str());
    e_filename.push_str(".txt");

    println!("Reading file {}", e_filename);
    let contents = fs::read_to_string(e_filename)?;

    // TODO: Make a AOCproblem factory to create these
    let mut day: Box<dyn AOCProblem>;
    match config.day {
        1 => day = Box::new(Day1::new()),
        2 => day = Box::new(Day2::new()),
        3 => day = Box::new(Day3::new()),
        4 => day = Box::new(Day4::new()),
        5 => day = Box::new(Day5::new()),
        6 => day = Box::new(Day6::new()),
        7 => day = Box::new(Day7::new()),
        _ => return Err("Day not yet handled".into()),
    }
    
    // Read the input, pass it to the AOCProblem trait
    for line in contents.lines() {
        (*day).handle_line(line, &config);
    }

    let ret: String = match config.variant {
        false => (*day).compute_a(),
        true => (*day).compute_b(),
    };
    Ok(ret)
}

