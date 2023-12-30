use crate::{Config, AOCProblem};
use std::fs::File;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug)]
struct Node {
    label: String,
    edges: Vec<String>,
}

impl Node {
    fn add_edge(&mut self, e: String) {
        self.edges.push(e.clone());
    }
}

pub struct Day25 {
    nodes: HashMap<String, Node>,
}

impl Day25 {
    pub fn new() -> Day25 {
        Day25 {
            nodes: HashMap::new(),
        }
    }

    fn add_missing_nodes(&mut self) {
        let mut add_extra_nodes = Vec::new();
        // TODO: Refactor, remove the excessive cloning
        for item in &self.nodes {
            for e in &item.1.edges {
                if !self.nodes.contains_key(e) {
                    add_extra_nodes.push(e.clone());
                }
            }
        }
        for e in &add_extra_nodes {
            self.nodes.insert(e.clone(), Node{label: e.clone(), edges: Vec::new()});
        }
    }

    fn add_edge(&mut self, n1: String, n2: String) {
        let other_node = self.nodes.get_mut(&n2).unwrap();
        other_node.add_edge(n1.clone());
    }

    fn get_bidirectional_edges(&self) -> Vec<(String, String)> {
        let mut add: Vec<(String, String)> = Vec::new();
         for n in self.nodes.values() {
            for e in &n.edges {
                add.push((n.label.clone(), e.clone()));
            }
        }
        add
    }

    fn add_bidirectional_edges(&mut self) {
        let add = self.get_bidirectional_edges();
        for e in add {
            self.add_edge(e.0, e.1);
        }
    }

    fn get_group_size_containing(&mut self, lbl: String) -> usize {
        println!("Node size: {}", self.nodes.len());
        let node_s = self.nodes.get_mut(&lbl).unwrap();
        let mut work_list: Vec<String> = Vec::new();
        work_list.push(node_s.label.clone());
        let mut cluster: Vec<String> = Vec::new();
        while let Some(l) = work_list.pop() {
            if cluster.contains(&l) {
                continue;
            }
            println!("Adding {} to the cluster", l);
            let n = self.nodes.get(&l).unwrap();
            for e in &n.edges {
                if !cluster.contains(&e) {
                    work_list.push(e.clone());
                }
            }
            cluster.push(l);
        }
        cluster.len()
    }
}

impl AOCProblem for Day25 {
    fn handle_line(&mut self, line: &str, _config: &Config) {
        let mut line_iter = line.split_whitespace();
        let lbl = line_iter.next().unwrap().replace(":","");
        let mut node = Node{label: lbl, edges: Vec::new()};
        for token in line_iter {
            node.edges.push(token.to_string());
        }
        // TODO: Figure out a better way to do this without cloning the string?
        self.nodes.insert(node.label.clone(), node);
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        // We need to add nodes to the map for nodes that are only defined by edges
        // e.g. a: b, and there's no corresponding b line in the input
        self.add_missing_nodes();
        self.add_bidirectional_edges();
        let mut ofile = File::create("day25.dot").unwrap();
        writeln!(ofile, "graph d25 {{").unwrap();
        //for item in &self.nodes {
        //    writeln!(ofile, "\tnode [label=\"{}\"];", item.label).unwrap();
        //}
        for item in &self.nodes {
            for e in &item.1.edges {
                writeln!(ofile, "\t{} -- {} [label=\"{}-{}\"];", item.1.label, e, item.1.label, e).unwrap();
            }
        }        
        writeln!(ofile, "}}").unwrap();

        // View the graph with graphviz, neato layout
        // neato -Tsvg day25.dot -o d25_neato.svg
        // The three edges to cut are obvious on the neato layout image
        // For my input, LDL-FPG, HCF-LHN, NXK-DFK
        
        // Remove these edges
        let node_ldl = self.nodes.get_mut("ldl").unwrap();
        let idx = node_ldl.edges.iter().position(|x| *x == "fpg").unwrap();
        node_ldl.edges.remove(idx);
        let node_hcf = self.nodes.get_mut("hcf").unwrap();
        let idx = node_hcf.edges.iter().position(|x| *x == "lhn").unwrap();
        node_hcf.edges.remove(idx);
        let node_nxk = self.nodes.get_mut("nxk").unwrap();
        let idx = node_nxk.edges.iter().position(|x| *x == "dfk").unwrap();
        node_nxk.edges.remove(idx);

        // Gather all nodes in group 1 containing LDL
        let c1 = self.get_group_size_containing("ldl".to_string());
        println!("C1: {}", c1);
        let c2 = self.nodes.len() - c1;
        println!("Cluster sizes {} and {}", c1, c2);
        (c1 * c2).to_string()
    }

    fn compute_b(&mut self) -> String {
        return self.compute_a();
    }
}
