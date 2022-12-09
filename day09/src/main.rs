use std::{error::Error, collections::HashSet};
use parse_display::{Display, FromStr};
use aochelpers::Coordinate;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{direction} {distance}")]
struct Step {
    direction: Direction,
    distance: i32
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Direction {
    #[display("U")]
    Up,
    #[display("D")]
    Down,
    #[display("L")]
    Left,
    #[display("R")]
    Right
}


fn main() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("./day09/input.txt")?;
    let instructions = data.lines().map(|l| l.parse::<Step>().unwrap()).collect::<Vec<_>>();
    println!("Part 1: {}", solution(&instructions,2));
    println!("Part 2: {}", solution(&instructions,10));
    Ok(())
}

fn solution(instructions: &Vec<Step>, rope_length: usize) -> usize {
    let mut rope = vec![Coordinate{x:0, y:0}; rope_length];
    let mut tail_visited = HashSet::new();
    tail_visited.insert(rope[rope_length-1]);
    for instruction in instructions {
        for _ in 0..instruction.distance {
            let head = rope.get_mut(0).unwrap();
            match instruction.direction {
                Direction::Up    => { head.y += 1 } 
                Direction::Down  => { head.y -= 1 }
                Direction::Left  => { head.x -= 1 } 
                Direction::Right => { head.x += 1 } 
            };
            for knot in 1..rope_length {
                let prev = *rope.get(knot-1).unwrap();
                let mut next_knot = rope.get_mut(knot).unwrap();
                if prev != *next_knot && !prev.extended_neighbours().contains(next_knot) {
                    next_knot.x += (prev.x - next_knot.x as i32).signum();
                    next_knot.y += (prev.y - next_knot.y as i32).signum();
                }
            }
            tail_visited.insert(*rope.get(rope_length-1).unwrap());
        }
    }
    tail_visited.len()
}


#[cfg(test)]
mod tests {
    use super::*;
    const DATA_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

const DATA_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    #[test]
    fn test_part_1 (){
        let instructions = DATA_1.lines().map(|l| l.parse::<Step>().unwrap()).collect::<Vec<_>>();
        println!("{:?}", instructions);
        assert_eq!(solution(&instructions,2), 13)
    }



    #[test]
    fn test_part_2() {
        let instructions = DATA_2.lines().map(|l| l.parse::<Step>().unwrap()).collect::<Vec<_>>();
        println!("{:?}", instructions);
        assert_eq!(solution(&instructions,10), 36)
    }

}