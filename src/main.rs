use day1::Config;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Usage error: {err}");
        process::exit(1);
    });

    println!("Executing code for day {} {} on input {}",
        config.day, 
        if config.variant {"a"} else {"b"}, 
        if config.test_input {"test"} else {"final"});

    match day1::run(config) {
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        },
        Ok(s) => println!("{}", s),
    }
}
