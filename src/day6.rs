use crate::{Config, AOCProblem};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    
    // How far does the boat move in a race if we press the button for x ms
    fn compute_race_distance(&self, button_hold: u64) -> u64 {
        let velocity = button_hold;
        let time_remaining = self.time - button_hold;
        time_remaining * velocity        
    }

    // How many ways can we win the race, by pressing the button for different times
    fn count_win_options(&self) -> u64 {
        let mut wins = 0;
        // pressing for 0 is useless, as is pressing for the entire race
        // Could binary search to find the first winning value...
        for i in 1..(self.time - 1) {
            let p_distance = self.compute_race_distance(i);
            //println!("Pressing for {} gives a distance of {} vs the record of {}", i, p_distance, self.distance);
            if p_distance > self.distance {
                wins += 1;
            } else if wins > 0 {
                // We already found a winning value, and now are losing
                // Further times will also lose
                break;
            }
        }
        println!("Wins for race: {}", wins);
        wins
    }

}

pub struct Day6 {
    races: Vec<Race>,
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 {
            races: Vec::new(),
        }
    }
}

impl AOCProblem for Day6 {
    fn handle_line(&mut self, line: &str, config: &Config) {
        let mut line_iter = line.split_whitespace();
        // Read times first
        let token = line_iter.next().unwrap();
        if token == "Time:" {
            if !config.variant {
                // Part a
                for time_token in line_iter {
                    let time = time_token.parse::<u64>().unwrap();
                    let race = Race { time, distance: 0};
                    self.races.push(race);
                } 
            } else {
                // Part b, we ignore whitespace and make 1 number
                let mut merged_str = String::new();
                for time_token in line_iter {
                    merged_str.push_str(time_token);
                }
                let time = merged_str.parse::<u64>().unwrap();
                let race = Race {time, distance: 0};
                self.races.push(race);
            }
        } else if token == "Distance:" {
            let mut i = 0;
            // Part a, parse N races
            if !config.variant {
                for dist_token in line_iter {
                    let dist = dist_token.parse::<u64>().unwrap();
                    let race = self.races.get_mut(i).unwrap();
                    race.distance = dist;
                    i += 1;
                    println!("Parsed race: {:?}", race);
                }
            } else {
               // Part b, we ignore whitespace and make 1 number
               let mut merged_str = String::new();
               for dist_token in line_iter {
                   merged_str.push_str(dist_token);
               }
               let dist = merged_str.parse::<u64>().unwrap();
               let race = self.races.get_mut(0).unwrap();
               race.distance = dist;
               println!("Parsed race: {:?}", race);
            }
        } else {
            eprintln!("Error: Unexpected line: {}", line);
        }
    }

    fn compute_a(&mut self) -> String {
        let mut val = 1;
        for race in &self.races {
            let opts = race.count_win_options();
            val = val * opts;
        }
        val.to_string()
    }

    fn compute_b(&mut self) -> String {
        // We parsed differently, computation is the same
        return self.compute_a();
    }
}