use crate::{Config, AOCProblem};

pub struct DayN {
    items: Vec<u64>,
}

impl DayN {
    pub fn new() -> DayN {
        DayN {
            items: Vec::new(),
        }
    }
}

impl AOCProblem for DayN {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let mut line_iter = line.split_whitespace();

        for token in line_iter {
            let item = token.parse::<u64>().unwrap();
            self.items.push(item);
        }
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        let mut val = 1;
        for item in &self.items {
            val = val + item;
        }
        val.to_string()
    }

    fn compute_b(&mut self) -> String {
        return self.compute_a();
    }
}

