#[derive(Clone, Debug)]
struct Robot {
    start: (i32, i32),
    mouv: (i32, i32),
}
impl Robot {
    fn compute_pos(&self, i: i32, (size_x, size_y): (i32, i32)) -> (i32, i32) {
        (
            (size_x + (self.start.0 + self.mouv.0 * i) % size_x) % size_x,
            (size_y + (self.start.1 + self.mouv.1 * i) % size_y) % size_y,
        )
    }
}

fn parse_robot(s: &str) -> Robot {
    let mut parts = s.split_whitespace();
    let mut start = parts.next().unwrap().split('=').nth(1).unwrap().split(',');
    let start_x = start.next().unwrap().parse::<i32>().unwrap();
    let start_y = start.next().unwrap().parse::<i32>().unwrap();
    let mut mouv = parts.next().unwrap().split('=').nth(1).unwrap().split(',');
    let mouv_x = mouv.next().unwrap().parse::<i32>().unwrap();
    let mouv_y = mouv.next().unwrap().parse::<i32>().unwrap();
    Robot {
        start: (start_x, start_y),
        mouv: (mouv_x, mouv_y),
    }
}

fn safety_factor(pos: Vec<(i32, i32)>, (size_x, size_y): (i32, i32)) -> u64 {
    let mut ne: u64 = 0;
    let mut nw: u64 = 0;
    let mut se: u64 = 0;
    let mut sw: u64 = 0;
    pos.iter().for_each(|(x, y)| {
        if *x < size_x / 2 {
            if *y < size_y / 2 {
                ne += 1;
            } else if *y > size_y / 2 {
                nw += 1;
            }
        } else if *x > size_x / 2 {
            if *y < size_y / 2 {
                se += 1;
            } else if *y > size_y / 2 {
                sw += 1;
            }
        }
    });
    println!("ne : {ne} -- nw : {nw} -- se : {se} -- sw : {sw}");
    let result = ne * nw * se * sw;
    result
}

fn main() {
    let test = false;
    let size_x;
    let size_y;
    let binding;
    if test {
        binding = include_str!("../files/test.txt");
        size_y = 7;
        size_x = 11;
    } else {
        binding = include_str!("../files/input.txt");
        size_y = 103;
        size_x = 101;
    }

    let robots: Vec<Robot> = binding.lines().map(|line| parse_robot(&line)).collect();

    step1(robots.clone(), (size_x, size_y));
    step2(&mut robots.clone(), (size_x, size_y));
}

fn step1(robots: Vec<Robot>, (size_x, size_y): (i32, i32)) {
    let nb_sec = 100;
    let final_pos: Vec<(i32, i32)> = robots
        .iter()
        .map(|r| r.compute_pos(nb_sec, (size_x, size_y)))
        .collect();
    let total = safety_factor(final_pos, (size_x, size_y));
    println!("total step 1 : {total}");
}

fn print_tree(pos: Vec<(i32, i32)>, (size_x, size_y): (i32, i32)) -> String {
    let mut s: String = "".to_string();
    for i in 0..size_y {
        for j in 0..size_x {
            if pos.contains(&(j, i)) {
                s.push('*');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    s
}

fn variance(pos: &Vec<(i32, i32)>) -> f64 {
    let (list_x, list_y) = pos.iter().fold((vec![], vec![]), |(mut lhs, mut rhs), (x,y)| {
        lhs.push(*x);
        rhs.push(*y);
        (lhs, rhs)
    });
    let moy_x:f64 = list_x.iter().sum::<i32>() as f64 / list_x.len() as f64;
    let moy_y:f64 = list_y.iter().sum::<i32>() as f64 / list_y.len() as f64;
    let var_x = list_x.iter().map(|x| (*x as f64 - moy_x).powf(2.0)).collect::<Vec<f64>>().iter().sum::<f64>() / list_x.len() as f64;
    let var_y = list_y.iter().map(|y| (*y as f64 - moy_y).powf(2.0)).collect::<Vec<f64>>().iter().sum::<f64>() / list_y.len() as f64;
    (var_x + var_y) / 2.0
}

fn step2(robots: &mut Vec<Robot>, (size_x, size_y): (i32, i32)) {
    let mut result = 0;
    let mut min:f64 = 10000.0;
    for i in 0..=10000 {
        let final_pos: Vec<(i32, i32)> = robots
            .iter_mut()
            .map(|r| r.compute_pos(i, (size_x, size_y)))
            .collect();
        let current = variance(&final_pos);
        if current < min {
            min = current;
            result = i;
        }
    }
    println!("result step 2 : {result}");
    println!("{}",
        print_tree(robots
            .iter()
            .map(|r| r.compute_pos(result, (size_x, size_y)))
            .collect(), (size_x, size_y))
    );
}
