fn main() {
    //let binding = include_str!("../files/small.txt");
    //let binding = include_str!("../files/test.txt");
    let binding = include_str!("../files/input.txt");

    step1(binding);
    step2(binding);
}

fn step1(binding: &str) {
    let mut strings = binding.split("\n\n");
    let store: Vec<Vec<char>> = strings
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut obstacles: Vec<(usize, usize)> = vec![];
    let mut crates: Vec<(usize, usize)> = vec![];
    let mut robot: (usize, usize) = (0, 0);
    for y in 0..store.len() {
        for x in 0..store[y].len() {
            match store[y][x] {
                '#' => obstacles.push((x, y)),
                'O' => crates.push((x, y)),
                '@' => robot = (x, y),
                '.' => {}
                pouet => println!("error, found {pouet} => {} in ({x}, {y})", store[x][y]),
            }
        }
    }

    let mouvs = strings
        .next()
        .unwrap()
        .chars()
        .filter(|c| "^><v".to_string().contains(*c));
    mouvs.for_each(|c| {
        let mut m: Mouvement = m_null;
        let mut t: Tester = t_null;
        match c {
            '^' => {
                m = m_up;
                t = t_up
            }
            'v' => {
                m = m_down;
                t = t_down
            }
            '<' => {
                m = m_left;
                t = t_left
            }
            '>' => {
                m = m_right;
                t = t_right
            }
            _ => println!("error, found {c} in filtered list"),
        }
        let mut to_move = vec![];
        let mut next_point = t(robot.clone());
        while crates.contains(&next_point) {
            to_move.push((next_point.0, next_point.1));
            m(&mut next_point);
        }
        if !obstacles.contains(&next_point) {
            to_move.iter().for_each(|(x, y)| {
                let pos: usize = crates
                    .iter()
                    .position(|(cx, cy): &(usize, usize)| *cx == *x && *cy == *y)
                    .unwrap();
                m(&mut crates[pos]);
            });
            m(&mut robot);
        }
    });

    let total1 = crates.iter().fold(0, |mut acc, (x, y)| {
        acc += y * 100 + x;
        acc
    });
    println!("step 1 total = {total1}");
    print_store(
        store.clone(),
        obstacles.clone(),
        crates.clone(),
        robot.clone(),
    );
}

fn print_store(
    store: Vec<Vec<char>>,
    obstacles: Vec<(usize, usize)>,
    crates: Vec<(usize, usize)>,
    robot: (usize, usize),
) {
    let mut s: String = "".to_string();
    for y in 0..store.len() {
        for x in 0..store[y].len() {
            if obstacles.contains(&(x, y)) {
                s.push('#');
            } else if crates.contains(&(x, y)) {
                s.push('O');
            } else if robot.0 == x && robot.1 == y {
                s.push('@');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("store :\n{s}");
}
fn print_store_large(
    store: Vec<Vec<char>>,
    obstacles: Vec<(usize, usize)>,
    crates: Vec<(usize, usize)>,
    robot: (usize, usize),
) {
    let mut s: String = "".to_string();
    for y in 0..store.len() {
        for x in 0..store[y].len() {
            if obstacles.contains(&(x, y)) {
                s.push('#');
            } else if crates.contains(&(x, y)) {
                let pos: usize = crates
                    .iter()
                    .position(|(cx, cy): &(usize, usize)| *cx == x && *cy == y)
                    .unwrap();
                if pos % 2 == 0 {
                    s.push('[');
                } else {
                    s.push(']');
                }
            } else if robot.0 == x && robot.1 == y {
                s.push('@');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("store :\n{s}");
}

type Mouvement = fn(&mut (usize, usize));
fn m_null((_x, _y): &mut (usize, usize)) {}
fn m_up((_x, y): &mut (usize, usize)) {
    *y = *y - 1;
}
fn m_down((_x, y): &mut (usize, usize)) {
    *y = *y + 1;
}
fn m_left((x, _y): &mut (usize, usize)) {
    *x = *x - 1
}
fn m_right((x, _y): &mut (usize, usize)) {
    *x = *x + 1
}
type Tester = fn((usize, usize)) -> (usize, usize);
fn t_null((x, y): (usize, usize)) -> (usize, usize) {
    (x, y)
}
fn t_up((x, y): (usize, usize)) -> (usize, usize) {
    (x, y - 1)
}
fn t_down((x, y): (usize, usize)) -> (usize, usize) {
    (x, y + 1)
}
fn t_left((x, y): (usize, usize)) -> (usize, usize) {
    (x - 1, y)
}
fn t_right((x, y): (usize, usize)) -> (usize, usize) {
    (x + 1, y)
}

fn get_pos(crates: &Vec<(usize, usize)>, (x, y): (usize, usize)) -> usize {
    crates
        .iter()
        .position(|(cx, cy): &(usize, usize)| *cx == x && *cy == y)
        .unwrap()
}

fn compute_to_move_rec(
    m: Mouvement,
    t: Tester,
    to_move: &mut Vec<usize>,
    obstacles: Vec<(usize, usize)>,
    crates: Vec<(usize, usize)>,
    current: (usize, usize),
) -> bool {
    let tester = t(current);
    if obstacles.contains(&tester) {
        return false;
    } else if crates.contains(&tester) {
        let pos: usize = get_pos(&crates, tester);
        let other_crate;
        let other_pos;
        if pos % 2 == 0 {
            other_pos = pos + 1;
            other_crate = crates[pos + 1].clone();
        } else {
            other_pos = pos - 1;
            other_crate = crates[pos - 1].clone();
        }
        let (mut one, mut two) = (true, true);
        if !to_move.contains(&pos) {
            to_move.push(pos);
            if t(tester) != other_crate {
                one = compute_to_move_rec(m, t, to_move, obstacles.clone(), crates.clone(), tester);
            }
        }
        if !to_move.contains(&other_pos) {
            to_move.push(other_pos);
            two = compute_to_move_rec(
                m,
                t,
                to_move,
                obstacles.clone(),
                crates.clone(),
                other_crate,
            );
        }
        return one && two;
    } else {
        return true;
    }
}

fn step2(binding: &str) {
    let mut strings = binding.split("\n\n");
    let store: Vec<Vec<char>> = strings
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .fold("".to_string(), |mut acc, c| {
                    acc = acc
                        + match c {
                            '#' => "##",
                            '@' => "@.",
                            'O' => "[]",
                            '.' => "..",
                            _ => "",
                        };
                    acc
                })
                .chars()
                .collect()
        })
        .collect();
    let mut obstacles: Vec<(usize, usize)> = vec![];
    let mut crates: Vec<(usize, usize)> = vec![];
    let mut robot: (usize, usize) = (0, 0);
    for y in 0..store.len() {
        for x in 0..store[y].len() {
            match store[y][x] {
                '#' => obstacles.push((x, y)),
                '@' => robot = (x, y),
                '[' | ']' => crates.push((x, y)),
                '.' => {}
                _ => println!("error, found {} in ({x}, {y})", store[x][y]),
            }
        }
    }

    print_store_large(
        store.clone(),
        obstacles.clone(),
        crates.clone(),
        robot.clone(),
    );
    let mouvs: Vec<char> = strings
        .next()
        .unwrap()
        .chars()
        .filter(|c| "^><v".to_string().contains(*c))
        .collect();
    mouvs.iter().for_each(|c| {
        let mut m: Mouvement = m_null;
        let mut t: Tester = t_null;
        match c {
            '^' => {
                m = m_up;
                t = t_up
            }
            'v' => {
                m = m_down;
                t = t_down
            }
            '<' => {
                m = m_left;
                t = t_left
            }
            '>' => {
                m = m_right;
                t = t_right
            }
            _ => println!("error, found {c} in filtered list"),
        }
        let mut to_move = vec![];
        if compute_to_move_rec(
            m,
            t,
            &mut to_move,
            obstacles.clone(),
            crates.clone(),
            robot.clone(),
        ) {
            to_move.iter().for_each(|i| m(&mut crates[*i]));
            m(&mut robot);
        }
    });

    let mut total2 = 0;
    let mut i = 0;
    while i < crates.len() {
        let (x, y) = crates[i];
        total2 += y * 100 + x;
        i += 2;
    }
    println!("step 2 total = {total2}");
    print_store_large(
        store.clone(),
        obstacles.clone(),
        crates.clone(),
        robot.clone(),
    );
}
