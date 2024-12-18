#[derive(Clone, Debug)]
struct Mouvement {
    x: u64,
    y: u64,
}
#[derive(Clone, Debug)]
struct Machine {
    a: Mouvement,
    b: Mouvement,
    target_x: u64,
    target_y: u64,
}
impl Machine {
    fn compute(&self) -> Option<u64> {
        for a in 1..(self.target_x / self.a.x) {
            if (self.target_x - a*self.a.x) % self.b.x == 0 {
                let b = (self.target_x - a*self.a.x) / self.b.x;
                if a*self.a.y + b*self.b.y == self.target_y {
                    return Some(3*a + b);
                }
            }
        }
        None
    }
    fn cramer_compute(&self) -> Option<u128> {
        let det = (self.a.x * self.b.y) as i128 - (self.a.y * self.b.x) as i128;
        let det_sub_a = (self.target_x * self.b.y) as i128 - (self.target_y * self.b.x) as i128;
        let det_sub_b = (self.a.x * self.target_y) as i128 - (self.a.y * self.target_x) as i128;
    
        if det == 0 || det_sub_a % det != 0 || det_sub_b % det != 0 {
            None
        } else {
            Some((det_sub_a / det *3 + det_sub_b / det) as u128)
        }
    }
}

fn parse_mouvement(s: &str) -> Mouvement {
    let nums: Vec<u64> = s
        .split(",")
        .map(|w| {
            w.to_string()
                .chars()
                .fold("".to_string(), |mut acc, c| {
                    if "0123456789".contains(c) {
                        acc.push(c);
                    }
                    acc
                })
                .parse::<u64>()
                .unwrap()
        })
        .collect();
    Mouvement {
        x: nums[0],
        y: nums[1],
    }
}

fn parse_machine(s: &str) -> Machine {
    let lines = s.lines();
    let mouv: Vec<Mouvement> = lines.map(|line| parse_mouvement(line)).collect();
    Machine {
        a: mouv[0].clone(),
        b: mouv[1].clone(),
        target_x: mouv[2].x,// + 10000000000000,
        target_y: mouv[2].y,// + 10000000000000,
    }
}

fn main() {
    //let binding = include_str!("../files/test.txt");
    let binding = include_str!("../files/input.txt");

    let machines: Vec<Machine> = binding
        .split("\n\n")
        .map(|machine| parse_machine(machine))
        .collect();

    step1(machines.clone());
    step2(&mut machines.clone());
}

fn step1(machines: Vec<Machine>) {
    let total = machines.iter().fold(0, |mut acc, m| {
        match m.compute() {
            Some(x) => acc += x,
            _ => acc += 0,
        };
        acc
    });
    println!("total1 = {total}");
}

fn step2(machines: &mut Vec<Machine>) {
    for m in machines.iter_mut() {
        m.target_x += 10000000000000; 
        m.target_y += 10000000000000;
    }
    let total = machines.iter().fold(0, |mut acc, m| {
        match m.cramer_compute() {
            Some(x) => acc += x,
            _ => acc += 0,
        };
        acc
    });
    println!("total2 = {total}");
}
