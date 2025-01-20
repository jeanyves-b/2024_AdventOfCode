fn main() {
    //let mut binding = include_str!("../files/test.txt").split("\n\n");
    let mut binding = include_str!("../files/input.txt").split("\n\n");
    let towels = binding.next().unwrap().split(", ").collect::<Vec<&str>>();
    let paterns = binding.next().unwrap().lines().collect::<Vec<&str>>();

    //println!("{towels:?} -- {paterns:?}");
    step1(&towels, &paterns);
    step2(&towels, &paterns);
}

enum OptCmp {
    Nope,
    Equal,
    Contains,
}

fn c_cmp(s1: &str, s2: &str) -> OptCmp {
    let mut max = OptCmp::Equal;
    if s1.len() > s2.len() {
        return OptCmp::Nope;
    } else if s1.len() < s2.len() {
        max = OptCmp::Contains;
    }
    for (i, c) in s1.chars().enumerate() {
        if c != s2.chars().nth(i).unwrap() {
            return OptCmp::Nope;
        }
    }
    max
}

fn is_buildable(towels: &Vec<&str>, patern: &str) -> bool {
    let mut testers: Vec<String> = vec!["".to_string()];
    while testers.len() > 0 {
        let t = testers.pop().unwrap();
        for s in towels {
            let t_s = t.clone() + s;
            match c_cmp(s, &patern.to_string().drain(t.len()..).collect::<String>()) {
                OptCmp::Equal => return true,
                OptCmp::Contains => testers.push(t_s),
                OptCmp::Nope => {}
            }
        }
    }
    false
}

fn step1(towels: &Vec<&str>, paterns: &Vec<&str>) {
    let mut result = 0;
    for patern in paterns {
        if is_buildable(towels, patern) {
            result += 1;
        }
    }
    println!("step 1 : {result}");
}

fn nb_builds(towels: &mut Vec<&str>, patern: &str) -> u32 {
    towels.sort();
    let mut result = 0;
    let mut testers: Vec<String> = vec!["".to_string()];
    while testers.len() > 0 {
        let t = testers.pop().unwrap();
        for s in &mut *towels {
            let t_s = t.clone() + s;
            match c_cmp(s, &patern.to_string().drain(t.len()..).collect::<String>()) {
                OptCmp::Equal => result += 1,
                OptCmp::Contains => testers.push(t_s),
                OptCmp::Nope => {}
            }
        }
    }
    result
}

fn step2(towels: &Vec<&str>, paterns: &Vec<&str>) {
    let mut result = 0;
    for patern in paterns {
        result += nb_builds(&mut towels.clone(), patern)
    }
    println!("step 2 : {result}");
}
