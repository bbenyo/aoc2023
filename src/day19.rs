use crate::{Config, AOCProblem};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn score(&self) -> i64 {
        return self.x + self.m + self.a + self.s;
    }
}

#[derive(Debug, Copy, Clone)]
struct Condition {
    op: char,
    variable: char,
    value: i64,
}

impl Condition {
    fn negate(&self) -> Condition {
        let neg_op: char;
        let neg_val: i64;
        match self.op {
            '>' => {neg_op = '<'; neg_val = self.value + 1; },
            '<' => {neg_op = '>'; neg_val = self.value - 1; },
            _ => panic!("Can't negate a {}", self.op),
        }
        Condition{op: neg_op, value: neg_val, variable: self.variable}
    }

    fn test(&self, part: &Part) -> bool {    
        let cur_val: i64;
        match self.variable {
            'x' => cur_val = part.x,
            'm' => cur_val = part.m,
            'a' => cur_val = part.a,
            's' => cur_val = part.s,
            _ => panic!("Error: unrecognized variable in condition: {:?}", self),
        }
        match self.op {
            '>' => if cur_val > self.value { return true; } else { return false; },
            '<' => if cur_val < self.value { return true; } else { return false; },
            _ => panic!("Error: unrecognized operator in condition: {:?}", self),
        }
    }
}

#[derive(Debug)]
struct Rule {
    cond: Option<Condition>,
    if_true: String,
}

impl Rule {
    fn execute(&self, part: &Part) -> Option<&str> {
        match &self.cond {
            None => Some(&self.if_true),
            Some(c) => {
                if c.test(&part) { return Some(&self.if_true) } else { return None };
            }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    //label: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn execute(&self, part: &Part) -> &str {
        for rule in &self.rules {
            let next = rule.execute(part);
            match next {
                Some(s) => return s,
                _ => (),
            }
        }
        panic!("Workflow failed, no end result: {:?}", self);
    }
}

pub struct Day19 {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
    parsing_parts: bool,
}

impl Day19 {
    pub fn new() -> Day19 {
        Day19 {
            workflows: HashMap::new(),
            parts: Vec::new(),
            parsing_parts: false,
        }
    }

    fn test_part(&self, part: &Part) -> bool {
        let mut workflow = self.workflows.get("in").unwrap();
        let mut log = String::from("in");
        loop {
            let result = workflow.execute(part);
            match result {
                "A" => { log.push_str(" -> A"); println!("{:?}: {}", &part, log); return true},
                "R" => { log.push_str("-> R"); println!("{:?}: {}", &part, log); return false; },
                _ => workflow = self.workflows.get(result).unwrap(),
            }
            log.push_str("-> ");
            log.push_str(result);
        }
    }
}

#[derive(Debug, Clone)]
struct WorkItem {
    cond_list: Vec<Condition>,
    current_workflow: String,
    current_rule: usize,
}

impl WorkItem {
    // How many options for variable 'var' pass the conditions in the list?
    fn count_options_char(&self, var: char) -> i64 {
        // Get all conditions that apply to this variable
        // Get the largest max condition and the lowest min condition
        let mut min = 1;
        let mut max = 4000;

        for cond in &self.cond_list {
            if cond.variable == var {
                match cond.op {
                    '<' => if max > cond.value { max = cond.value - 1; },
                    '>' => if min < cond.value { min = cond.value + 1; },
                    _ => (),
                }
            }
        }
        if max <= min { return 0; }
        (max - min) + 1
    }

    fn count_options(&self) -> i64 {
        let opt_x = self.count_options_char('x');
        let opt_m = self.count_options_char('m');
        let opt_a = self.count_options_char('a');
        let opt_s = self.count_options_char('s');
        let count = opt_x * opt_m * opt_a * opt_s;
        println!("Score: x {} m {} a {} s {} = {}", opt_x, opt_m, opt_a, opt_s, count);
        count

    }
}

impl AOCProblem for Day19 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        if self.parsing_parts {
            let line_str = &line[1..line.len()-1];  // Strip {}
            let line_split = line_str.split(",");
            let mut part = Part {x: 0, m: 0, a: 0, s: 0};
            for token in line_split {
                let mut t_split = token.split("=");
                let var = t_split.next().unwrap();
                let val = t_split.next().unwrap().parse::<i64>().unwrap();
                match var {
                    "x" => part.x = val,
                    "m" => part.m = val,
                    "a" => part.a = val,
                    "s" => part.s = val,
                    _ => panic!("Can't parse part: {}", token),
                }
            }
            println!("Adding part: {:?}", &part);
            self.parts.push(part);

        } else if line.len() == 0 {
            self.parsing_parts = true;
        } else {
            let mut line_iter = line.split(|c| c == '{' || c == '}');
            // First is the label
            let label = line_iter.next().unwrap();
            let workflow_str = line_iter.next().unwrap();
            let workflow_iter = workflow_str.split(",");
            let mut rule_vec = Vec::new();
            for token in workflow_iter {
                let mut token_split = token.split(":");
                let cond_str = token_split.next().unwrap();
                let ift = token_split.next();
                let rule = match ift {
                    None => Rule{cond: None, if_true: String::from(cond_str)},
                    Some(lbl) => {
                        let mut cond_split = cond_str.split(|c| c == '>' || c == '<');
                        let variable = cond_split.next().unwrap().chars().next().unwrap();
                        let value = cond_split.next().unwrap().parse::<i64>().unwrap();
                        let mut op = '>';
                        if cond_str.contains("<") { op = '<'};
                        let cond = Condition {op, variable, value};
                        Rule{cond: Some(cond), if_true: lbl.to_string()}
                    }
                };
                rule_vec.push(rule);
            }
            let workflow = Workflow{rules: rule_vec};
            println!("{:?}", &workflow);
            self.workflows.insert(label.to_string(), workflow);
        }   
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        let mut score = 0;
        for part in &self.parts {
            if self.test_part(&part) {
                let part_score = part.score();
                println!("Accepted part: {:?} Score: {}", part, part_score);
                score += part_score;
            }
        }
        score.to_string()
    }

    fn compute_b(&mut self) -> String {
        let mut work_queue: VecDeque<WorkItem> = VecDeque::new();
        let empty_cond: Vec<Condition> = Vec::new();
        let mut accept_paths: Vec<WorkItem> = Vec::new();
        let start = WorkItem {cond_list: empty_cond, current_workflow: "in".to_string(), current_rule: 0};
        work_queue.push_front(start);
        while let Some(next) = work_queue.pop_front().to_owned() {
            println!("Handling {:?}", next);
            if "R".eq(&next.current_workflow) {
                continue;
            }
            if "A".eq(&next.current_workflow) {
                accept_paths.push(next.clone());
                continue;
            }
            let workflow = self.workflows.get(&next.current_workflow).unwrap();
            let rule = &workflow.rules[next.current_rule];
            match rule.cond {
                None => {
                    if rule.if_true.eq("R") {
                        // Done with this flow, rejected
                        continue;
                    } else if rule.if_true.eq("A") {
                        accept_paths.push(next.clone());
                        continue;
                    } else {
                        let mut work_item_false = next.clone();
                        work_item_false.current_rule = 0;
                        work_item_false.current_workflow = rule.if_true.clone();
                        work_queue.push_front(work_item_false);
                    }
                },
                Some(c) => {
                    // For each rule, there are 2 paths to try, it's true or false.  Push both on
                    let mut cond_vec_f = next.cond_list.clone();
                    let neg_cond = c.negate();
                    cond_vec_f.push(neg_cond);
                    let work_item_false = WorkItem{cond_list: cond_vec_f, current_workflow: next.current_workflow, current_rule: next.current_rule + 1};
                    work_queue.push_front(work_item_false);

                    let mut cond_vec1 = next.cond_list.clone();
                    cond_vec1.push(c);
                    let work_item_true = WorkItem{cond_list: cond_vec1, current_workflow: rule.if_true.clone(), current_rule: 0};
                    work_queue.push_front(work_item_true);
                            
                }
            }
            println!("WorkQueue size: {}", work_queue.len());            
        }

        let mut count: i64 = 0;
        for accept_path in accept_paths {
            println!("Accept Path: {:?}", accept_path);
            let combos = accept_path.count_options();

            println!("Combos: {}", combos);
            count += combos;
        }
        return count.to_string();
    }
}
