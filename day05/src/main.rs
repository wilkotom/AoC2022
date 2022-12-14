use std::io::Error;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Copy, Clone)]
#[display("move {quantity} from {origin} to {destination}")]
struct CraneMove{
    quantity: usize,
    origin: usize,
    destination: usize
}

fn main() -> Result<(), Error> {
    let in_file = std::fs::read_to_string("./day05/input.txt")?;
    let mut data = in_file.split("\n\n");
    let stacks = parse_stacks(data.next().unwrap());
    let instructions = parse_instructions(data.next().unwrap());
    println!("Part 1: {}", solution(stacks.clone(), &instructions, true));
    println!("Part 2: {}", solution(stacks, &instructions, false));
    Ok(())
}

fn parse_stacks(starting_state: &str) -> [Vec<char>; 9] {
    let mut stacks: [Vec<char>; 9] = Default::default();
    for line in starting_state.split('\n') {
        let mut pos = 1;
        while let Some(c) = line.chars().nth(pos) {
            if c.is_ascii_uppercase() {
                stacks[(pos -1) / 4].insert(0, c);
            }
            pos +=4;
        }
    }
    stacks
}

fn parse_instructions(instructions: &str) -> Vec<CraneMove> {
    instructions.split('\n').map(|x| x.parse::<CraneMove>().unwrap()).collect()
}

fn solution(mut stacks:[Vec<char>; 9], instructions: &Vec<CraneMove>, part1: bool) -> String {
    for instruction in instructions {
        if part1 {
            for _ in 0..instruction.quantity {
                let cargo_crate = stacks[instruction.origin -1].pop().unwrap();
                stacks[instruction.destination-1].push(cargo_crate)
            }
        } else {
            let mut moved_crates = stacks[instruction.origin -1]
                .split_off(stacks[instruction.origin -1].len() - instruction.quantity);
            stacks[instruction.destination-1].append(&mut moved_crates);
        }
    }
    stacks.iter().filter_map(|c| c.last()).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";


    #[test]
    fn test_p1() {
        let mut data = DATA.split("\n\n");
        let stacks = parse_stacks(data.next().unwrap());
        let instructions = parse_instructions(data.next().unwrap());
        assert_eq!(solution(stacks,&instructions, true), "CMZ");
    }

    #[test]
    fn test_p2() {
        let mut data = DATA.split("\n\n");
        let stacks = parse_stacks(data.next().unwrap());
        let instructions = parse_instructions(data.next().unwrap());
        assert_eq!(solution(stacks,&instructions, false), "MCD");
    }
}