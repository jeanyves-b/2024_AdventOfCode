fn main() {
    // let binding = include_str!("../files/test.txt");
    // let size = 7;
    // let nb_falling = 12;
    let binding = include_str!("../files/input.txt");
    let size = 71;
    let nb_falling = 1024;
    let falling_bytes: Vec<(usize, usize)> = binding
        .lines()
        .map(|s| {
            let mut sp = s.split(",");
            (
                sp.next().unwrap().parse::<usize>().unwrap(),
                sp.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let mut maze: Vec<Vec<u64>> = vec![vec![]];
    (0..size).for_each(|i| {
        maze.push(vec![]);
        (0..size).for_each(|_| maze[i].push(u64::MAX));
    });

    //print_maze(maze.clone(), &falling_bytes[0..nb_falling], &[]);

    let mut step1_bytes = falling_bytes.clone();
    step1_bytes.truncate(nb_falling);
    step1(maze.clone(), size, &step1_bytes);
    step2(maze.clone(), size, nb_falling, &falling_bytes);
}

fn print_maze(maze: Vec<Vec<u64>>, bytes: &[(usize, usize)], path: &[(usize, usize)]) {
    (0..maze.len()).for_each(|y| {
        (0..maze[y].len()).for_each(|x| {
            if path.contains(&(x, y)) {
                print!("O ");
            } else if bytes.contains(&(x, y)) {
                print!("# ");
            } else {
                print!(". ");
            }
        });
        println!();
    });
}

fn option_cmp(
    l1: Option<Vec<(usize, usize)>>,
    l2: Option<Vec<(usize, usize)>>,
) -> Option<Vec<(usize, usize)>> {
    match (l1, l2) {
        (Some(l1), Some(l2)) => {
            if l1.len() < l2.len() {
                Some(l1)
            } else {
                Some(l2)
            }
        }
        (Some(l), None) | (None, Some(l)) => Some(l),
        (None, None) => None,
    }
}

fn solve_water_rec(
    maze: &mut Vec<Vec<u64>>,
    falling_bytes: &Vec<(usize, usize)>,
    size: usize,
    depth: u64,
    pos: (usize, usize),
    best: &mut u64,
) -> Option<Vec<(usize, usize)>> {
    if depth >= *best {
        return None;
    }
    if pos == (size - 1, size - 1) {
        *best = depth;
        return Some(vec![pos]);
    }
    if depth < maze[pos.1][pos.0] {
        maze[pos.1][pos.0] = depth;
    } else {
        return None;
    }
    if falling_bytes.contains(&pos) {
        return None;
    }

    let mut top = None;
    if pos.0 > 0 {
        top = solve_water_rec(
            maze,
            falling_bytes,
            size,
            depth + 1,
            (pos.0 - 1, pos.1),
            best,
        )
    }
    let mut left = None;
    if pos.1 > 0 {
        left = solve_water_rec(
            maze,
            falling_bytes,
            size,
            depth + 1,
            (pos.0, pos.1 - 1),
            best,
        )
    }
    let mut right = None;
    if pos.1 < size - 1 {
        right = solve_water_rec(
            maze,
            falling_bytes,
            size,
            depth + 1,
            (pos.0, pos.1 + 1),
            best,
        )
    }
    let mut bottom = None;
    if pos.0 < size - 1 {
        bottom = solve_water_rec(
            maze,
            falling_bytes,
            size,
            depth + 1,
            (pos.0 + 1, pos.1),
            best,
        )
    }

    let res = option_cmp(top, option_cmp(left, option_cmp(right, bottom)));
    match res {
        Some(mut x) => {
            x.push(pos);
            Some(x)
        }
        None => None,
    }
}

fn step1(maze: Vec<Vec<u64>>, size: usize, falling_bytes: &Vec<(usize, usize)>) {
    let mut best = u64::MAX;
    let solution = solve_water_rec(&mut maze.clone(), falling_bytes, size, 0, (0, 0), &mut best);
    match solution {
        Some(l) => {
            print_maze(maze, falling_bytes, &l);
            println!("step1 : {}", l.len() - 1);
        }
        None => {
            println!("step 1 : error");
        }
    }
}

fn exist_water_rec(
    maze: &mut Vec<Vec<u64>>,
    falling_bytes: &Vec<(usize, usize)>,
    size: usize,
    depth: u64,
    pos: (usize, usize),
) -> bool {
    if pos == (size - 1, size - 1) {
        return true;
    }
    if depth < maze[pos.1][pos.0] {
        maze[pos.1][pos.0] = depth;
    } else {
        return false;
    }
    if falling_bytes.contains(&pos) {
        return false;
    }

    let mut bottom = false;
    if pos.0 < size - 1 {
        bottom = exist_water_rec(maze, falling_bytes, size, depth + 1, (pos.0 + 1, pos.1))
    }
    if bottom {
        return true;
    }
    let mut right = false;
    if pos.1 < size - 1 {
        right = exist_water_rec(maze, falling_bytes, size, depth + 1, (pos.0, pos.1 + 1))
    }
    if right {
        return true;
    }
    let mut top = false;
    if pos.0 > 0 {
        top = exist_water_rec(maze, falling_bytes, size, depth + 1, (pos.0 - 1, pos.1))
    }
    if top {
        return true;
    }
    let mut left = false;
    if pos.1 > 0 {
        left = exist_water_rec(maze, falling_bytes, size, depth + 1, (pos.0, pos.1 - 1))
    }
    if left {
        return true;
    }

    false
}

fn step2(maze: Vec<Vec<u64>>, size: usize, nb_falling: usize, falling_bytes: &Vec<(usize, usize)>) {
    let mut tested = nb_falling + (falling_bytes.len() - nb_falling) / 2;
    let mut incr = (falling_bytes.len() - nb_falling) / 2;
    let mut save = 3450;
    let mut exists;
    while incr > 1 {
        incr /= 2;
        let mut bytes = falling_bytes.clone();
        bytes.truncate(tested+1);
        save = tested;
        exists = exist_water_rec(&mut maze.clone(), &bytes, size, 0, (0, 0));
        if exists {
            tested += incr;
        } else {
            tested -= incr;
        }
    }
    println!("step 2 : {save} -> {:?}", falling_bytes[save]);
}
