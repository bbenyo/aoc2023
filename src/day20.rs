use crate::{Config, AOCProblem};
use std::{collections::HashMap, collections::hash_map::Entry};

#[derive(Debug)]
struct Pulse {
    from: String,
    high: bool,
    to: String,
}

#[derive(Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
    Output,
}

// TODO: Make these labels string slices instead
#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    label: String,
    outputs: Vec<String>,
    state: bool,
    input_map: HashMap<String, bool>,
}

impl Module {

    fn pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        match self.module_type {
            ModuleType::Broadcast => self.broadcast(pulse),
            ModuleType::FlipFlop => self.flipflop(pulse),
            ModuleType::Conjunction => self.conjunction(pulse),
            ModuleType::Output => Vec::new(),
        }
    }

    fn broadcast(&self, pulse: &Pulse) -> Vec<Pulse> {
        let mut pulses = Vec::new();
        for o in &self.outputs {
            let pulse = Pulse { from: self.label.clone(), high: pulse.high, to: o.to_string()};
            pulses.push(pulse);
        }
        pulses
    }

    fn flipflop(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        if pulse.high {
            // Ignore high pulse
            return Vec::new();
        }
        self.state = !self.state;
        let mut pulses = Vec::new();
        for o in &self.outputs {
            let pulse = Pulse { from: self.label.clone(), high: self.state, to: o.to_string()};
            pulses.push(pulse);
        }
        pulses
    }
    
    fn conjunction(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        let from = &pulse.from;
        self.input_map.insert(from.clone(), pulse.high);
        let mut all_high = true;
        for (_, value) in self.input_map.iter() {
            if !value { all_high = false};
        }
        let to_send = !all_high;
        
        let mut pulses = Vec::new();
        for o in &self.outputs {
            let pulse = Pulse { from: self.label.clone(), high: to_send, to: o.to_string()};
            pulses.push(pulse);
        }
        pulses
    }
}


pub struct Day20 {
    modules: HashMap<String, Module>,
    input_map: HashMap<String, Vec<String>>,
}

impl Day20 {
    pub fn new() -> Day20 {
        Day20 {
            modules: HashMap::new(),
            input_map: HashMap::new(),
        }
    }

    fn initialize(&mut self) {
        // Could iterate through the outputs for each module and define Output
        //  Modules for every output that doesn't exist, but we'll take a shortcut
        let output = Module {module_type: ModuleType::Output, 
            label: "output".to_string(), outputs: Vec::new(), state: false, input_map: HashMap::new()};
        self.modules.insert("output".to_string(), output);    
        let rx = Module {module_type: ModuleType::Output, 
            label: "rx".to_string(), outputs: Vec::new(), state: false, input_map: HashMap::new()};
        self.modules.insert("rx".to_string(), rx);    

        for module in self.modules.values_mut() {
            if let ModuleType::Conjunction = module.module_type {
                println!("Setting Conjunction inputs for {}", module.label);
                let i_vec = self.input_map.get(&module.label);
                match i_vec {
                    None => (),
                    Some(vec) => {
                        for v in vec {
                            module.input_map.insert(v.to_string(), false);
                        }
                    }
                }
            }            
        }
    }

    fn press_button(&mut self) -> (i64, i64, bool, bool, bool, bool) {
        let pulse = Pulse{from: "button".to_string(), high: false, to: "broadcaster".to_string()};
        let mut pulses = Vec::new();
        pulses.push(pulse);

        let mut high_count = 0;
        let mut low_count = 0;
        let mut high_th = false;
        let mut high_ch = false;
        let mut high_sv = false;
        let mut high_gh = false;
        while pulses.len() > 0 {
            let mut new_pulses: Vec<Pulse> = Vec::new();
            for p in &pulses {
                //println!("{:?}", p);
                if p.high { high_count +=1 } else { low_count += 1 };
                if p.from.eq("th") && p.high == true {
                    high_th = true;
                }
                if p.from.eq("ch") && p.high == true {
                    high_ch = true;
                }
                if p.from.eq("sv") && p.high == true {
                    high_sv = true;
                }
                if p.from.eq("gh") && p.high == true {
                    high_gh = true;
                }
                let m = self.modules.get_mut(&p.to).unwrap();
                let output_pulses = m.pulse(p);
                for op in output_pulses {
                    new_pulses.push(op);
                }
            }
            pulses.clear();
            pulses = new_pulses;
        }
        
        (low_count, high_count, high_ch, high_gh, high_sv, high_th)
    }
}

impl AOCProblem for Day20 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let mut line_iter = line.split("->");
        let lbl = line_iter.next().unwrap().trim();
        let name: &str;
        if "broadcaster".eq(lbl) {
            name = lbl; 
        } else {
            name = &lbl[1..];
        }
        let outputs = line_iter.next().unwrap();
        let mut o_vec = Vec::new();
        let output_split = outputs.split(",");
        for o in output_split {
            let o_str = o.trim().to_string();
            o_vec.push(o_str.clone());
            let _values = match self.input_map.entry(o_str) {
                Entry::Occupied(o) => {
                    o.into_mut().push(name.to_string());
                },
                Entry::Vacant(v) => {
                    let mut new_vec = Vec::new();
                    new_vec.push(name.to_string());
                    v.insert(new_vec);
                }
            };
        }

        if "broadcaster".eq(lbl) {
            let broadcaster = Module{module_type: ModuleType::Broadcast, 
                    label: lbl.to_string(), outputs: o_vec, state: false, input_map: HashMap::new()};
            self.modules.insert(broadcaster.label.clone(), broadcaster);
        } else if lbl.starts_with("%") {
            let flipflop = Module {module_type: ModuleType::FlipFlop,
                label: name.to_string(), state: false, outputs: o_vec, input_map: HashMap::new()};
            self.modules.insert(name.to_string(), flipflop);
        } else if lbl.starts_with("&") {
            let conj = Module{module_type: ModuleType::Conjunction, 
                label: name.to_string(), input_map: HashMap::new(), 
                outputs: o_vec, state: false};
            self.modules.insert(name.to_string(), conj);
        }       
    }
    
    fn compute_a(&mut self) -> String {
        println!("Input Map: {:?}", self.input_map);
        self.initialize();
        for m in &self.modules {
            println!("{} = {:?}", m.0, m.1);
        }
        // Let's try without caching or cycle detection
        let mut low_count = 0;
        let mut high_count = 0;
        for _ in 0..1000 {
            let counts = self.press_button();            
            println!("Low Count: {}", counts.0);
            println!("High Count: {}", counts.1);
            println!("Single Low RX: {}", counts.2);
            low_count += counts.0;
            high_count += counts.1;
        }
        println!("Total Low Count: {}", low_count);
        println!("Total High Count: {}", high_count);
        (low_count * high_count).to_string()
    }

    fn compute_b(&mut self) -> String {
        // Surely this won't work, just simulating it

        // &cn -> rx 
        // th, sv, gh, ch -> cn 
        // when do all th, sv, gh, ch go high?
        
        // Looks like they go high every so often, in a cycle
        // LCM of the cycles should be when they're all high
        // This only works for this specific input
        self.initialize();
        let mut presses = 1;
        let mut high_inputs: (i64, i64, i64, i64) = (0, 0, 0, 0);
        loop {
            let counts = self.press_button();
            if counts.2 && high_inputs.0 == 0 {
                high_inputs.0 = presses;
            }
            if counts.3 && high_inputs.1 == 0 {
                high_inputs.1 = presses;
            }
            if counts.4 && high_inputs.2 == 0 {
                high_inputs.2 = presses;
            }
            if counts.5 && high_inputs.3 == 0 {
                high_inputs.3 = presses;
            }
            match high_inputs {
                (0,_,_,_) | (_,0,_,_) | (_,_,0,_) | (_,_,_,0)=> (),
                _ => break,
            }
            presses += 1;
        }
        println!("High Input first seen: {:?}", high_inputs);
        (high_inputs.0 * high_inputs.1 * high_inputs.2 * high_inputs.3).to_string()
    }
}