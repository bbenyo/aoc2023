use crate::{Config, AOCProblem};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    label: String,
    left: String,
    right: String,
}
pub struct Day8 {
    instructions: Vec<char>,
    nodes: HashMap<String, Node>,
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8 {
            instructions: Vec::new(),
            nodes: HashMap::new(),
        }
    }
}

impl AOCProblem for Day8 {
    fn handle_line(&mut self, line: &str, _config: &Config) {        
        if line.len() == 0 {
            return;
        }
        
        if self.instructions.len() == 0 {
            self.instructions = line.chars().collect();
            return;
        }

        // Drop the = ( , from the string to get
        // Label Left Right
        let stripped: String = line.chars().filter(|c| c.is_whitespace() || c.is_alphanumeric()).collect();
        let mut line_iter = stripped.split_whitespace();
        // println!("Stripped String: {}", stripped);
        let label = line_iter.next().unwrap();
        let left = line_iter.next().unwrap();
        let right = line_iter.next().unwrap();

        let node = Node { 
            label: String::from(label), 
            left: String::from(left), 
            right: String::from(right)
        };

        println!("Parsed node: {:?}", node);
        self.nodes.insert(String::from(&node.label), node);
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        let mut cur_node = self.nodes.get("AAA").unwrap();
        let mut iptr = 0;
        let mut steps = 0;
        while cur_node.label != "ZZZ" {
            let prev_node_lbl = &cur_node.label;
            let step = self.instructions.get(iptr).unwrap();
            iptr += 1;
            if iptr >= self.instructions.len() { iptr = 0 };
            match step {
                'L' => cur_node = self.nodes.get(&cur_node.left[..]).unwrap(),
                'R' => cur_node = self.nodes.get(&cur_node.right[..]).unwrap(),
                _ => { eprintln!("Unknown instruction: {}", step); return "Error".to_string(); },
            }
            steps += 1;
            println!("{} -> {} -> {}", prev_node_lbl, step, cur_node.label);
        }
        steps.to_string()
    }

    fn compute_b(&mut self) -> String {
        let mut cur_nodes: Vec<&Node> = Vec::new();
        // Start on every node ending with A
        for node in &self.nodes {
            if node.0.ends_with("A") {
                cur_nodes.push(&node.1);
            }
        }
        // Brute force takes too long, ugh
        
        // Let's do each node 1 by 1 and see how long to find an end, and look for a cycle
        // Testing shows we have cycles where the instruction pointer is at the same spot, so a real cycle
        // Compute the lowest common multiplier of all cycle periods.
        //  Basically, this is "lucky", if you look for a different end node (such as steps ending in B) it won't work
        //   Since the instruction pointer is at a different spot, it's not really a cycle since we may go a different path
        //  But for nodes ending in Z as the final, the iptr is at the same spot so it's a real cycle
        
        let mut cycle_counts: Vec<u64> = Vec::new();

        for node in cur_nodes {
            let mut steps = 0;
            let mut iptr = 0;
            let mut cur_node = node;
            let cycle = false;
            let mut found_end_once = false;
            let mut cycle_steps = 0;
            while !cycle {
                let found_end = cur_node.label.ends_with("Z");
                if found_end {
                    if !found_end_once { 
                        found_end_once = true;
                        cycle_steps = 0;
                        println!("{} found end {} for the first time after {} steps with iptr {}", node.label, cur_node.label, steps, iptr);
                    } else {
                        println!("{} found {} in {} initial steps then again after {} steps with iptr {}",
                            node.label, cur_node.label, steps, cycle_steps, iptr);
                            cycle_counts.push(cycle_steps);
                        break;
                    }
                }
                let step = self.instructions.get(iptr).unwrap();
                iptr += 1;
                if iptr >= self.instructions.len() { iptr = 0 };
                match step {
                    'L' => cur_node = self.nodes.get(&cur_node.left[..]).unwrap(),
                    'R' => cur_node = self.nodes.get(&cur_node.right[..]).unwrap(),
                    _ => { eprintln!("Unknown instruction: {}", step); return "Error".to_string(); },
                }
                if found_end_once { cycle_steps += 1; } else { steps += 1; }
                //println!("{} -> {} -> {}", prev_node_lbl, step, cur_node.label);
            }
            // println!("{} Found end node in {} steps, iptr at {}", node.label, steps, iptr);
        }

        let mut num1: u64 = 0;
        for ccount in cycle_counts {
            if num1 == 0 { num1 = ccount; continue; }
            let l: u64 = num::integer::lcm(ccount, num1);
            println!("LCM of {} and {} is {}", ccount, num1, l);
            num1 = l;
        }

        return num1.to_string();
    }

    /*
      Brute force
       while !self.is_finished(&cur_nodes) {
            println!("Step {}", steps);
            let step = self.instructions.get(iptr).unwrap();
            let mut new_nodes: Vec<&Node> = Vec::new();
            for cur_node in &cur_nodes {
                let prev_node_lbl = &cur_node.label;
                if iptr >= self.instructions.len() { iptr = 0 };
                let new_node;
                match step {
                    'L' => new_node = self.nodes.get(&cur_node.left[..]).unwrap(),
                    'R' => new_node = self.nodes.get(&cur_node.right[..]).unwrap(),
                    _ => { eprintln!("Unknown instruction: {}", step); return "Error".to_string(); },
                }
                println!("\t{} -> {} -> {}", prev_node_lbl, step, new_node.label);
                new_nodes.push(new_node);
            }
            cur_nodes.clear();
            cur_nodes.append(&mut new_nodes);
            steps += 1;
            iptr += 1;
            if iptr >= self.instructions.len() { iptr = 0 };
        }
     */
    
}
