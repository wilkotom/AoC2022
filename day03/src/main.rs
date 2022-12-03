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
    let mut identified: HashSet<usize> = HashSet::new();
    let mut score = 0;
    'first: for (i, first_elf) in elves.iter().enumerate() {
        if identified.contains(&i) {
            continue;
        }
        for (j, mid_elf) in elves.iter().enumerate().skip(i+1) {
            if identified.contains(&j) {
                continue;
            }
            let first_two = first_elf.intersection(mid_elf).copied().collect::<HashSet<_>>();
            if first_two.is_empty() {
                continue;
            }
            for (k, last_elf) in elves.iter().enumerate().skip(i+2) {
                if identified.contains(&k) {
                    continue;
                }
                let all_three = first_two.intersection(last_elf).collect::<HashSet<_>>();
                if all_three.len() == 1{
                    if let Some(c) = all_three.iter().next() {
                        score += if c.is_lowercase() {
                            **c as i32 - 96
                        } else {
                            **c as i32 - 38
                        };
                        identified.extend([i, j, k].iter());
                        continue 'first;
                    }
                }
            }
        }
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