use std::{collections::BinaryHeap, io::Error};

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day01/input.txt")?;
    let (part1, part2) = solution(&data);
    println!("Part 1: {:?}\nPart 2: {:?}", part1, part2);
    Ok(())
}

fn solution(data: &str) -> (i32, i32) {
    let mut elves = data.split("\n\n")
        .map(|e| e.split('\n')
                .map(|x| x.parse::<i32>()
                .unwrap_or(0)).sum())
        .collect::<BinaryHeap<_>>();
    (*elves.peek().unwrap_or(&0), 
        elves.pop().unwrap_or(0) + elves.pop().unwrap_or(0) + elves.pop().unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_day1() {
        let (part1_res, part2_res) = solution(DATA);
        assert_eq!(part1_res, 24000);
        assert_eq!(part2_res, 45000);
    }
}