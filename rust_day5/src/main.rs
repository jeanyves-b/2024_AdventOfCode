use std::fs::read_to_string;

fn main() {
    //let binding = read_to_string("files/test.txt").unwrap();
    let binding = read_to_string("files/input.txt").unwrap();

    let mut input = binding.split("\r\n\r\n");
    let rules = input
        .next()
        .unwrap()
        .lines()
        .map(|rule| {
            let mut a = rule.split("|");
            (a.next().unwrap(), a.next().unwrap())
        })
        .collect::<Vec<(_, _)>>();

    let lines = input
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    step1(rules.clone(), lines.clone());
    step2(rules.clone(), lines.clone());
}

fn step1(rules: Vec<(&str, &str)>, lines: Vec<Vec<&str>>) {
    let resutl: i32 = lines
        .iter()
        .map(|line| match apply_rules(line, &rules) {
            true => line[(line.len()) / 2].parse::<i32>().unwrap(),
            false => 0,
        })
        .sum();

    println!("step1 : {}", resutl);
}

fn step2(rules: Vec<(&str, &str)>, lines: Vec<Vec<&str>>) {
    let resutl: i32 = lines
        .iter()
        .map(|line| match apply_rules(line, &rules) {
            true => 0,
            false => order(line, &rules)[(line.len()) / 2]
                .parse::<i32>()
                .unwrap(),
        })
        .sum();

    println!("step2 : {}", resutl);
}

fn order<'a>(line: &'a Vec<&'a str>, rules: &Vec<(&str, &str)>) -> Vec<&'a str> {
    let mut ordered_line: Vec<&str> = vec![line[0]];
    for i in 1..line.len() {
        ordered_line.push(line[i]);
        for j in (1..=i).rev() {
            match apply_rules(&ordered_line, rules) {
                true => break,
                false => ordered_line.swap(j, j - 1),
            }
        }
    }
    ordered_line
}

fn apply_rule(slice: &[&str], before: &str, after: &str) -> bool {
    slice[slice.len() - 1] != before
        || slice[slice.len() - 1] == before && !slice[0..slice.len()].contains(&after)
}

fn apply_rules(line: &Vec<&str>, rules: &Vec<(&str, &str)>) -> bool {
    for i in 1..line.len() {
        for (before, after) in rules.iter() {
            if !apply_rule(&line[0..=i], before, after) {
                return false;
            }
        }
    }
    true
}
