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
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day20::Day20;
use day21::Day21;
use day22::Day22;
use day23::Day23;
use day24::Day24;
use day25::Day25;

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
        8 => day = Box::new(Day8::new()),
        9 => day = Box::new(Day9::new()),
        10 => day = Box::new(Day10::new()),
        11 => day = Box::new(Day11::new()),
        12 => day = Box::new(Day12::new()),
        13 => day = Box::new(Day13::new()),
        14 => day = Box::new(Day14::new()),
        15 => day = Box::new(Day15::new()),
        16 => day = Box::new(Day16::new()),
        17 => day = Box::new(Day17::new()),
        18 => day = Box::new(Day18::new()),
        19 => day = Box::new(Day19::new()),
        20 => day = Box::new(Day20::new()),
        21 => day = Box::new(Day21::new()),
        22 => day = Box::new(Day22::new()),
        23 => day = Box::new(Day23::new()),
        24 => day = Box::new(Day24::new()),
        25 => day = Box::new(Day25::new()),
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

