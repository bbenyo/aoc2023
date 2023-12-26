use crate::{Config, AOCProblem};
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Edge {
    x: usize,
    y: usize, 
    steps: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Node {
    // Start
    x: usize,
    y: usize,
    edges: Vec<Edge>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct WorkItem {
    x: usize,
    y: usize,
    fx: usize, // Where we came from
    fy: usize,
    
    cur_steps: i32, // Steps we've taken in a search so far
    cur_path: Vec<(usize, usize)>,
}

impl Node {
    fn new(x: usize, y: usize) -> Node {
        Node {
            x, y,
            edges: Vec::new(),
        }
    }
}

pub struct Day23 {
    board: Vec<Vec<char>>,
    nodes: HashMap<(usize, usize), Node>,
    variant: bool,
    max: i32,
}

fn print_board(board: &Vec<Vec<char>>) {
    println!("Board:\n");
    for i in 0..board.len() {
        let foo = &board[i].iter().collect::<String>();
        println!("{:?}", foo);
    }
}

impl Day23 {
    pub fn new() -> Day23{
        Day23 {
            board: Vec::new(),
            nodes: HashMap::new(),
            variant: false,
            max: 0,
        }
    }

    // X,Y: Potential neighbor we're checking
    // FX, FY:  Place we're at now, x/y is a neighbor
    // LastX, LastY: Where we came to x,y from.  Since we can't go back, this isn't a valid neighbor
    fn is_valid(&self, x: usize, y: usize, fx: usize, fy: usize, last_x: usize, last_y: usize) -> bool {
        if y == last_y && x == last_x {return false;}
        if self.variant {
            if self.board[y][x] == '#' { return false; } else { return true;}
        }
        match self.board[y][x] {
            '#' => false,
            '.' => true,
            '>' => x != fx - 1,
            '<' => x != fx + 1,
            'v' => y != fy - 1,
            '^' => y != fy + 1,
            _ => false,
        }
    }

    fn get_neighbors(&self, x: usize, y: usize, from_x: usize, from_y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if y > 0 && self.is_valid(x, y - 1, x, y, from_x, from_y) {
            neighbors.push((x, y - 1));
        }
        if x > 0 && self.is_valid(x - 1, y, x, y, from_x, from_y) {
            neighbors.push((x - 1, y));
        }
        if y < self.board.len() - 1 && self.is_valid(x, y + 1, x, y, from_x, from_y) {
            neighbors.push((x, y + 1));
        }
        if x < self.board[y].len() - 1 && self.is_valid(x + 1, y, x, y, from_x, from_y) {
            neighbors.push((x + 1, y));
        }
        neighbors
    }

    fn get_end_node(&self) -> (usize, usize) {
        let h = self.board.len();
        let w = self.board[h - 1].len();
        (w-2, h-1)
    }

    // Follow the path starting at n, until we hit a choice point
    fn follow_to_choice(&self, wi: &WorkItem) -> Edge {
        let mut neighbors = self.get_neighbors(wi.x, wi.y, wi.fx, wi.fy);
        let mut cur_loc = (wi.x, wi.y);
        let mut steps = 0;
        while neighbors.len() == 1 {
            let n1 = neighbors[0];
            steps += 1;
            neighbors = self.get_neighbors(n1.0, n1.1, cur_loc.0, cur_loc.1);
            cur_loc.0 = n1.0; // New current spot we moved to
            cur_loc.1 = n1.1;
            //println!("From {},{} to {},{} Neighbors {:?}", n.fx, n.fy, n1.0, n1.1, neighbors);
        }
        let e = Edge{x: cur_loc.0, y: cur_loc.1, steps};
        e
    }
    
    fn create_nodes(&mut self) {
        // Start at 0,1
        let mut work_list: VecDeque<WorkItem> = VecDeque::new();
        let node = Node::new(1, 0);
        self.nodes.insert((1,0), node);
        let work_item1 = WorkItem {x: 1, y: 0, cur_steps: 0, cur_path: Vec::new(), fx: 1, fy: 0};
        work_list.push_front(work_item1);
        while let Some(wi ) = work_list.pop_front() {
            println!("Create: Working on {:?}", &wi);
            let edge = self.follow_to_choice(&wi);
            println!("\tFollowed to {:?}", &edge);
            let o_node = self.nodes.get_mut(&(wi.fx, wi.fy)).unwrap();
            o_node.edges.push(edge.clone());
            let end_node = self.get_end_node();
            if edge.x == end_node.0 && edge.y == end_node.1 {
                continue;
            }
            // Do we have this node already?
            let new_node_opt = self.nodes.get(&(edge.x, edge.y));
            if let None = new_node_opt {
                let new_node = Node::new(edge.x, edge.y);
                self.nodes.insert((edge.x, edge.y), new_node);
                let neighbors = self.get_neighbors(edge.x, edge.y, wi.fx, wi.fy);
                for nxt in &neighbors {
                    println!("\tAdding new path to follow: {:?}", &nxt);
                    let new_wi = WorkItem{x: nxt.0, y: nxt.1, fx: edge.x, fy: edge.y, cur_steps: 0, cur_path: Vec::new()};
                    work_list.push_front(new_wi);
                }   
            }
        }
        let en = self.get_end_node();
        let end_node = Node::new(en.0, en.1);
        self.nodes.insert((end_node.x, end_node.y), end_node);
    }

    fn all_paths(&mut self) -> Vec<i32> {
        let mut path_lengths = Vec::new();
        let mut work_list = VecDeque::new();
        let mut start_wi = WorkItem{x: 1,y: 0, fx: 1, fy: 0, cur_steps: 0, cur_path: Vec::new()};
        start_wi.cur_path.push((1,0));
        work_list.push_front(start_wi);
        let end_node = self.get_end_node();
        while let Some(wi) = work_list.pop_front() {
            //println!("Work Queue: {}", work_list.len());
            let cur_pos = (wi.x, wi.y);
            let cur_node = self.nodes.get(&cur_pos).unwrap();
            for e in &cur_node.edges {
                if wi.cur_path.contains(&(e.x, e.y)) {
                    continue;
                }
                let mut len = wi.cur_steps + e.steps + 1;
                let mut new_item = WorkItem{x: e.x, y: e.y, fx: cur_pos.0, fy: cur_pos.1, 
                    cur_steps: len, cur_path: Vec::new()};
                if end_node == (e.x, e.y) {
                    // No +1 for the end step
                    len = len - 1;
                    //println!("Found path to end through {:?} = {}", wi, new_item.cur_steps);
                    path_lengths.push(len);
                    if len > self.max {
                        println!("Found new max path length {}", len);
                        self.max = len;
                    }
                } else {
                    //println!("\tAdding {:?}", new_item);
                    let mut cp2 = wi.cur_path.clone();
                    new_item.cur_path.append(&mut cp2);
                    new_item.cur_path.push((e.x, e.y));
                    work_list.push_front(new_item);
                }            
            }
        }
        path_lengths
    }
}

impl AOCProblem for Day23 {

    fn handle_line(&mut self, line: &str, config: &Config) {
        let row: Vec<char> = line.chars().collect();
        self.board.push(row);
        if config.variant {  // TODO: Move to initialization method
            self.variant = true;
        }
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        print_board(&self.board);
        self.create_nodes();
        let f_node_o = self.nodes.get_mut(&(137,137));
        if let Some(f_node) = f_node_o {
            f_node.edges.clear();
            f_node.edges.push(Edge{x: 139, y: 140, steps: 4});
        } 
        for n in &self.nodes {
            println!("Node: {:?}", n);
        }
        let path_lengths = self.all_paths();
        path_lengths.iter().max().unwrap().to_string()
    }

    fn compute_b(&mut self) -> String {
        return self.compute_a();
    }
}
