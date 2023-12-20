use crate::{Config, AOCProblem};

#[derive(Debug)]
struct Op {
    dir: char,
    steps: usize,
    // In the end, we didn't need to store this
    // color: String,  
}

pub struct Day18 {
    board: Vec<Vec<char>>,
    dug: Vec<Vec<char>>,
    opcodes: Vec<Op>,
    test: bool,
}

fn _print_board(board: &Vec<Vec<char>>) {
    println!("Board:\n");
    for i in 0..board.len() {
        let foo = &board[i].iter().collect::<String>();
        println!("{:?}", foo);
    }
}

impl Day18 {
    // Let's pick a large enough board to allocate, rather than trying to dynamically size
    
    pub fn new() -> Day18 {
        Day18 {
            board: Vec::new(),
            dug: Vec::new(),
            opcodes: Vec::new(),
            test: true,
        }
    }

    fn init_board(&mut self, width: usize, height: usize) {
        for _ in 0..height {
            let row = vec!['.'; width];
            self.board.push(row);
            let drow = vec!['.'; width];
            self.dug.push(drow);
        }
    }

    fn dig(&mut self, start_x: i64, start_y: i64) -> i64 {
        let mut cur_x: i64 = start_x;
        let mut cur_y = start_y;
        let mut area = 0;
        let mut last_x = start_x;
        let mut last_y = start_y;
        let mut len: i64 = 0;
        for op in &self.opcodes {
            println!("Digging {}{} from {},{}", op.dir, op.steps, cur_x, cur_y);
            for _ in 1..op.steps+1 {
                match op.dir {
                    'R' => cur_x += 1,
                    'D' => cur_y += 1,
                    'L' => cur_x -= 1,
                    'U' => cur_y -= 1,
                    _ => eprintln!("Unrecognized operation {:?}", op),
                }
            }
            
            // Shoelace formula, A = 1/2 SUM (y0+y1)(x0-x1)
            // We consider the perimeter to be inside the polygon too, so add steps
            area += (cur_x + last_x) * (cur_y - last_y);
            len += op.steps as i64;
            last_x = cur_x;
            last_y = cur_y;
        }
        len = len + 1; // Include start
        area = area / 2;
        // Pick's theorem
        area + (len / 2) + 1
    }

}

impl AOCProblem for Day18 {
    fn handle_line(&mut self, line: &str, config: &Config) {
        //let line_str: String = String::from(line);
        let mut line_iter = line.split_whitespace();

        let mut dir = line_iter.next().unwrap().chars().next().unwrap();
        let steps = line_iter.next().unwrap().parse::<usize>().unwrap();
        let mut color = String::from(line_iter.next().unwrap());
        color.retain(|c| !"()#".contains(c));        

        if !config.variant {
            let op = Op {dir, steps};
            self.opcodes.push(op);
        } else {
            // Variant.  Convert color hex digits to the real number
            let hex_num = i32::from_str_radix(&color[0..5], 16).unwrap();
            match &color[5..6] {
                "0" => dir = 'R',
                "1" => dir = 'D',
                "2" => dir = 'L',
                "3" => dir = 'U',
                _ => eprintln!("Unable to parse hex {}", color),
            }
            let op = Op {dir, steps: hex_num as usize};
            self.opcodes.push(op);
        }
        if config.test_input { self.test = true };
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        let poly_interior: i64;
        if self.test {
            self.init_board(10, 10);
            poly_interior = self.dig(0, 0);
        } 
        else {
            self.init_board(490, 450);
            poly_interior = self.dig(100, 300);
        }
        poly_interior.to_string()
    }

    fn compute_b(&mut self) -> String {
        return self.compute_a();
    }
}
