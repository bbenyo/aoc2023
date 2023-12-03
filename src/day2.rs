// TODO: Make this an AOCProblem trait
use crate::Config;

// For now, we'll store only the maximum number of each cube seen in one draw
//   Since that's all we need for part a


pub struct Day2Game {
    id: u32,
    max_red: u32,
    max_blue: u32,
    max_green: u32,
}

pub struct SinglePull {
    red: u32,
    green: u32,
    blue: u32,
}

pub struct Day2{
    games: Vec<Day2Game>,
    sum: u32,
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {
            games: Vec::new(),
            sum: 0,
        }
    }
}

impl SinglePull {
    pub fn new() -> SinglePull {
        SinglePull {red: 0, green: 0, blue: 0}
    }
}

pub fn handle_line(line: &str, _config: &Config, day: &mut Day2) {    
    // Split the line by " "
    let line_str: String = String::from(line);
    let mut line_iter = line_str.split(" ");

    line_iter.next(); // Game
    let game_id;
    let mut max_red = 0;
    let mut max_green=  0;
    let mut max_blue = 0;

    // TODO: Error checking here
    if let Some(id) = line_iter.next() {
        let id_num = &id[0..id.len() - 1]; // Strip off the :
        game_id = id_num.parse::<u32>().unwrap_or(0);
    } else {
        eprintln!("Game id not found in {}", line);
        return;
    }

    let mut cur_count: u32 = 0;
    let mut cur_pull = SinglePull::new();
    for token in line_iter {
        println!("Token: '{}'", token);
        if cur_count == 0 {
            // We haven't parsed the count number yet, that's next
            cur_count = token.parse::<u32>().unwrap_or(0);
        } else {
            // We've parsed the count, expect a string [blue|red|green], ending in a comma or semicolon
            let tlen = token.len() - 1;
            let last_char = &token[tlen..];
            let color;
            if last_char == ";" || last_char == "," {
                color = &token[0..tlen];
            } else {
                color = &token;
            }
            match color {
                "red" => cur_pull.red = cur_count,
                "green" => cur_pull.green = cur_count,
                "blue" => cur_pull.blue = cur_count,
                _ => {
                    eprintln!("Unable to parse color from {} on line {}", token, line);
                    return;
                }
            }
            match last_char {
                "," => cur_count = 0, // look for the next pull
                _ => {
                    cur_count = 0;
                    if max_red < cur_pull.red { max_red = cur_pull.red};
                    if max_blue < cur_pull.blue { max_blue = cur_pull.blue};
                    if max_green < cur_pull.green { max_green = cur_pull.green};
                }
            }
        }
    }
      
    let game = Day2Game {
        id: game_id,
        max_blue,
        max_red,
        max_green,
    };
    println!("Adding game {}", game.id);
    day.games.push(game);

}

pub fn compute_a(day: &mut Day2) {
    println!("Computing variant a");
    day.sum = 0;
    for game in day.games.iter() {
        println!("Evaluating Game: {}", game.id);
        if game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14 {
            println!("Game {} is valid: {} <= 12 {} <= 13 {} <= 14", game.id, game.max_red, game.max_green, game.max_blue);
            day.sum += game.id;
        }
    }
}

pub fn compute_b(day: &mut Day2) {
    println!("Computing variant b");
    day.sum = 0;
    for game in day.games.iter() {
        println!("Evaluating Game: {}", game.id);
        let power = game.max_red * game.max_blue * game.max_green;
        println!("Game {}:  {} * {} * {} = power {}", game.id, game.max_red, game.max_green, game.max_blue, power);
        day.sum += power;
    }
}

pub fn get_result(day: &Day2) -> String {
    return day.sum.to_string();
}
