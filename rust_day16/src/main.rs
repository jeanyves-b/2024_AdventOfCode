#[derive(Clone, Debug)]
struct Raindeer {
    position: (usize, usize),
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

    step1(&mut maze.clone());
    step2(&mut maze.clone());
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
            position: left(rednose.position),
            direction: '<',
        },
        'v' => Raindeer {
            position: right(rednose.position),
            direction: '>',
        },
        '<' => Raindeer {
            position: down(rednose.position),
            direction: 'v',
        },
        '>' => Raindeer {
            position: up(rednose.position),
            direction: '^',
        },
        c => panic!("error, found {c} in filtered list"),
    }
}
fn get_right(rednose: Raindeer) -> Raindeer {
    match rednose.direction {
        '^' => Raindeer {
            position: right(rednose.position),
            direction: '>',
        },
        'v' => Raindeer {
            position: left(rednose.position),
            direction: '<',
        },
        '<' => Raindeer {
            position: up(rednose.position),
            direction: '^',
        },
        '>' => Raindeer {
            position: down(rednose.position),
            direction: 'v',
        },
        c => panic!("error, found {c} in filtered list"),
    }
}
fn get_front(rednose: Raindeer) -> Raindeer {
    match rednose.direction {
        '^' => Raindeer {
            position: up(rednose.position),
            direction: '^',
        },
        'v' => Raindeer {
            position: down(rednose.position),
            direction: 'v',
        },
        '<' => Raindeer {
            position: left(rednose.position),
            direction: '<',
        },
        '>' => Raindeer {
            position: right(rednose.position),
            direction: '>',
        },
        c => panic!("error, found {c} in filtered list"),
    }
}

fn solve_water_rec(maze: &mut Vec<Vec<(char, u64)>>, rednose: Raindeer, depth: u64) -> u64 {
    use std::cmp;
    if depth < maze[rednose.position.1][rednose.position.0].1 {
        maze[rednose.position.1][rednose.position.0].1 = depth;
    }

    if rednose.position == (maze[1].len() - 2, 1) {
        return depth;
    }

    let frontdeer = get_front(rednose.clone());
    let mut front = u64::MAX;
    if maze[frontdeer.position.1][frontdeer.position.0].0 != '#'
        && maze[frontdeer.position.1][frontdeer.position.0].1 > depth
    {
        front = solve_water_rec(maze, frontdeer, depth + 1);
    }

    let rightdeer = get_left(rednose.clone());
    let mut right = u64::MAX;
    if maze[rightdeer.position.1][rightdeer.position.0].0 != '#'
        && maze[rightdeer.position.1][rightdeer.position.0].1 > depth
    {
        right = solve_water_rec(maze, rightdeer, depth + 1001);
    }

    let leftdeer = get_right(rednose.clone());
    let mut left = u64::MAX;
    if maze[leftdeer.position.1][leftdeer.position.0].0 != '#'
        && maze[leftdeer.position.1][leftdeer.position.0].1 > depth
    {
        left = solve_water_rec(maze, leftdeer, depth + 1001);
    }

    return cmp::min(front, cmp::min(left, right));
}

fn step1(maze: &mut Vec<Vec<(char, u64)>>) {
    let rednose = Raindeer {
        position: (1, maze.len() - 2),
        direction: '>',
    };
    let found = solve_water_rec(&mut maze.clone(), rednose, 0);
    println!("Step 1 total = {found}");
}

fn solve_path_rec(
    maze: &mut Vec<Vec<(char, u64)>>,
    rednose: Raindeer,
    depth: u64,
) -> (u64, Vec<(usize, usize)>) {
    if rednose.position == (maze[1].len() - 2, 1) {
        if depth < maze[rednose.position.1][rednose.position.0].1 {
            maze[rednose.position.1][rednose.position.0].1 = depth;
        }
        return (depth, vec![rednose.position]);
    }

    let frontdeer = get_front(rednose.clone());
    let mut front: (u64, Vec<(usize, usize)>) = (u64::MAX, vec![]);
    if maze[frontdeer.position.1][frontdeer.position.0].0 != '#' {
        if depth <= maze[rednose.position.1][rednose.position.0].1 {
            maze[rednose.position.1][rednose.position.0].1 = depth;
            front = solve_path_rec(maze, frontdeer, depth + 1);
        }
    } else {
        if depth + 1000 < maze[rednose.position.1][rednose.position.0].1 {
            maze[rednose.position.1][rednose.position.0].1 = depth + 1000;
        }
    }

    let rightdeer = get_left(rednose.clone());
    let mut right: (u64, Vec<(usize, usize)>) = (u64::MAX, vec![]);
    if maze[rightdeer.position.1][rightdeer.position.0].0 != '#'
        && maze[rightdeer.position.1][rightdeer.position.0].1 >= depth
    {
        right = solve_path_rec(maze, rightdeer, depth + 1001);
    }

    let leftdeer = get_right(rednose.clone());
    let mut left: (u64, Vec<(usize, usize)>) = (u64::MAX, vec![]);
    if maze[leftdeer.position.1][leftdeer.position.0].0 != '#'
        && maze[leftdeer.position.1][leftdeer.position.0].1 >= depth
    {
        left = solve_path_rec(maze, leftdeer, depth + 1001);
    }

    use std::cmp;
    let min_value = cmp::min(front.0, cmp::min(left.0, right.0));
    let mut paths: Vec<(usize, usize)> = vec![rednose.position];
    if min_value == u64::MAX {
        return (u64::MAX, paths);
    }
    if front.0 == min_value {
        paths.append(&mut front.1);
    }
    if right.0 == min_value {
        paths.append(&mut right.1);
    }
    if left.0 == min_value {
        paths.append(&mut left.1);
    }
    return (min_value, paths);
}

fn find_paths_rec(
    maze: &Vec<Vec<(char, u64)>>,
    (x, y): (usize, usize),
    paths: &mut Vec<(usize, usize)>,
) {
    paths.push((x, y));
    let value = maze[y][x].1;
    let (mut next_x, mut next_y) = up((x, y));
    if maze[next_y][next_x].1 < value
        && maze[next_y][next_x].0 != '#'
        && !paths.contains(&(next_x, next_y))
    {
        find_paths_rec(maze, (next_x, next_y), paths)
    }
    (next_x, next_y) = right((x, y));
    if maze[next_y][next_x].1 < value
        && maze[next_y][next_x].0 != '#'
        && !paths.contains(&(next_x, next_y))
    {
        find_paths_rec(maze, (next_x, next_y), paths)
    }
    (next_x, next_y) = down((x, y));
    if maze[next_y][next_x].1 < value
        && maze[next_y][next_x].0 != '#'
        && !paths.contains(&(next_x, next_y))
    {
        find_paths_rec(maze, (next_x, next_y), paths)
    }
    (next_x, next_y) = left((x, y));
    if maze[next_y][next_x].1 < value
        && maze[next_y][next_x].0 != '#'
        && !paths.contains(&(next_x, next_y))
    {
        find_paths_rec(maze, (next_x, next_y), paths)
    }
}

fn step2(maze: &mut Vec<Vec<(char, u64)>>) {
    let rednose = Raindeer {
        position: (1, maze.len() - 2),
        direction: '>',
    };

    let (path_length, mut total) = solve_path_rec(maze, rednose, 0);
    total = total.into_iter().fold(vec![], |mut acc, p| {
        if !acc.contains(&p) {
            acc.push(p);
        }
        acc
    });
    println!("total = {total:?}");
    let mut total2: Vec<(usize, usize)> = vec![];
    find_paths_rec(&maze.clone(), (maze[1].len() - 2, 1), &mut total2);
    println!("total2 = {total2:?}");
    println!("Step 2 total = {path_length} => {}", total.len());
    println!("Step 2 total2 = {path_length} => {}", total2.len());

    let mut s = "".to_string();
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            if maze[y][x].1 < u64::MAX {
                if maze[y][x].1 > 99999 {
                    s.push_str(&"trop ".to_string())
                } else {
                    s.push_str(&maze[y][x].1.to_string());
                    if maze[y][x].1 < 10 {
                        s.push(' ');
                    }
                    if maze[y][x].1 < 100 {
                        s.push(' ');
                    }
                    if maze[y][x].1 < 1000 {
                        s.push(' ');
                    }
                    if maze[y][x].1 < 10000 {
                        s.push(' ');
                    }
                }
            } else {
                s.push(maze[y][x].0);
                s.push_str(&"----".to_string());
            }
            s.push(' ')
        }
        s.push('\n');
    }
    println!("store :\n{s}");
}
