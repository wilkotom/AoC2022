use std::{collections::{HashSet, HashMap}, io::Error};
use aochelpers::Coordinate;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North, South, East, West
}

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day23/input.txt")?;
    println!("Part 1: {}", solution(parse_data(&data), 10));
    solution(parse_data(&data), usize::MAX);
    Ok(())
}

fn solution(mut state: HashSet<Coordinate<i32>>, rounds: usize) -> i32 {
    
    let nw = Coordinate{x: -1, y: -1};
    let n = Coordinate{x: 0, y: -1};
    let ne = Coordinate{x:1, y:-1};
    let e = Coordinate{x: 1, y:0};
    let se = Coordinate{x:1, y: 1};
    let s = Coordinate{x:0, y:1};
    let sw = Coordinate{x: -1, y: 1};
    let w = Coordinate{x: -1, y: 0};

    let mut directions= vec![Direction::North, Direction::South, Direction::West, Direction::East];

    for round in 0..rounds {
        let mut next_spaces: HashMap<Coordinate<i32>, Vec<Coordinate<i32>>> = HashMap::new();
        let mut next_state = HashSet::new();
        'outer: for elf in state.iter() {
            if elf.extended_neighbours().iter().all(|n| !state.contains(n)) {
                next_state.insert(*elf);
            } else {
                for direction in directions.iter() {
                    match direction {
                        Direction::North => { 
                            if !state.contains(&(*elf + nw)) && !state.contains(&(*elf + n)) && !state.contains(&(*elf + ne)) {
                                next_spaces.entry(*elf + n).or_default();
                                next_spaces.get_mut(&(*elf +n)).unwrap().push(*elf);
                                continue 'outer;
                            }
                        }
                        Direction::South => {
                            if !state.contains(&(*elf + sw)) && !state.contains(&(*elf + s)) && !state.contains(&(*elf + se)) {
                                next_spaces.entry(*elf + s).or_default();
                                next_spaces.get_mut(&(*elf +s)).unwrap().push(*elf);
                                continue 'outer;
                            }
                        }
                        Direction::West => { 
                            if !state.contains(&(*elf + sw)) && !state.contains(&(*elf + w)) && !state.contains(&(*elf + nw)) {
                                next_spaces.entry(*elf + w).or_default();
                                next_spaces.get_mut(&(*elf +w)).unwrap().push(*elf);
                                continue 'outer;
                            }
                        }
                        Direction::East => {
                            if !state.contains(&(*elf + ne)) && !state.contains(&(*elf + e)) && !state.contains(&(*elf + se)) {
                                next_spaces.entry(*elf + e).or_default();
                                next_spaces.get_mut(&(*elf + e)).unwrap().push(*elf);
                                continue 'outer;
                            }
                        }
                    }
                }
            }
            next_state.insert(*elf);
        }
        if next_state.len() == state.len() {
            println!("No Elves moved at round {}", round+1);
            break;
        }

        for (new, old) in next_spaces.iter() {
            if old.len() == 1 {
                next_state.insert(*new);
            } else {
                for elf in old {
                    next_state.insert(*elf);
                }
            }
        }
        let last_dir = directions.remove(0);
        directions.push(last_dir);
        state = next_state;
    }
    let min_x = state.iter().map(|c| c.x).min().unwrap();
    let max_x = state.iter().map(|c| c.x).max().unwrap();
    let min_y = state.iter().map(|c| c.y).min().unwrap();
    let max_y = state.iter().map(|c| c.y).max().unwrap();

    (max_x - min_x +1) * (max_y - min_y +1) - state.len() as i32
}

fn parse_data(data: &str) -> HashSet<Coordinate<i32>> {
    let mut starting_state = HashSet::new();
    for (y, line) in data.lines().enumerate() {
        for (x,c)in line.chars().enumerate() {
            if c == '#' {
                starting_state.insert(Coordinate{x: x as i32, y: y as i32});
            }
        }
    }
    starting_state
}


#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_day1() {
        let data = parse_data(DATA);
        println!("{}", solution(data, usize::MAX));
    }
}