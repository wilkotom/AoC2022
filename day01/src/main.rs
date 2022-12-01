use std::{collections::BinaryHeap, io::Error};

    fn main() -> Result<(), Error> {
        let data = std::fs::read_to_string("./day01/input.txt")?;
        let mut elves = data.split("\n\n")
            .map(|e| e.split('\n')
                    .map(|x| x.parse::<i32>()
                    .unwrap_or(0)).sum())
            .collect::<BinaryHeap<_>>();
        println!("Part 1: {:?}", elves.peek().unwrap_or(&0));
        println!("Part 2: {:?}", elves.pop().unwrap_or(0) + elves.pop().unwrap_or(0) + elves.pop().unwrap_or(0));
        Ok(())
    }
