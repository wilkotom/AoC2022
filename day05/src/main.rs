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
    let cranes = parse_cranes(data.next().unwrap());
    let instructions = parse_instructions(data.next().unwrap());
    println!("Part 1: {}", solution(cranes.clone(), instructions.clone(), false));
    println!("Part 2: {}", solution(cranes, instructions, true));
    Ok(())
}

fn parse_cranes(starting_state: &str) -> [Vec<char>; 10] {
    let mut cranes: [Vec<char>; 10] = Default::default();
    for line in starting_state.split('\n') {
        let mut pos = 1;
        while let Some(c) = line.chars().nth(pos) {
            let crane = (pos -1) / 4 ;
            pos +=4;
            if c.is_ascii_uppercase() {
                cranes[crane].insert(0, c)
            }
        }
    }
    cranes
}

fn parse_instructions(instructions: &str) -> Vec<CraneMove> {
    instructions.split('\n').map(|x| x.parse::<CraneMove>().unwrap()).collect()
}

fn solution(mut cranes:[Vec<char>; 10], instructions: Vec<CraneMove>, part1: bool) -> String {
    for instruction in instructions {
        if part1 {
            for _ in 0..instruction.quantity {
                let cargo_crate = cranes[instruction.origin -1].pop().unwrap();
                cranes[instruction.destination-1].push(cargo_crate)
            }
        } else {
            let mut moved_crates = cranes[instruction.origin -1].split_off(cranes[instruction.origin -1].len() - instruction.quantity);
            cranes[instruction.destination-1].append(&mut moved_crates);
        }
    }
    
    cranes.iter().filter_map(|c| c.last()).collect::<String>()
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
        let cranes = parse_cranes(data.next().unwrap());
        let instructions = parse_instructions(data.next().unwrap());
        assert_eq!(solution(cranes,instructions, true), "CMZ");
    }

    #[test]
    fn test_p2() {
        let mut data = DATA.split("\n\n");
        let cranes = parse_cranes(data.next().unwrap());
        let instructions = parse_instructions(data.next().unwrap());
        assert_eq!(solution(cranes,instructions, false), "MCD");
    }
}