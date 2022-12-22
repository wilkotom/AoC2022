use std::{collections::{HashSet, VecDeque, HashMap}, io::Error};
use aochelpers::Coordinate;

#[derive(Debug, PartialEq, Eq)]
enum Facing {
    North,
    South,
    East,
    West
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Turn(Direction),
    Walk(i32)
}

#[derive(Debug, PartialEq, Eq)]
enum MapSquare {
    Blocked,
    Open
}

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day22/input.txt")?;
    let (jungle_map, path) = parse_data(&data);
    println!("Part 1: {}", solution_1(&jungle_map, &path));
    println!("Part 2: {}", solution_2(&jungle_map, &path));
    Ok(())
}

fn solution_2(jungle_map: &HashMap<Coordinate<i32>,MapSquare>, instructions: &Vec<Instruction>) -> i32 {
    let mut facing = Facing::East;
    let mut location = Coordinate{x: min_x_for_row(0, jungle_map), y:0};
    for instr in instructions {
        match instr {
            Instruction::Turn(Direction::Left) => {
                facing = match facing {
                    Facing::North => Facing::West,
                    Facing::East => Facing::North,
                    Facing::South => Facing::East,
                    Facing::West => Facing::South
                };
            },
            Instruction::Turn(Direction::Right) => {
                facing = match facing {
                    Facing::North => Facing::East,
                    Facing::East => Facing::South,
                    Facing::South => Facing::West,
                    Facing::West => Facing::North
                }
            }
            Instruction::Walk(mut distance) => {
                while distance > 0 {
                    let next_square_delta = match facing {
                        Facing::North => Coordinate{x: 0, y: -1},
                        Facing::South => Coordinate{x: 0, y: 1},
                        Facing::East => Coordinate { x: 1, y: 0 },
                        Facing::West => Coordinate { x: -1, y: 0 },
                    };            
                    let next_square = location + next_square_delta;
                    if let Some(possible) = jungle_map.get(&next_square) {
                        match possible {
                            MapSquare::Blocked => {
                                break;
                            },
                            MapSquare::Open => {
                                location = next_square;
                                distance -=1;
                            },
                        }
                    } else {
                        /*
                        Work out what face we're on, move appropriately. Won't work for test data as the shape and dimensions
                        are assumed to match my specific input. Faces are laid out in my data:
                            AB
                            C
                           ED
                           F
                        */
                        if next_square.y == -1 && next_square.x > 49 && next_square.x < 100 && facing == Facing::North{
                            // "A" to "F"
                            let next_square = Coordinate{x: 0, y: next_square.x +100};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::East;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else if next_square.x == 49 && next_square.y <50 && facing == Facing::West{
                            // "A" to "E"
                            let next_square = Coordinate{x: 0, y: 149 - next_square.y};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::East;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else if next_square.x > 99 && next_square.y == 50 && facing == Facing::South{
                            // "B" to "C"
                            let next_square = Coordinate{x: 99, y: next_square.x -50};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::West;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else if next_square.x > 99 && next_square.y == -1 && facing == Facing::North{
                            // "B" to "F"
                            let next_square = Coordinate{x: next_square.x -100, y: 199};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::North;
                                location = next_square
                            } else {
                                break;
                            }
                        }  else if next_square.x >149 && facing == Facing::East{
                            // "B" to "D"
                            let next_square = Coordinate{x: 99, y: 149 - next_square.y};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::West;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else if next_square.x == 49 && next_square.y > 49 && next_square.y < 100 && facing == Facing::West {
                            // "C" to "E"
                            let next_square = Coordinate{x: next_square.y -50, y: 100};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::South;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else if next_square.x == 100 && next_square.y > 49 && next_square.y < 100 && facing == Facing::East {
                            // "C" to "B"
                            let next_square = Coordinate{x: next_square.y +50, y: 49};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::North;
                                location = next_square;
                            } else {
                                break;
                            }
                        }  else if next_square.x == 100 && next_square.y > 99 && next_square.y < 150 && facing == Facing::East {
                            // "D" to "B"
                            let next_square = Coordinate{x: 149, y: 149 - next_square.y};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::West;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else if next_square.y == 150 && next_square.x > 49 && next_square.x < 100 && facing == Facing::South {
                            // "D" to "F"
                            let next_square = Coordinate{x: 49, y: next_square.x +100};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::West;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else if next_square.x <50  && next_square.y < 100 && facing == Facing::North{
                            // "E" to "C"
                            let next_square = Coordinate{x: 50, y: next_square.x +50};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::East;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else if next_square.x < 0  && next_square.y > 99 && next_square.y < 150 && facing == Facing::West {
                            // "E" to "A"
                            let next_square = Coordinate{x: 50, y: 149 - next_square.y};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::East;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else if next_square.x < 0  && next_square.y > 149 && facing == Facing::West {
                            // "F" to "A"
                            let next_square = Coordinate{x: next_square.y -100, y: 0};

                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::South;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else if next_square.x == 50  && next_square.y > 149 && facing == Facing::East {
                            // "F" to "D"
                            let next_square = Coordinate{x: next_square.y - 100,  y: 149};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::North;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else if next_square.y > 199 && facing == Facing::South{
                            // "F" to "B"
                            let next_square = Coordinate{x: next_square.x + 100, y: 0};
                            if jungle_map.get(&next_square).unwrap() == &MapSquare::Open {
                                distance -=1;
                                facing = Facing::South;
                                location = next_square;
                            } else {
                                break;
                            }
                        } else {
                            panic!("Asked to make illegal move to {next_square} while facing {:?}", facing);
                        }

                    }
                }
            },
        }
    }

    let final_location = location + Coordinate{x:1, y:1};
    final_location.y * 1000 + final_location.x * 4 + match facing {
        Facing::North => 3,
        Facing::South => 1,
        Facing::East => 0,
        Facing::West => 2,
    }
}

fn solution_1(jungle_map: &HashMap<Coordinate<i32>,MapSquare>, instructions: &Vec<Instruction>) -> i32 {
    let mut facing = Facing::East;
    let mut location = Coordinate{x: min_x_for_row(0, jungle_map), y:0};
    for instr in instructions {
        let next_square_delta = match facing {
            Facing::North => Coordinate{x: 0, y: -1},
            Facing::South => Coordinate{x: 0, y: 1},
            Facing::East => Coordinate { x: 1, y: 0 },
            Facing::West => Coordinate { x: -1, y: 0 },
        };
        match instr {
            Instruction::Turn(Direction::Left) => {
                facing = match facing {
                    Facing::North => Facing::West,
                    Facing::East => Facing::North,
                    Facing::South => Facing::East,
                    Facing::West => Facing::South
                };
            },
            Instruction::Turn(Direction::Right) => {
                facing = match facing {
                    Facing::North => Facing::East,
                    Facing::East => Facing::South,
                    Facing::South => Facing::West,
                    Facing::West => Facing::North
                }
            }
            Instruction::Walk(mut distance) => {
                while distance > 0 {
                    let next_square = location + next_square_delta;
                    if let Some(possible) = jungle_map.get(&next_square) {
                        match possible {
                            MapSquare::Blocked => {
                                break;
                            },
                            MapSquare::Open => {
                                location = next_square;
                                distance -=1;
                            },
                        }
                    } else {
                        let next_square = match facing {
                            Facing::North => Coordinate{x: location.x, y: max_y_for_col(location.x, jungle_map)},
                            Facing::South => Coordinate{x: location.x, y: min_y_for_col(location.x, jungle_map)},
                            Facing::East => Coordinate { x: min_x_for_row(location.y, jungle_map), y: location.y },
                            Facing::West => Coordinate { x: max_x_for_row(location.y, jungle_map), y: location.y },
                        };
                        if let Some(possible) = jungle_map.get(&next_square) {
                            match possible {
                                MapSquare::Blocked => break,
                                MapSquare::Open => {
                                    location = next_square;
                                    distance -=1;
                                },
                            }
                        }
                        else {
                            unreachable!{"Jungle square {} doesn't appear on the map!", next_square};
                        }
                    }
                }
            },
        }
    }
    let final_location = location + Coordinate{x:1, y:1};

    final_location.y * 1000 + final_location.x * 4 + match facing {
        Facing::North => 3,
        Facing::South => 1,
        Facing::East => 0,
        Facing::West => 2,
    }
}

fn min_x_for_row(y: i32, jungle_map: &HashMap<Coordinate<i32>,MapSquare>) -> i32 {
    jungle_map.keys().filter(|c| y == c.y).map(|c| c.x).min().unwrap()
}

fn max_x_for_row(y: i32, jungle_map: &HashMap<Coordinate<i32>,MapSquare>) -> i32 {
    jungle_map.keys().filter(|c| y == c.y).map(|c| c.x).max().unwrap()
}

fn min_y_for_col(x:i32, jungle_map: &HashMap<Coordinate<i32>,MapSquare>) -> i32 {
    jungle_map.keys().filter(|c| x == c.x).map(|c| c.y).min().unwrap()
}

fn max_y_for_col(x: i32, jungle_map: &HashMap<Coordinate<i32>,MapSquare>) -> i32 {
    jungle_map.keys().filter(|c| x == c.x).map(|c| c.y).max().unwrap()
}

fn parse_data(data: &str) -> (HashMap<Coordinate<i32>,MapSquare>, Vec<Instruction>){
    let mut sections = data.split("\n\n");

    let mut board = HashMap::new();

    let board_str = sections.next().unwrap();
    for (y, line) in board_str.lines().enumerate() {
        for(x,c) in line.chars().enumerate() {
            match c {
                '.' => {board.insert(Coordinate{x: x as i32,y: y as i32}, MapSquare::Open);}
                '#' => {board.insert(Coordinate{x: x as i32,y: y as i32}, MapSquare::Blocked);}
                ' ' => {},
                _ => unimplemented!()
            };
        }
    }

    let mut instructions = Vec::new();
    let raw_instructions = sections.next().unwrap();
    let mut distance = 0;
    for c in raw_instructions.chars() {
        if let Some(d) = c.to_digit(10) {
            distance *= 10;
            distance += d as i32;
        } else {
            instructions.push(Instruction::Walk(distance));
            distance = 0;
            instructions.push(match c {
                'L' => Instruction::Turn(Direction::Left),
                'R' => Instruction::Turn(Direction::Right),
                _ => unimplemented!()
            });
        }
    }
    instructions.push(Instruction::Walk(distance));
    (board, instructions)
}


#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_day1() {
        let (map, path) = parse_data(DATA);
        println!("{}", solution_1(&map, &path));
    }
}