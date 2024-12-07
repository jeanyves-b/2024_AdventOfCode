use std::fs::read_to_string;

#[derive(Clone)]
enum SpaceType {
    Free,
    Obstacle,
    Visited,
    New,
}

#[derive(Clone)]
struct Space {
    space_type: SpaceType,
}
fn to_space(character: char, position: (usize, usize)) -> (Space, Option<Guard>) {
    match character {
        '<' => {
            return (
                Space {
                    space_type: SpaceType::Visited,
                },
                Some(Guard {
                    position: position,
                    direction: '<',
                }),
            )
        }
        '^' => {
            return (
                Space {
                    space_type: SpaceType::Visited,
                },
                Some(Guard {
                    position: position,
                    direction: '^',
                }),
            )
        }
        '>' => {
            return (
                Space {
                    space_type: SpaceType::Visited,
                },
                Some(Guard {
                    position: position,
                    direction: '>',
                }),
            )
        }
        'v' => {
            return (
                Space {
                    space_type: SpaceType::Visited,
                },
                Some(Guard {
                    position: position,
                    direction: 'v',
                }),
            )
        }
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
    fn test_ahead(&mut self, room: &mut Vec<Vec<Space>>) -> bool {
        match self.direction {
            '<' => {
                if self.position.0 == 0 {
                    return false;
                } else {
                    match room[self.position.0][self.position.1 - 1].space_type {
                        SpaceType::Free | SpaceType::Visited => {
                            room[self.position.0][self.position.1 - 1].space_type =
                                SpaceType::Visited;
                            self.position = (self.position.0, self.position.1 - 1);
                        }
                        SpaceType::Obstacle | SpaceType::New => self.direction = '^',
                    }
                    return true;
                }
            }
            '^' => {
                if self.position.1 == 0 {
                    return false;
                } else {
                    match room[self.position.0 - 1][self.position.1].space_type {
                        SpaceType::Free | SpaceType::Visited => {
                            room[self.position.0 - 1][self.position.1].space_type =
                                SpaceType::Visited;
                            self.position = (self.position.0 - 1, self.position.1);
                        }
                        SpaceType::Obstacle | SpaceType::New => self.direction = '>',
                    }
                    return true;
                }
            }
            '>' => {
                if self.position.1 == room[0].len() - 1 {
                    return false;
                } else {
                    match room[self.position.0][self.position.1 + 1].space_type {
                        SpaceType::Free | SpaceType::Visited => {
                            room[self.position.0][self.position.1 + 1].space_type =
                                SpaceType::Visited;
                            self.position = (self.position.0, self.position.1 + 1);
                        }
                        SpaceType::Obstacle | SpaceType::New => self.direction = 'v',
                    }
                    return true;
                }
            }
            'v' => {
                if self.position.0 == room.len() - 1 {
                    return false;
                } else {
                    match room[self.position.0 + 1][self.position.1].space_type {
                        SpaceType::Free | SpaceType::Visited => {
                            room[self.position.0 + 1][self.position.1].space_type =
                                SpaceType::Visited;
                            self.position = (self.position.0 + 1, self.position.1);
                        }
                        SpaceType::Obstacle | SpaceType::New => self.direction = '<',
                    }
                    return true;
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
    while local_guard.test_ahead(room) {
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
    print_room(room, guard);
    println!("step1 = {}", result);
}

fn is_looping(room: &mut Vec<Vec<Space>>, guard: &Guard) -> bool {
    return false;
}

fn step2(room: &mut Vec<Vec<Space>>, guard: &Guard) {
    let result: i32 = room
        .into_iter()
        .map(|mut line| {
            line.into_iter()
                .map(|space| match space.space_type {
                    SpaceType::Visited => {
                        space.space_type = SpaceType::New;
                        if is_looping(&mut room.clone(), guard) {
                            space.space_type = SpaceType::Visited;
                            return 1;
                        } else {
                            space.space_type = SpaceType::Visited;
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
