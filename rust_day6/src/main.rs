use std::fs::read_to_string;

#[derive(Copy, Clone, Debug)]
enum SpaceType {
    Free,
    Obstacle,
    Visited,
    New,
}

#[derive(Copy, Clone, Debug)]
struct Space {
    space_type: SpaceType,
}
fn to_space(character: char, position: (usize, usize)) -> (Space, Option<Guard>) {
    match character {
        '<' | '^' | '>' | 'v' => (
            Space {
                space_type: SpaceType::Visited,
            },
            Some(Guard {
                position: position,
                direction: character,
            }),
        ),
        '.' => {
            return (
                Space {
                    space_type: SpaceType::Free,
                },
                None,
            )
        }
        '#' => {
            return (
                Space {
                    space_type: SpaceType::Obstacle,
                },
                None,
            )
        }
        _ => panic!(),
    }
}

struct Guard {
    position: (usize, usize),
    direction: char,
}
impl Guard {
    fn move_guard(&mut self, room: &mut Vec<Vec<Space>>) -> bool {
        match self.direction {
            '<' => {
                if self.position.1 == 0 {
                    false
                } else {
                    match room[self.position.0][self.position.1 - 1].space_type {
                        SpaceType::Free | SpaceType::Visited => {
                            room[self.position.0][self.position.1 - 1].space_type =
                                SpaceType::Visited;
                            self.position = (self.position.0, self.position.1 - 1);
                        }
                        SpaceType::Obstacle | SpaceType::New => self.direction = '^',
                    }
                    true
                }
            }
            '^' => {
                if self.position.0 == 0 {
                    false
                } else {
                    match room[self.position.0 - 1][self.position.1].space_type {
                        SpaceType::Free | SpaceType::Visited => {
                            room[self.position.0 - 1][self.position.1].space_type =
                                SpaceType::Visited;
                            self.position = (self.position.0 - 1, self.position.1);
                        }
                        SpaceType::Obstacle | SpaceType::New => self.direction = '>',
                    }
                    true
                }
            }
            '>' => {
                if self.position.1 == room[0].len() - 1 {
                    false
                } else {
                    match room[self.position.0][self.position.1 + 1].space_type {
                        SpaceType::Free | SpaceType::Visited => {
                            room[self.position.0][self.position.1 + 1].space_type =
                                SpaceType::Visited;
                            self.position = (self.position.0, self.position.1 + 1);
                        }
                        SpaceType::Obstacle | SpaceType::New => self.direction = 'v',
                    }
                    true
                }
            }
            'v' => {
                if self.position.0 == room.len() - 1 {
                    false
                } else {
                    match room[self.position.0 + 1][self.position.1].space_type {
                        SpaceType::Free | SpaceType::Visited => {
                            room[self.position.0 + 1][self.position.1].space_type =
                                SpaceType::Visited;
                            self.position = (self.position.0 + 1, self.position.1);
                        }
                        SpaceType::Obstacle | SpaceType::New => self.direction = '<',
                    }
                    true
                }
            }
            _ => panic!(),
        }
    }
}

fn get_character(room: &Vec<Vec<Space>>, guard: &Guard, position: (usize, usize)) -> char {
    if position.0 == guard.position.0 && position.1 == guard.position.1 {
        return guard.direction;
    }
    match room[position.0][position.1].space_type {
        SpaceType::Free => ' ',
        SpaceType::Obstacle => '0',
        SpaceType::Visited => '*',
        SpaceType::New => '+',
    }
}
fn print_room(room: &Vec<Vec<Space>>, guard: &Guard) {
    for y in 0..room.len() {
        for x in 0..room[0].len() {
            print!("{}", get_character(room, &guard, (y, x)));
        }
        println!();
    }
    println!("-------------------------------",);
}

fn step1(room: &mut Vec<Vec<Space>>, guard: &Guard) {
    let mut local_guard: Guard = Guard {
        position: guard.position,
        direction: guard.direction,
    };
    while local_guard.move_guard(room) {
        //print_room(room, guard);
    }
    let result: i32 = room
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|space| match space.space_type {
                    SpaceType::Visited => 1,
                    _ => 0,
                })
                .collect::<Vec<i32>>()
                .iter()
                .sum::<i32>()
        })
        .sum();
    print_room(room, &local_guard);
    println!("step1 = {}", result);
}

fn is_looping(room: &mut Vec<Vec<Space>>, guard: &Guard) -> bool {
    let mut local_guard: Guard = Guard {
        position: guard.position,
        direction: guard.direction,
    };
    let mut path = vec![guard.position];
    let mut previous_pos = guard.position;

    while local_guard.move_guard(room) {
        for i in 1..path.len() {
            if path[i] == local_guard.position
                && path[i - 1] == previous_pos
                && path[i] != path[i - 1]
            {
                return true;
            }
        }
        path.push(local_guard.position);
        previous_pos = local_guard.position;
    }
    return false;
}

fn step2(room: &Vec<Vec<Space>>, guard: &Guard) {
    let mut room_working = room.to_owned().clone();
    let result: i32 = room
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.into_iter()
                .enumerate()
                .map(|(j, space)| match space.space_type {
                    SpaceType::Visited => {
                        room_working[i][j].space_type = SpaceType::New;
                        if is_looping(&mut room_working, guard) {
                            room_working[i][j].space_type = SpaceType::Visited;
                            return 1;
                        } else {
                            room_working[i][j].space_type = SpaceType::Visited;
                            return 0;
                        }
                    }
                    _ => 0,
                })
                .collect::<Vec<i32>>()
                .iter()
                .sum::<i32>()
        })
        .sum();
    println!("step2 = {}", result);
}

fn main() {
    //let binding = read_to_string("files/test.txt").unwrap();
    let binding = read_to_string("files/input.txt").unwrap();

    let input: Vec<&str> = binding.lines().collect();
    let mut guard: Guard = Guard {
        position: (0, 0),
        direction: '_',
    };
    let mut room: Vec<Vec<Space>> = input
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, character)| {
                    let (space, _guard) = to_space(character, (i, j));
                    if _guard.is_some() {
                        guard = _guard.unwrap();
                    }
                    space
                })
                .collect()
        })
        .collect();

    step1(&mut room, &guard);
    step2(&mut room, &guard);
}
