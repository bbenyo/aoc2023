use crate::{Config, AOCProblem};

#[derive(Debug, PartialEq, Clone)]
struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
    index: u32,
    
    // Calculate these and store so we don't have to redo the computations
    slope: f64,
    c: f64,
}

impl Hailstone {
    fn new(v: Vec<i64>, idx: u32) -> Hailstone {
        let mut h = Hailstone {x: v[0], y: v[1], z: v[2], vx: v[3], vy: v[4], vz: v[5], index: idx,
            slope: 0., c: 0.};
        h.slope = h.get_slope();
        h.c = h.get_cross();
        h
    }

    fn get_slope(&self) -> f64 {
        // y = ax + b
        // Slope = vy/vx
        return self.vy as f64 / self.vx as f64;
    } 

    fn get_cross(&self) -> f64 {
        // y = ax + b
        // b = y - ax
        return self.y as f64 - (self.get_slope() * self.x as f64);
    }

    fn get_intersect(&self, other: &Hailstone) -> Option<(f64,f64,f64)> {
        // Slopes equal: parallel
        if self.slope == other.slope { return None; }
        // X = c1 - c2 / slope2 - slope1
        let x: f64 = (self.c - other.c) / (other.slope - self.slope);
        // Y = ax+b
        let y = (self.slope * x as f64) + self.c;
        Some((x, y, self.z as f64))
    }

    // Will the hailstone make it to the x/y/z position passed in, in the future?
    //   Or would it have hit that position in the past?  Just check velocities
    fn future_pos_xy(&self, pos: (f64, f64, f64)) -> bool {
        if (pos.0 > self.x as f64) && self.vx < 0 {
            return false;
        }
        if (pos.0 < self.x as f64) && self.vx > 0 {
            return false;
        }
        if (pos.1 > self.y as f64) && self.vy < 0 {
            return false;
        }
        if (pos.1 < self.y as f64) && self.vy > 0 {
            return false;
        }
        return true;
    }
}

pub struct Day24 {
    stones: Vec<Hailstone>,
    variant: bool,
    test: bool,
}

impl Day24 {
    pub fn new() -> Day24 {
        Day24 {
            stones: Vec::new(),
            variant: false,
            test: true,
        }
    }
}

fn intersection<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut c: Vec<T> = Vec::new();
    for e1 in a {
        if b.contains(&e1) {
            c.push(e1);
        }
    }
    c
}

fn get_potential_v(x1: i64, x2: i64, vx: i64) -> Vec<i64> {
    let mut new_vx_set: Vec<i64> = Vec::new();
    let diff = x2 - x1;                
    for pv in -2000..2000 {
        let vdiff = pv - vx;
        if pv != vx {
            if diff % vdiff == 0 {
                new_vx_set.push(pv);
            }
        }
    }
    new_vx_set
}

impl AOCProblem for Day24 {
    fn handle_line(&mut self, line: &str, config: &Config) {
        self.variant = config.variant;
        self.test = config.test_input;
        let parts: Vec<i64> = line
            .split(|c: char| c == ',' || c == '@' || c.is_ascii_whitespace())
            .filter(|p| !p.is_empty())
            .map(|p| p.parse::<i64>().unwrap())
            .collect();
        let index = self.stones.len();
        let stone = Hailstone::new(parts, index as u32);
        self.stones.push(stone);      
    }
    
    // Just count the items in the list
    fn compute_a(&mut self) -> String {
        let area_min: f64;
        let area_max: f64;
        if self.test {
            area_min = 7.;
            area_max = 27.;
        } else {
            area_min = 200000000000000.;
            area_max = 400000000000000.;
        }
        let mut count = 0;
        for a in 0..self.stones.len() {
            let stone = &self.stones[a];
            for b in (a+1)..self.stones.len() {
                let stone2 = &self.stones[b];
                //println!("Hailstone: {:?} vs {:?}", stone, stone2);
                let intersect = stone.get_intersect(&stone2);
                //println!("\t{:?}", intersect);
                if let Some(i) = intersect {
                    if i.0 >= area_min && i.0 <= area_max && i.1 >= area_min && i.1 <= area_max {
                        if !stone.future_pos_xy(i) {
                            println!("\tInside test area, in the past for {}", stone.index);
                        } else if !stone2.future_pos_xy(i) {
                            println!("\tInside test area, in the past for {}", stone2.index);
                        } else {
                            println!("\tInside test area, in the future!");
                            count += 1; 
                        }
                    }
                }
            }
        }
        count.to_string()
    }

    fn compute_b(&mut self) -> String {
        let mut rock = Hailstone{x: 0, y: 0, z: 0, vx: 0, vy: 0, vz: 0, index: 0, slope: 0., c: 0.};
        let mut pvx = Vec::new();
        let mut pvy: Vec<i64> = Vec::new();
        let mut pvz: Vec<i64> = Vec::new();

        // Inspiration from u/TheZigerionScammer 
        //  https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/keqf8uq/
        
        // For two hailstones moving at the same velocity for 1 dimension
        //   The rock we throw must intersect them both at the same velocity
        //   Since velocities are integers, the rock velocity has to evenly divide the 
        //   difference between the hailstone positions, since they're both going the
        //   same speed, and will thus keep the same distance from eachother
        //  If we don't get multiple stones with the same velocity along each component, this won't work
        for a in 0..self.stones.len() {
            let stone = &self.stones[a];
            for b in (a+1)..self.stones.len() {
                let stone2 = &self.stones[b];
                if stone.vx == stone2.vx {
                    let mut new_vx_set = get_potential_v(stone.x, stone2.x, stone.vx);
                    if pvx.len() == 0 {
                        pvx.append(&mut new_vx_set);
                    } else {
                        pvx = intersection(pvx, new_vx_set);
                    }
                }
                if stone.vy == stone2.vy {
                    let mut new_vy_set = get_potential_v(stone.y, stone2.y, stone.vy);
                    if pvy.len() == 0 {
                        pvy.append(&mut new_vy_set);
                    } else {
                        pvy = intersection(pvy, new_vy_set);
                    }
                }
                if stone.vz == stone2.vz {
                    let mut new_vz_set = get_potential_v(stone.z, stone2.z, stone.vz);
                    if pvz.len() == 0 {
                        pvz.append(&mut new_vz_set);
                    } else {
                        pvz = intersection(pvz, new_vz_set);
                    }
                }
            }
        }

        println!("Potential X velocities: {:?}", pvx);
        println!("Potential Y velocities: {:?}", pvy);
        println!("Potential Z velocities: {:?}", pvz);

        // Could search if we got multiple options, but I got one possibility for each vx/vy/vz
        if pvx.len() != 1 || pvy.len() != 1 || pvz.len() != 1 {
            panic!("Multiple possible options, add a loop to try them all, brute force");
        }
        rock.vx = pvx[0];
        rock.vy = pvy[0];
        rock.vz = pvz[0];

        // Pick hailstone1
        // subtract rock velocity to get a line that gives potential thrown rock positions
        let s1 = &self.stones[0];
        let mut h1 = Hailstone{x: s1.x, y: s1.y, z: s1.z, vx: s1.vx - rock.vx, vy: s1.vy - rock.vy, vz: s1.vz - rock.vz, index: 0, slope: 0., c: 0.};
        h1.slope = h1.get_slope();
        h1.c = h1.get_cross();
        // Get another line from another hailstone
        let s2 = &self.stones[1];
        let mut h2 = Hailstone{x: s2.x, y: s2.y, z: s2.z, vx: s2.vx - rock.vx, vy: s2.vy - rock.vy, vz: s2.vz - rock.vz, index: 0, slope: 0., c: 0.};
        h2.slope = h2.get_slope();
        h2.c = h2.get_cross();
        let isect = h1.get_intersect(&h2);
        println!("{:?}", h1);
        println!("{:?}", h2);
        println!("{:?}", isect);
        
        rock.x = ((h2.c  - h1.c) / (h1.slope - h2.slope)).round() as i64;
        rock.y = ((h1.slope * rock.x as f64) + h1.c).round() as i64;
        let t3 = (rock.x - s1.x) / (s1.vx - rock.vx);
        rock.z = s1.z + (s1.vz - rock.vz) * t3;
        rock.slope = rock.get_slope();
        rock.c = rock.get_cross();

        // Have the answer here as rock, let's just test it
        println!("{:?}", rock);
        for s in &self.stones {
            let isect = rock.get_intersect(&s);
            match isect {
                None => eprintln!("Rock doesn't intersect {:?}", s),
                Some(intersect) => {
                    //println!("Intersects at {:?}", intersect);
                    if !rock.future_pos_xy(intersect) {
                        eprintln!("\tin the past!");
                        eprintln!("\tStone {:?}", s);
                    }
                }
            }
        }
        return (rock.x + rock.y + rock.z).to_string();
        //return self.compute_a();
    }
}
