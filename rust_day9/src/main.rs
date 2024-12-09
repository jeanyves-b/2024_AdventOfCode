fn main() {
    //let binding = include_str!("../files/test.txt");
    let binding = include_str!("../files/input.txt");
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

    let disk2 = binding
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(i, character)| {
                    (
                        character.to_string().parse::<usize>().unwrap(),
                        if i % 2 == 0 { (i / 2) as isize } else { -1 },
                    )
                })
                .collect::<Vec<(usize, isize)>>()
        })
        .collect::<Vec<Vec<(usize, isize)>>>();

    step2(&disk2);
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

fn find_large_enough(line: &[(usize, isize)], size: usize) -> Option<usize> {
    for (i, elem) in line.iter().enumerate() {
        if elem.0 >= size && elem.1 == -1 {
            return Some(i);
        }
    }
    None
}

fn contains_free(line: &[(usize, isize)]) -> bool {
    for (_, elem) in line.iter() {
        if *elem == -1 {
            return true;
        }
    }
    false
}

fn step2_sort(line: &mut Vec<(usize, isize)>) {
    let mut i = line.len() - 1;
    while contains_free(&line[0..i]) {
        if line[i].1 == -1 {
            i -= 1;
        } else {
            match find_large_enough(&line[0..i], line[i].0) {
                Some(j) => {
                    if line[j].0 == line[i].0 {
                        line.swap(i, j);
                        i -= 1;
                    } else {
                        let size_j = line[j].0;
                        line[j].0 = line[i].0;
                        line[j].1 = line[i].1;
                        line[i].1 = -1;
                        line.insert(j + 1, ((size_j - line[i].0), -1));
                    }
                }
                None => {
                    i -= 1;
                }
            }
        }
    }
}

fn step2(disk: &Vec<Vec<(usize, isize)>>) {
    let total = disk.clone().iter_mut().fold(0, |tot, line| {
        tot + {
            step2_sort(line);
            line.iter()
                .fold(vec![], |mut acc, (size, elem)| {
                    for _ in 0..*size {
                        acc.push(*elem);
                    }
                    acc
                })
                .iter()
                .enumerate()
                .fold(0, |tot, (j, element)| {
                    tot + j as i64 * {
                        match *element {
                            -1 => 0,
                            _ => *element as i64,
                        }
                    }
                })
        }
    });

    println!("step2 : {total}");
}
