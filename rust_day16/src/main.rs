#[derive(Clone, Debug)]
struct Raindeer {
    pos: (usize, usize),
    direction: char,
}

fn main() {
    //let binding = include_str!("../files/small.txt");
    //let binding = include_str!("../files/test.txt");
    let binding = include_str!("../files/input.txt");

    let maze: Vec<Vec<(char, u64)>> = binding
        .lines()
        .map(|line| line.chars().map(|c| (c, u64::MAX)).collect())
        .collect();

    let maze2: Vec<Vec<char>> = binding.lines().map(|line| line.chars().collect()).collect();

    step1(&mut maze.clone());
    step2(&mut maze2.clone());
}

fn up((x, y): (usize, usize)) -> (usize, usize) {
    (x, y - 1)
}
fn down((x, y): (usize, usize)) -> (usize, usize) {
    (x, y + 1)
}
fn left((x, y): (usize, usize)) -> (usize, usize) {
    (x - 1, y)
}
fn right((x, y): (usize, usize)) -> (usize, usize) {
    (x + 1, y)
}
fn get_left(rednose: Raindeer) -> Raindeer {
    match rednose.direction {
        '^' => Raindeer {
            pos: left(rednose.pos),
            direction: '<',
        },
        'v' => Raindeer {
            pos: right(rednose.pos),
            direction: '>',
        },
        '<' => Raindeer {
            pos: down(rednose.pos),
            direction: 'v',
        },
        '>' => Raindeer {
            pos: up(rednose.pos),
            direction: '^',
        },
        c => panic!("error, found {c} in filtered list"),
    }
}
fn get_right(rednose: Raindeer) -> Raindeer {
    match rednose.direction {
        '^' => Raindeer {
            pos: right(rednose.pos),
            direction: '>',
        },
        'v' => Raindeer {
            pos: left(rednose.pos),
            direction: '<',
        },
        '<' => Raindeer {
            pos: up(rednose.pos),
            direction: '^',
        },
        '>' => Raindeer {
            pos: down(rednose.pos),
            direction: 'v',
        },
        c => panic!("error, found {c} in filtered list"),
    }
}
fn get_front(rednose: Raindeer) -> Raindeer {
    match rednose.direction {
        '^' => Raindeer {
            pos: up(rednose.pos),
            direction: '^',
        },
        'v' => Raindeer {
            pos: down(rednose.pos),
            direction: 'v',
        },
        '<' => Raindeer {
            pos: left(rednose.pos),
            direction: '<',
        },
        '>' => Raindeer {
            pos: right(rednose.pos),
            direction: '>',
        },
        c => panic!("error, found {c} in filtered list"),
    }
}

fn solve_water_rec(maze: &mut Vec<Vec<(char, u64)>>, rednose: Raindeer, depth: u64) -> u64 {
    use std::cmp;
    if depth < maze[rednose.pos.1][rednose.pos.0].1 {
        maze[rednose.pos.1][rednose.pos.0].1 = depth;
    }

    if rednose.pos == (maze[1].len() - 2, 1) {
        return depth;
    }

    let frontdeer = get_front(rednose.clone());
    let mut front = u64::MAX;
    if maze[frontdeer.pos.1][frontdeer.pos.0].0 != '#'
        && maze[frontdeer.pos.1][frontdeer.pos.0].1 > depth
    {
        front = solve_water_rec(maze, frontdeer, depth + 1);
    }

    let rightdeer = get_left(rednose.clone());
    let mut right = u64::MAX;
    if maze[rightdeer.pos.1][rightdeer.pos.0].0 != '#'
        && maze[rightdeer.pos.1][rightdeer.pos.0].1 > depth
    {
        right = solve_water_rec(maze, rightdeer, depth + 1001);
    }

    let leftdeer = get_right(rednose.clone());
    let mut left = u64::MAX;
    if maze[leftdeer.pos.1][leftdeer.pos.0].0 != '#'
        && maze[leftdeer.pos.1][leftdeer.pos.0].1 > depth
    {
        left = solve_water_rec(maze, leftdeer, depth + 1001);
    }

    return cmp::min(front, cmp::min(left, right));
}

fn step1(maze: &Vec<Vec<(char, u64)>>) {
    let rednose = Raindeer {
        pos: (1, maze.len() - 2),
        direction: '>',
    };
    let found = solve_water_rec(&mut maze.clone(), rednose, 0);
    println!("Step 1 total = {found}");
}

use std::collections::HashMap;
fn build_paths_rec(
    maze: &mut Vec<Vec<char>>,
    rodolph: Raindeer,
    current_path: Vec<(usize, usize)>,
    d: u64,
    min_d: &mut u64,
    paths: &mut Vec<(usize, usize)>,
    repetitions: &mut HashMap<(usize, usize), u64>,
) -> bool {
    if maze[rodolph.pos.1][rodolph.pos.0] == '#' {
        return false;
    }
    match repetitions.get(&rodolph.pos) {
        Some(x) => {
            if d > x + 1000 {
                return false;
            } else {
                repetitions.entry(rodolph.pos).and_modify(|val| *val = d);
            }
        }
        None => {
            repetitions.insert(rodolph.pos, d);
        }
    }
    if rodolph.pos.0 == maze[1].len() - 2 && rodolph.pos.1 == 1 {
        if d <= *min_d {
            if d < *min_d {
                paths.clear();
            }
            *min_d = d;
            if !paths.contains(&rodolph.pos) {
                paths.push(rodolph.pos);
            }
            return true;
        }
        return false;
    }
    let mut now_path = current_path.clone();
    if current_path.contains(&rodolph.pos) {
        return false;
    } else {
        now_path.push(rodolph.pos);
    }
    let left = build_paths_rec(
        maze,
        get_left(rodolph.clone()),
        now_path.clone(),
        d + 1001,
        min_d,
        paths,
        repetitions,
    );
    let front = build_paths_rec(
        maze,
        get_front(rodolph.clone()),
        now_path.clone(),
        d + 1,
        min_d,
        paths,
        repetitions,
    );
    let right = build_paths_rec(
        maze,
        get_right(rodolph.clone()),
        now_path.clone(),
        d + 1001,
        min_d,
        paths,
        repetitions,
    );
    if left || front || right {
        if !paths.contains(&rodolph.pos) {
            paths.push(rodolph.pos);
        }
        return true;
    } else {
        return false;
    }
}

fn step2(maze: &mut Vec<Vec<char>>) {
    let start = (1, maze.len() - 2);
    let rodolph = Raindeer {
        pos: start,
        direction: '>',
    };
    let mut min_d = u64::MAX;
    let mut paths: Vec<(usize, usize)> = vec![];
    let mut repetitions: HashMap<(usize, usize), u64> = HashMap::new();
    if build_paths_rec(
        maze,
        rodolph,
        vec![],
        0,
        &mut min_d,
        &mut paths,
        &mut repetitions,
    ) {
        let total = paths.len();
        println!("Step 2 total = {total}");
    } else {
        println!("Step 2 error");
    }

    let mut maze_map = "".to_string();
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            if paths.contains(&(x, y)) {
                maze_map.push('O');
            } else {
                maze_map.push(maze[y][x]);
            }
            maze_map.push(' ');
        }
        maze_map.push('\n');
    }
    println!("{maze_map}");
}
