use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct Equation {
    result: i64,
    operation: Vec<i64>,
}

impl Equation {
    fn find_equal(&self) -> bool {
        let mut vec_in = self.operation.clone();
        let mut vec_out: Vec<i64> = vec![];
        let input: Vec<i64> = vec_in.split_off(1);
        for i in input {
            for j in vec_in {
                vec_out.push(i + j);
                vec_out.push(i * j);
                vec_out.push((j.to_string() + &i.to_string()).parse::<i64>().unwrap());
            }
            vec_in = vec![];
            vec_in.append(&mut vec_out);
        }

        vec_in.contains(&self.result)
    }
}

fn main() {
    //let binding = read_to_string("files/test.txt").unwrap();
    let binding = read_to_string("files/input.txt").unwrap();
    let input: Vec<Equation> = binding
        .lines()
        .map(|line| {
            let Some((result, operation)) = line.split_once(": ").map(|(result, operation)| {
                (
                    result.parse::<i64>().unwrap(),
                    operation
                        .split(' ')
                        .map(|number| number.parse::<i64>().unwrap())
                        .collect(),
                )
            }) else {
                panic!("this cannot happend");
            };
            Equation { result, operation }
        })
        .collect();

    step1(&input.clone());
    //step2(input.clone());
}

fn step1(input: &[Equation]) {
    let total: i64 = input
        .iter()
        .map(|line| match line.find_equal() {
            true => line.result,
            false => 0,
        })
        .collect::<Vec<i64>>()
        .into_iter()
        .sum();
    println!("step1 : {}", total);
}
