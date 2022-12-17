use std::{collections::{HashMap, HashSet}, io::Error};
use aochelpers::Coordinate;

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day17/input.txt")?;
    let part1 = rock_tetris(&data, 2022);
    println!("Part 1: {}", part1);
    let part2 = rock_tetris(&data, 1_000_000_000_000);
    println!("Part 2: {}", part2);

    Ok(())
}

fn rock_tetris(data: &str,  rounds: i128) -> i128 {

    let horz = vec![Coordinate{x:0_i128,y:0}, Coordinate{x:1,y:0}, Coordinate{x:2,y:0}, Coordinate{x:3,y:0}];
    let plus = vec![Coordinate{x:0, y:1}, Coordinate{x:1, y:1}, Coordinate{x:2, y:1}, Coordinate{x:1, y:0}, Coordinate{x:1, y:2}];
    let el = vec![Coordinate{x:0,y:0}, Coordinate{x:1,y:0}, Coordinate{x:2,y:0}, Coordinate{x:2,y:1}, Coordinate{x:2,y:2}];
    let vert = vec![Coordinate{x:0,y:0}, Coordinate{x:0,y:1}, Coordinate{x:0,y:2}, Coordinate{x:0,y:3}];
    let square = vec![Coordinate{x:0,y:0}, Coordinate{x:1,y:0},Coordinate{x:0,y:1}, Coordinate{x:1,y:1}];
    let pieces = [horz, plus, el, vert, square];
    let instructions = data.chars().collect::<Vec<_>>();

    let mut arena = HashSet::new();

    let mut starting_y = 3;
    let mut round = 0;
    let mut inst_ptr = 0;
    let mut flat_floor_cycles = HashMap::new();

    while round < rounds {
        
        let mut piece = pieces[round as usize % pieces.len()].clone();
        for square in piece.iter_mut() {
            square.y += starting_y;
            square.x += 2;

        }
        let mut jammed = false;
        while !jammed {
            let next_instruction = instructions[inst_ptr % instructions.len()];
            let shift = match next_instruction {
                '>' => {1},
                '<' =>  {-1},
                _ => unimplemented!()
            };
            for square in piece.iter_mut() {
                square.x = match next_instruction {
                    '>' => {square.x + 1},
                    '<' =>  {square.x -1},
                    _ => unimplemented!()
                };
            }
            for square in piece.iter() {
                if arena.contains(&square.clone()) || square.x < 0 || square.x > 6 {
                    for square in piece.iter_mut() {
                        square.x -= shift;
                    }                           
                    break;
                }
            }
            for square in piece.iter_mut() {
                square.y -= 1;
            }
            for square in piece.iter() {
                if arena.contains(&square.clone()) || square.y < 0  {
                    for square in piece.iter_mut() {
                        square.y += 1;
                    }
                    jammed = true;
                    break;
                }
            }
            inst_ptr +=1;
        }

        // Cycle Detection. Look for a point at which we have:
        // -  a flat floor (for the top of the pile, all X coordinates are filled)
        // - The instruction pointer is in the same place
        // Measure the interval between two of these, use this to skip 
        // most of the one trillion rounds
        
        if (0..7).all(|x| arena.contains(&Coordinate{x, y: starting_y -4})) {
            let piece_index = round % pieces.len() as i128;
            if let Some((first_match, height_to_first_match)) = flat_floor_cycles.get(&(piece_index, inst_ptr % instructions.len())) {
                let cycle_time = round - first_match;
                let height_in_cycle = starting_y - 3 - height_to_first_match;
                let cycles_after_first_match = (rounds - first_match) / cycle_time;
                let additional_rounds = (rounds -first_match) % cycle_time;
                let bookends = rock_tetris(data, first_match + additional_rounds);
                return cycles_after_first_match * height_in_cycle + bookends;
            } else {
                flat_floor_cycles.insert((piece_index, inst_ptr % instructions.len()), (round, starting_y - 3));
            }
        }

        round +=1;
        for square in piece {
            arena.insert(square);
            if starting_y < square.y +4 {
                starting_y = square.y +4;
            } 
        }
    }
    (starting_y - 3) as i128
}
