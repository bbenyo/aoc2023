use crate::{Config, AOCProblem};

// Values is what we're trying to compute
//  Seq_of_seq (sequence of sequences) is the original sequence
//    And the following difference sequences, however many are needed
pub struct Day9 {
    values: Vec<i64>,
    seq_of_seq: Vec<Vec<i64>>,
}

impl Day9 {
    pub fn new() -> Day9 {
        Day9 {
            values: Vec::new(),
            seq_of_seq: Vec::new(),
        }
    }

    fn get_last_seq(&self) -> &Vec<i64> {
        return self.seq_of_seq.last().unwrap();
    }

    // Generate a sequence containing the difference in values from the input sequence
    fn generate_differece_seq(&self, seq: &Vec<i64>) -> Vec<i64> {
        let mut last_num: i64 = *seq.get(0).unwrap();
        let mut diff_seq: Vec<i64> = Vec::new();
        for idx in 1..seq.len() {
            let num = *seq.get(idx).unwrap();
            diff_seq.push(num - last_num);
            last_num = num;
        }
        diff_seq
    }

    fn is_complete(&self, seq: &Vec<i64>) -> bool {
        for num in seq {
            if *num != 0 { return false; }
        }
        return true;
    }
}

impl AOCProblem for Day9 {
    fn handle_line(&mut self, line: &str, config: &Config) {
        let line_str: String = String::from(line);
        let line_iter = line_str.split_whitespace();
        self.seq_of_seq.clear();

        let mut orig_sequence: Vec<i64> = Vec::new();

        for token in line_iter {
            let item = token.parse::<i64>().unwrap();
            orig_sequence.push(item);
        }

        println!("Orig Sequence: {:?}", &orig_sequence);
        self.seq_of_seq.push(orig_sequence);
        let mut cur_sequence = self.get_last_seq();
        while !(self.is_complete(cur_sequence)) {
            let next_seq = self.generate_differece_seq(&cur_sequence);
            println!("Next sequence: {:?}", &next_seq);
            self.seq_of_seq.push(next_seq);
            cur_sequence = self.get_last_seq();
        }

        // Part a, extrapolate the next sequence item on the end
        if !config.variant {
            let mut add_val = 0;
            for i in 1..self.seq_of_seq.len()+1 {
                let idx = self.seq_of_seq.len() - i;
                let next_seq = self.seq_of_seq.get(idx).unwrap();
                let num = next_seq.last().unwrap();
                if idx + 1 < self.seq_of_seq.len() {
                    add_val = num + add_val;
                }
                println!("Last val for row {} = {}", i, add_val);
            }
            self.values.push(add_val);
        } else {
            // Part b, extrapolate at the beginning instead
            let mut sub_val = 0;
            for i in 1..self.seq_of_seq.len()+1 {
                let idx = self.seq_of_seq.len() - i;
                let next_seq = self.seq_of_seq.get(idx).unwrap();
                let num = next_seq.get(0).unwrap();
                if idx + 1 < self.seq_of_seq.len() {
                    sub_val = num - sub_val;
                }
                println!("First val for row {} = {}", i, sub_val);
            }
            self.values.push(sub_val);
        }
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        let mut val = 0;
        for item in &self.values {
            println!("Adding {}", item);
            val = val + item;
        }
        val.to_string()
    }

    fn compute_b(&mut self) -> String {
        return self.compute_a();
    }
}

