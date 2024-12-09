fn main() {
    let binding = include_str!("../files/test.txt");
    //let binding = include_str!("../files/input.txt");
    let disk: Vec<Vec<isize>> = binding
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .fold(vec![], |mut acc, (i, character)| {
                    acc.append(
                        &mut (0..character.to_string().parse::<i8>().unwrap())
                            .map(|_| if i % 2 == 0 { (i / 2) as isize } else { -1 })
                            .collect::<Vec<isize>>(),
                    );
                    acc
                })
        })
        .collect();

    step1(&disk);
}

fn find_last(line: &Vec<isize>) -> Option<usize> {
    for (i, elem) in line.iter().rev().enumerate().rev() {
        if *elem != -1 {
            return Some(i);
        }
    }
    None
}

fn step1(disk: &Vec<Vec<isize>>) {
    let total = disk.clone().iter_mut().fold(0, |tot, line| {
        tot + {
            let mut i: usize = 0;
            while line.contains(&-1) {
                if line[i] == -1 {
                    let l = find_last(line).unwrap();
                    line.swap(i, l);
                    while *line.last().unwrap() == -1 {
                        line.pop();
                    }
                }
                i += 1;
            }
            line.iter()
                .enumerate()
                .fold(0, |acc, (j, element)| acc + j * *element as usize)
        }
    });

    println!("step1 : {total}");
}
