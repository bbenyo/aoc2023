// TODO: Refactor Days into a trait (AOCProblem), so we can call methods on the AOCProblem trait here
//   e.g. AOCProblem::handle_line
//   Add a builder factory that takes the day and returns the proper AOCProblem
//   Then each day, we can add a new AOCProblem implementation only
//  Until we do that refactor, we'll just add another line to the matches

use std::{fs, error::Error};

mod day1;
mod day2;

use day1::Day1;
use day2::Day2;

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

    println!("Reading file {}", e_filename);
    let contents = fs::read_to_string(e_filename)?;

    // TODO make this a trait
    let mut day1: Day1 = Day1::new();
    let mut day2 = Day2::new();

    // TODO: Make an AOC algorithm trait
    for line in contents.lines() {
        match config.day {
            1 => day1::handle_line(line, &config, &mut day1),
            2 => day2::handle_line(line, &config, &mut day2),
            _ => return Err("Day not yet handled".into()),
        }
    }

    // TODO: Convert to functions on day trait
    match config.variant {
        false => {
            match config.day {
                1 => day1::compute_a(&mut day1),
                2 => day2::compute_a(&mut day2),
                _ => return Err("Day not yet handled".into()),
            }
        }
        true => {
            match config.day {
                1 => day1::compute_b(&mut day1),
                2 => day2::compute_b(&mut day2),
                _ => return Err("Day not yet handled".into()),
            }
        }
    };

    match config.day {
        1 => Ok(day1::get_result(&day1)),
        2 => Ok(day2::get_result(&day2)),
        _ => return Err("Day not yet handled".into()),
    }
}

