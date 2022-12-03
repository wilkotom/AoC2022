use std::{collections::HashSet, io::Error};

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day03/input.txt")?;
    println!("{}", part1(&data));
    println!("{}", part2(&data));
    Ok(())
}

fn part1(data:&str) -> i32 {
    let mut score = 0;
    for line in data.split('\n') {
        let first: HashSet<char> = line[0..line.len()/2].chars().collect();
        let second: HashSet<char> = line[line.len()/2..].chars().collect();
        let c = *first.intersection(&second).next().unwrap();
        score += if c.is_lowercase() {
            c as i32 - 96
        } else {
            c as i32 - 38
        };
    }
    score
}

fn part2(data:&str) -> i32 {
    let elves: Vec<HashSet<char>> = data.split('\n').map(|l| l.chars().collect()).collect();
    let mut score = 0;
    for group in elves.chunks(3) {
        let c = *group[0].intersection(&group[1]).copied().collect::<HashSet<_>>().intersection(&group[2]).next().unwrap();
        score += if c.is_lowercase() {
            c as i32 - 96
        } else {
            c as i32 - 38
        };
    }
    score
}


#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_p1() {
        assert_eq!(part1(DATA), 157);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2(DATA), 70);
    }
}