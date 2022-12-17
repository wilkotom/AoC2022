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
    let mut skylines = HashMap::new();
    let mut starting_y = 3;
    let mut round = 0;
    let mut inst_ptr = 0;

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
        for square in piece {
            arena.insert(square);
            if starting_y < square.y +4 {
                starting_y = square.y +4;
            } 
        }


        // Cycle Detection. Look for a point at which we have:
        // - a floor profile we've seen before 
        // - The instruction pointer is in the same place
        // - The same piece has just been played
        // Measure the interval between two of these, use this to skip 
        // most of the one trillion rounds
        // Skip the first 1000 rounds as we seem to see some odd loops before 
        // things stabilise
        if round > 1000 {

            let mut depths = vec![];
            for x in 0..7 {
                if arena.contains(&Coordinate{x, y: starting_y - 4}) {
                    depths.push(0);
                } else {
                    let mut y = 1;
                    while y < starting_y {
                        if !arena.contains(&Coordinate{x, y: starting_y - 4 -y}) {
                            y +=1;
                        } else {
                
                            break;
                        }
                    }
                    depths.push(y);
                }
            }

            let piece_index = round % pieces.len() as i128;
            if let Some((first_match, height_to_first_match)) = skylines.get(&(depths.clone(), piece_index, inst_ptr % instructions.len())) {
                let cycle_time = round - first_match;
                let height_in_cycle = starting_y - 3 - height_to_first_match;
                let cycles_after_first_match = (rounds - first_match) / cycle_time;
                let additional_rounds = (rounds -first_match) % cycle_time;
                let bookends = rock_tetris(data, first_match + additional_rounds);
                return cycles_after_first_match * height_in_cycle + bookends;
            } else {
                skylines.insert((depths, piece_index, inst_ptr % instructions.len()), (round,starting_y - 3));
            }
        }

        round +=1;

    }
    (starting_y - 3) as i128
}
