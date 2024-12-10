use std::collections::HashSet;

fn main() {
    //let binding = include_str!("../files/test.txt");
    let binding = include_str!("../files/input.txt");
    let mut starts: Vec<(usize, usize)> = vec![];
    let island: Vec<Vec<i8>> = binding
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == '0' {
                        starts.push((i, j));
                    }
                    c.to_string().parse::<i8>().unwrap()
                })
                .collect()
        })
        .collect();
    step1(&starts.clone(), &island.clone());
    step2(&starts.clone(), &island.clone());
}

fn dedup_point(vector: &mut Vec<(usize, usize)>) {
    let mut seen = HashSet::new();
    vector.retain(|pos| {
        let is_first = !seen.contains(pos);
        seen.insert(*pos);
        is_first
    });
}

fn get_nei(island: &Vec<Vec<i8>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = vec![];
    let value = island[x][y];
    if x > 0 {
        if island[x - 1][y] == value + 1 {
            output.push((x - 1, y))
        }
    }
    if x < island.len() - 1 {
        if island[x + 1][y] == value + 1 {
            output.push((x + 1, y))
        }
    }
    if y > 0 {
        if island[x][y - 1] == value + 1 {
            output.push((x, y - 1))
        }
    }
    if y < island.len() - 1 {
        if island[x][y + 1] == value + 1 {
            output.push((x, y + 1))
        }
    }
    output
}

fn step1(starts: &Vec<(usize, usize)>, island: &Vec<Vec<i8>>) {
    let total = starts.iter().fold(0, |mut acc, point| {
        let mut paths: Vec<(usize, usize)> = vec![*point];
        for _ in 1..10 {
            let mut ends = paths.iter().fold(vec![], |mut acc, point| {
                acc.append(&mut get_nei(island, *point));
                acc
            });
            if ends.is_empty() {
                break;
            }
            dedup_point(&mut ends);
            paths = ends.clone();
        }
        acc += paths.len();
        acc
    });
    println!("step1 : {}", total);
}

fn dedup_rep(vector: &mut Vec<((usize, usize), i64)>) -> Vec<((usize, usize), i64)> {
    let mut output: Vec<((usize, usize), i64)> = vec![];
    let mut seen = HashSet::new();
    vector.retain(|(point, rep)| {
        if seen.contains(point) {
            let pos = output
                .iter()
                .position(|p: &((usize, usize), i64)| p.0 .0 == point.0 && p.0 .1 == point.1)
                .unwrap();
            output[pos].1 += *rep;
            false
        } else {
            output.push((*point, *rep));
            seen.insert(*point);
            true
        }
    });
    output
}

fn step2(starts: &Vec<(usize, usize)>, island: &Vec<Vec<i8>>) {
    let total = starts.iter().fold(0, |mut acc, point| {
        let mut paths: Vec<((usize, usize), i64)> = vec![(*point, 1)];
        for _ in 1..10 {
            let mut ends = paths.iter().fold(vec![], |mut acc, (point, rep)| {
                get_nei(island, *point)
                    .iter()
                    .for_each(|point| acc.push((*point, *rep)));
                acc
            });
            if ends.is_empty() {
                break;
            }
            paths = dedup_rep(&mut ends);
            ends.clear();
        }
        acc += paths.iter().fold(0, |acc, (_, rep)| acc + rep);
        acc
    });
    println!("step2 : {}", total);
}
