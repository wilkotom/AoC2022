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
    println!("{}", solution(cranes.clone(), instructions.clone(), false));
    println!("{}", solution(cranes, instructions, true));
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

fn solution(mut cranes:[Vec<char>; 10], instructions: Vec<CraneMove>, part2: bool) -> String {
    for instruction in instructions {
        let mut moved_crates = Vec::new();
        for _ in 0..instruction.quantity {
            let cargo_crate = cranes[instruction.origin -1].pop().unwrap();
            moved_crates.push(cargo_crate)
        }
        if part2 {
            moved_crates.reverse();
        }
        cranes[instruction.destination-1].append(&mut moved_crates);
    }
    let mut result = String::new();
    for crane in cranes.iter() {
        if let Some(c) = crane.iter().last() {
            result.push(*c);
        }
    }
    result
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
        assert_eq!(solution(cranes,instructions, false), "CMZ");
    }

    #[test]
    fn test_p2() {
        let mut data = DATA.split("\n\n");
        let cranes = parse_cranes(data.next().unwrap());
        let instructions = parse_instructions(data.next().unwrap());
        assert_eq!(solution(cranes,instructions, true), "MCD");
    }
}