use std::collections::BTreeMap;
fn main() {
    //let binding = include_str!("../files/test.txt");
    let binding = include_str!("../files/input.txt");

    let stones: Vec<u64> = binding
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    step1(stones.clone());
    step2(stones.clone());
}

fn step1(stones: Vec<u64>) {
    let mut mut_stones = stones.clone();
    (0..25).for_each(|_| {
        mut_stones = mut_stones
            .iter()
            .fold(vec![], |mut acc: Vec<u64>, stone: &u64| {
                match stone {
                    0 => acc.push(1),
                    _ => match stone.to_string().len() % 2 {
                        0 => {
                            let stone_chars = stone.to_string();
                            let (a, b) = stone_chars.split_at(stone_chars.len() / 2);
                            acc.push(a.parse::<u64>().unwrap());
                            acc.push(b.parse::<u64>().unwrap());
                        }
                        1 => acc.push(stone * 2024),
                        _ => panic!("AAAAAAAAAAAAAAAA {stone}"),
                    },
                };
                acc
            })
            .clone();
    });
    let total = mut_stones.len();
    println!("step1: {total}");
}

fn compute_stone(stone: u64) -> Vec<u64> {
    let mut result = vec![];
    match stone {
        0 => result.push(1),
        _ => match stone.to_string().len() % 2 {
            0 => {
                let stone_chars = stone.to_string();
                let (a, b) = stone_chars.split_at(stone_chars.len() / 2);
                result.push(a.parse::<u64>().unwrap());
                result.push(b.parse::<u64>().unwrap());
            }
            1 => result.push(stone * 2024),
            _ => panic!("AAAAAAAAAAAAAAAA {stone}"),
        },
    };
    result
}

fn compute_recur(cache: &mut BTreeMap<(u64, u8), u64>, stone: u64, depth: u8) -> u64 {
    if let Some(&v) = cache.get(&(stone, depth)) {
        return v;
    }
    let result = match depth {
        0 => 1,
        _ => compute_stone(stone).iter().fold(0, |acc, new_stone| {
            acc + compute_recur(cache, *new_stone, depth - 1)
        }),
    };
    cache.insert((stone, depth), result);
    result
}

fn step2(stones: Vec<u64>) {
    let mut cache = BTreeMap::new();
    let total = stones
        .iter()
        .fold(0, |acc, stone| acc + compute_recur(&mut cache, *stone, 75));
    println!("step2: {total}");
}
