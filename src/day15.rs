use crate::{Config, AOCProblem};

#[derive(Debug)]
struct Lens {
    label: String,
    focal: u8,
}

#[derive(Debug)]
struct LensBox {
    index: u8,
    lenses: Vec<Lens>,
}

impl LensBox {
    fn add_lens(&mut self, lens: Lens) {
        let mut old_lens = None;
        let mut idx = 0;
        for l in &self.lenses {
            if l.label == lens.label {
                old_lens = Some(idx);
                break;
            }
            idx+=1;
        }
        match old_lens {
            // No existing lens, add to the end
            None => self.lenses.push(lens),
            // Existing lens, remove it, and 
            Some(l) => {
                self.lenses.remove(l);
                self.lenses.insert(l, lens);
            }
        }
    }

    fn remove_lens(&mut self, lens: &str) {
        let mut old_lens = None;
        let mut idx = 0;
        for l in &self.lenses {
            if l.label == lens {
                old_lens = Some(idx);
                break;
            }
            idx+=1;
        }
        match old_lens {
            // No existing lens, NO-OP
            None => println!("Attempting to remove a non-existent lens! {:?}", lens),
            // Existing lens, remove it, and 
            Some(l) => {self.lenses.remove(l);()},
        }
    }

    fn focusing_power(&self) -> u64 {
        let mut fp: u64 = 0;
        let mut idx = 1;
        for lens in &self.lenses {
            let lp: u64 = (1 + self.index as u64) * idx * (lens.focal as u64);
            fp += lp;
            idx += 1;
        }
        fp
    }
}

pub struct Day15 {
    steps: Vec<String>,
    boxes: Vec<Box<LensBox>>,
}

impl Day15 {
    pub fn new() -> Day15 {
        let mut boxes: Vec<Box<LensBox>> = Vec::new();
        for i in 0..=255 {
            boxes.push(Box::new(LensBox{index: i, lenses: Vec::new()}));
        }
        Day15 { steps: Vec::new(), boxes}
    }

    fn add_lens(&mut self, box_idx: usize, lens: Lens) {
        self.boxes[box_idx].add_lens(lens);
    }

    fn remove_lens(&mut self, box_idx: usize, lens: &str) {
        self.boxes[box_idx].remove_lens(lens);
    }

    fn focusing_power(&self) -> u64 {
        let mut fp = 0;
        for b in &self.boxes {
            fp += b.focusing_power();
        }
        fp
    }
}

// Run the HASH algorithm defined in the problem
// HASH(char) = (old_val + ascii) * 17) % 256
fn hash(str: &String) -> u64 {
    let mut val: u64 = 0;
    for c in str.chars() {
        val = hash_char(c, val);
    }
    val
}

fn hash_char(c: char, val: u64) -> u64 {
    let ascii = c as u64;
    ((val + ascii) * 17) % 256
}

impl AOCProblem for Day15 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let line_str: String = String::from(line);
        let line_iter = line_str.split(',');
        for token in line_iter {
            self.steps.push(String::from(token));
        }
        println!("Read in {} steps", self.steps.len());
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        let mut val: u64 = 0;
        for item in &self.steps {
            let step_val = hash(item);
            println!("HASH({}) = {}", item, step_val);
            val += step_val;
        }
        val.to_string()
    }

    fn compute_b(&mut self) -> String {
        let steps = self.steps.clone();
        for item in steps {
            if item.contains('=') {
                let op: Vec<&str> = item.split('=').collect();
                let label = String::from(op[0]);
                let step_val = hash(&label) as usize;
                let focal = op[1].parse::<u8>().unwrap();
                let new_lens = Lens{label, focal};
                self.add_lens(step_val, new_lens);
            } else if item.contains('-') {
                let op: Vec<&str> = item.split('-').collect();
                let label = String::from(op[0]);
                let step_val = hash(&label) as usize;
                self.remove_lens(step_val, &label);
            }
            //println!("After {}: {:?}", item, self.boxes);
        }
        return self.focusing_power().to_string();
    }
}
