fn main() {
    //let binding = include_str!("../files/test.txt");
    let binding = include_str!("../files/input.txt");

    let mut strings = binding.split("\n\n");
    let regs: Vec<u64> = strings
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(' ').nth(2).unwrap().parse::<u64>().unwrap())
        .collect();

    let instructions: Vec<u8> = strings
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    println!("{regs:?} -- {instructions:?}");
    step1(&mut regs.clone(), &instructions);
    step2(&mut regs.clone(), &instructions);
}

fn combo(op: u8, regs: &Vec<u64>) -> u64 {
    match op {
        0 | 1 | 2 | 3 => op as u64,
        4 | 5 | 6 => regs[(op - 4) as usize],
        _ => panic!("op is out of bound !"),
    }
}

fn adv(operande: u8, regs: &mut Vec<u64>, i: &mut usize) {
    regs[0] = regs[0] / 2_u64.pow(combo(operande, regs) as u32);
    *i = *i + 2;
}

fn bxl(operande: u8, regs: &mut Vec<u64>, i: &mut usize) {
    use std::ops::BitXor;
    regs[1] = regs[1].bitxor(operande as u64);
    *i = *i + 2;
}

fn bst(operande: u8, regs: &mut Vec<u64>, i: &mut usize) {
    use std::ops::BitAnd;
    regs[1] = combo(operande, regs).bitand(7 as u64);
    *i = *i + 2;
}

fn jnz(operande: u8, regs: &Vec<u64>, i: &mut usize) {
    if regs[0] == 0 {
        *i = *i + 2;
    } else {
        *i = operande as usize;
    }
}

fn bxc(regs: &mut Vec<u64>, i: &mut usize) {
    use std::ops::BitXor;
    regs[1] = regs[1].bitxor(regs[2]);
    *i = *i + 2;
}

fn out(operande: u8, regs: &Vec<u64>, i: &mut usize) -> u8{
    use std::ops::BitAnd;
    *i = *i + 2;
    combo(operande, regs).bitand(7 as u64) as u8
}

fn bdv(operande: u8, regs: &mut Vec<u64>, i: &mut usize) {
    regs[1] = regs[0] / 2_u64.pow(combo(operande, regs) as u32);
    *i = *i + 2;
}

fn cdv(operande: u8, regs: &mut Vec<u64>, i: &mut usize) {
    regs[2] = regs[0] / 2_u64.pow(combo(operande, regs) as u32);
    *i = *i + 2;
}

fn execute(func: u8, operande: u8, regs: &mut Vec<u64>, i: &mut usize, list: &mut Vec<u8>) {
    match func {
        0 => adv(operande, regs, i),
        1 => bxl(operande, regs, i),
        2 => bst(operande, regs, i),
        3 => jnz(operande, regs, i),
        4 => bxc(regs, i),
        5 => list.push(out(operande, regs, i)),
        6 => bdv(operande, regs, i),
        7 => cdv(operande, regs, i),
        _ => panic!("wrong function numbre, got {func}"),
    }
}

fn step1(regs: &mut Vec<u64>, instructions: &Vec<u8>) {
    let mut i = 0;
    let mut list = vec![];
    print!("step 1 = ");
    while i < instructions.len() {
        execute(instructions[i], instructions[i + 1], regs, &mut i, &mut list);
    }
    println!("{}", list.iter().fold("".to_string(), |mut acc:String, n:&u8| {acc = acc + &(n.to_string()) + ","; acc}))
}

fn compare_list(l1: &[u8], l2: &[u8]) -> bool {
    if l1.len() != l2.len() {
        return false
    } else {
        for i in 0..l1.len() {
            if l1[i] != l2[i] {
                return false
            }
        } 
    }
    true
}

fn step2(regs: &mut Vec<u64>, instructions: &Vec<u8>) {
    let mut results: Vec<u64> = vec![0];
    let len = instructions.len();
    let mut finished  = false;
    let mut result = 0;

    while !finished && results.len() > 0{
        let test = results.remove(0);
        for res_val in 0..8 {
            let mut list = vec![];
            regs[0] = (test << 3) + res_val;
            let mut i=0;
            while i < instructions.len() {
                execute(instructions[i], instructions[i + 1], regs, &mut i, &mut list);
            }
            if compare_list(&list, &instructions[len-list.len()..]) {
                results.push((test << 3) +res_val);
            }
            if compare_list(&list, &instructions) {
                result = (test << 3) +res_val;
                finished = true;
            }
        }
    }
    println! {"step 2 = {result}"};
}
