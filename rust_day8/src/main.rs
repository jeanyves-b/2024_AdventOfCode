use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct Network {
    antenna: char,
    positions: Vec<(usize, usize)>,
}

impl Network {
    fn calculate_nods(&self, roof: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = vec![];
        let x_size: usize = roof.len();
        let y_size: usize = roof[0].len();
        for (i, pos_0) in self.positions.iter().enumerate() {
            for j in i + 1..self.positions.len() {
                let pos_1 = self.positions[j];
                let x_dep = pos_1.0 as isize - pos_0.0 as isize;
                let y_dep = pos_1.1 as isize - pos_0.1 as isize;

                result.push(pos_1);
                result.push(*pos_0);

                let mut x_pre: isize = pos_0.0 as isize - x_dep;
                let mut y_pre: isize = pos_0.1 as isize - y_dep;
                while x_pre < x_size as isize && x_pre >= 0 && y_pre < y_size as isize && y_pre >= 0
                {
                    result.push((x_pre as usize, y_pre as usize));
                    x_pre -= x_dep;
                    y_pre -= y_dep;
                }

                let mut x_post: isize = pos_1.0 as isize + x_dep;
                let mut y_post: isize = pos_1.1 as isize + y_dep;
                while x_post >= 0
                    && y_post >= 0
                    && x_post < x_size as isize
                    && y_post < y_size as isize
                {
                    result.push((x_post as usize, y_post as usize));
                    x_post += x_dep;
                    y_post += y_dep;
                }
            }
        }
        result
    }
}

fn find_tuple<'a>(
    list: &'a mut Vec<Network>,
    letter: &'a char,
) -> Option<&'a mut Vec<(usize, usize)>> {
    for network in list {
        if network.antenna == *letter {
            return Some(&mut network.positions);
        }
    }
    None
}

fn main() {
    //let binding = read_to_string("files/test.txt").unwrap();
    let binding = read_to_string("files/input.txt").unwrap();

    let roof: Vec<Vec<char>> = binding.lines().map(|line| line.chars().collect()).collect();
    let mut list_points: Vec<Network> = vec![];
    roof.iter().enumerate().for_each(|(x, line)| {
        line.iter()
            .enumerate()
            .for_each(|(y, letter)| match letter {
                '.' => {}
                _ => match find_tuple(&mut list_points, letter) {
                    Some(positions) => positions.push((x as usize, y as usize)),
                    None => list_points.push(Network {
                        antenna: *letter,
                        positions: vec![(x, y)],
                    }),
                },
            })
    });

    step1(&roof.clone(), &list_points);
    //step2(input.clone());
}

fn step1(roof: &Vec<Vec<char>>, list_points: &Vec<Network>) {
    let mut list_nods: Vec<(usize, usize)> = vec![];
    list_points.iter().for_each(|network| {
        network.calculate_nods(roof).iter().for_each(|(x, y)| {
            list_nods.push((*x, *y));
        })
    });
    let mut seen = HashSet::new();
    list_nods.retain(|pos| {
        let is_first = !seen.contains(pos);
        seen.insert(*pos);
        is_first
    });
    let result = list_nods.len();
    println!("step1 : {result}");
}
