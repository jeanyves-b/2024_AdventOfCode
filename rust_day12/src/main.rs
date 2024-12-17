#[derive(Clone, Debug)]
struct Dimention {
    points: Vec<(usize, usize)>,
    perim: u64,
    area: u64,
}

#[derive(Clone, Debug)]
struct Plant {
    c: char,
    seen: bool,
}

fn main() {
    //let binding = include_str!("../files/test.txt");
    let binding = include_str!("../files/input.txt");
    let garden: Vec<Vec<Plant>> = binding
        .lines()
        .map(|line| line.chars().map(|c| Plant { c: c, seen: false }).collect())
        .collect();

    steps(garden.clone());
}

fn test_plant(zone: &mut Dimention, p: &mut Plant, c: char, point: (usize, usize)) -> bool {
    if p.c == c {
        if !p.seen {
            zone.points.push(point);
            zone.area += 1;
            p.seen = true;
            return true;
        }
    } else {
        zone.perim += 1;
    }
    false
}
fn create_zone_rec(zone: &mut Dimention, garden: &mut Vec<Vec<Plant>>, (x, y): (usize, usize)) {
    let c = garden[x][y].c;
    if x < garden.len() - 1 {
        if test_plant(zone, &mut garden[x+1][y], c, (x + 1, y)) {
            create_zone_rec(zone, garden, (x + 1, y));
        }
    } else {
        zone.perim += 1;
    }

    if y < garden[0].len() - 1 {
        if test_plant(zone, &mut garden[x][y+1], c, (x, y + 1)) {
            create_zone_rec(zone, garden, (x, y + 1));
        }
    } else {
        zone.perim += 1;
    }

    if x > 0 {
        if test_plant(zone, &mut garden[x-1][y], c, (x - 1, y)) {
            create_zone_rec(zone, garden, (x - 1, y));
        }
    } else {
        zone.perim += 1;
    }

    if y > 0 {
        if test_plant(zone, &mut garden[x][y-1], c, (x, y - 1)) {
            create_zone_rec(zone, garden, (x, y - 1));
        }
    } else {
        zone.perim += 1;
    }
}

fn test_space(cache: &mut Vec<Dimention>, garden: &mut Vec<Vec<Plant>>, (x, y): (usize, usize)) {
    if !garden[x][y].seen {
        let mut zone = Dimention {
            points: vec![(x,y)],
            area: 1,
            perim: 0,
        };
        garden[x][y].seen = true;
        create_zone_rec(&mut zone, garden, (x, y));
        cache.push(zone);
    }
}

fn discounted_perim(dim: Dimention) -> u64 {
    let zone:Vec<(usize,usize)> = dim.points.iter().map(|(x,y)| (x+1, y+1)).collect();
    let mut result = 0;
    for (x ,y) in &zone {
        if zone.contains(&(*x+1,*y)) && zone.contains(&(*x,*y+1)) && !zone.contains(&(*x+1,*y+1)) {
            result += 1;
        }
        if zone.contains(&(*x-1,*y)) && zone.contains(&(*x,*y-1)) && !zone.contains(&(*x-1,*y-1)) {
            result += 1;
        }
        if zone.contains(&(*x+1,*y)) && zone.contains(&(*x,*y-1)) && !zone.contains(&(*x+1,*y-1)) {
            result += 1;
        }
        if zone.contains(&(*x-1,*y)) && zone.contains(&(*x,*y+1)) && !zone.contains(&(*x-1,*y+1)) {
            result += 1;
        }
        if !zone.contains(&(*x-1,*y)) && !zone.contains(&(*x,*y-1)) {
            result += 1;
        }
        if !zone.contains(&(*x+1,*y)) && !zone.contains(&(*x,*y+1)) {
            result += 1;
        }
        if !zone.contains(&(*x-1,*y)) && !zone.contains(&(*x,*y+1)) {
            result += 1;
        }
        if !zone.contains(&(*x+1,*y)) && !zone.contains(&(*x,*y-1)) {
            result += 1;
        }
    }
    result
}

fn steps(mut garden: Vec<Vec<Plant>>) {
    let mut cache = vec![];
    let (size_x, size_y) = (garden.len(), garden[0].len());
    for i in 0..size_x {
        for j in 0..size_y {
            test_space(&mut cache, &mut garden, (i, j));
        }
    }
    //println!("cache = {cache:?}");
    let total1 = cache
        .iter()
        .fold(0, |mut acc, dim: &Dimention| {acc += dim.area * dim.perim; acc});
    println!("step1 = {total1}");

    let total2 = cache
        .iter()
        .fold(0, |mut acc, dim: &Dimention| {
            acc += dim.area * discounted_perim(dim.clone());
            acc});
    println!("step2 = {total2}");

}
