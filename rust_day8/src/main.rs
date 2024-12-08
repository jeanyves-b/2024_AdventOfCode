use std::fs::read_to_string;

fn main() {
    let binding = read_to_string("files/test.txt").unwrap();
    //let binding = read_to_string("files/input.txt").unwrap();

    let input: Vec<&str> = binding.lines().collect();

    step1(input.clone());
    //step2(input.clone());
}

fn step1() {}
